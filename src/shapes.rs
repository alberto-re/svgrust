use std::f64::consts::TAU;

use geo::algorithm::bool_ops::BooleanOps;
use geo::algorithm::Rotate as GeoRotate;
use geo::coord;
use geo::Coord;
use geo::CoordsIter;
use geo::EuclideanDistance;
use geo::MultiLineString;
use geo::MultiPolygon;
use geo::Polygon;
use rand::Rng;

#[derive(Clone, PartialEq)]
pub struct LineStr {
    pub points: Vec<Coord>,
}

impl LineStr {
    pub fn new(points: Vec<Coord>) -> Self {
        Self { points }
    }

    pub fn from_tuples(points: Vec<(f64, f64)>) -> Self {
        Self::new(
            points
                .iter()
                .map(|p| coord! { x: p.0, y: p.1})
                .collect::<Vec<Coord>>(),
        )
    }

    pub fn add_vec(&mut self, vec: Coord) -> Self {
        self.points.iter_mut().for_each(|p| {
            p.x += vec.x;
            p.y += vec.y;
        });
        self.clone()
    }

    pub fn clip(&self, other: &LineStr, invert: bool) -> Vec<LineStr> {
        let ls = geo::LineString(self.points.clone());
        let mls = MultiLineString::new(vec![ls]);
        let poly_lstr = geo::LineString::new(other.points.clone());
        let poly = Polygon::new(poly_lstr, vec![]);
        let mpoly = MultiPolygon::new(vec![poly]);
        let clipped = mpoly.clip(&mls, invert);
        let mut res = vec![];
        clipped.0.iter().for_each(|l| {
            let mut points: Vec<Coord> = vec![];
            l.clone().into_points().iter().for_each(|p| {
                points.push(p.0);
            });
            res.push(LineStr::new(points));
        });
        res
    }
}

impl Centroid for LineStr {
    fn centroid(&self) -> Coord {
        // TODO: we must prevent division by zero
        let mut xsum: f64 = 0.;
        let mut ysum: f64 = 0.;
        self.points.iter().for_each(|p| {
            xsum += p.x;
            ysum += p.y;
        });
        coord! {
            x: xsum / self.points.len() as f64,
            y: ysum / self.points.len() as f64,
        }
    }
}

impl Upsample for LineStr {
    fn upsample(&self, factor: u64) -> Self {
        // TODO: add wrap bool argument like Chaikin
        let mut points = self.points.clone();
        (0..factor).for_each(|_| {
            let mut upsampled = vec![];
            for i in 1..points.len() {
                upsampled.push(points[i - 1]);
                let middle_point = coord! {
                    x: (points[i-1].x + points[i].x) * 0.5,
                    y: (points[i-1].y + points[i].y) * 0.5,
                };
                upsampled.push(middle_point);
            }
            upsampled.push(coord! {
                x: (points[0].x + points[points.len()-1].x) * 0.5,
                y: (points[0].y + points[points.len()-1].y) * 0.5,
            });
            points = upsampled.clone();
        });
        LineStr::new(points)
    }
}

impl Chaikin for LineStr {
    fn chaikin(&self, iterations: u64, closed: bool) -> Self {
        let mut points = self.points.clone();
        (0..iterations).for_each(|_| {
            let mut smoothed = vec![];
            for i in 1..points.len() {
                smoothed.push(coord! {
                    x: points[i - 1].x * 0.75 + points[i].x * 0.25,
                    y: points[i - 1].y * 0.75 + points[i].y * 0.25,
                });
                smoothed.push(coord! {
                    x: points[i - 1].x * 0.25 + points[i].x * 0.75,
                    y: points[i - 1].y * 0.25 + points[i].y * 0.75,
                });
            }
            if closed {
                smoothed.push(coord! {
                    x: points[points.len() - 1].x * 0.75 + points[1].x * 0.25,
                    y: points[points.len() - 1].y * 0.75 + points[1].y * 0.25,
                });
                smoothed.push(coord! {
                    x: points[points.len() -1].x * 0.25 + points[1].x * 0.75,
                    y: points[points.len() -1].y * 0.25 + points[1].y * 0.75,
                });
            }
            points = smoothed.clone();
        });
        LineStr::new(points)
    }
}

impl Rotate for LineStr {
    // TODO: this should be a polygon method.
    // For simplicity here we assume the linestring is closed
    // and represents a polygon.
    // TODO: add direction (clockwise, anti-clockwise) of rotation
    // TODO: implement from scratch?
    fn rotate(&self, radians: f64) -> Self {
        let poly: Polygon = geo::Polygon::new(geo::LineString::new(self.points.clone()), vec![]);
        let degrees = radians * 180.0 / TAU;
        let poly = poly.rotate_around_centroid(degrees);
        LineStr::new(
            poly.exterior()
                .points()
                .map(|p| p.coords_iter().nth(0).unwrap())
                .collect::<Vec<Coord>>(),
        )
    }
}

#[derive(Clone, PartialEq)]
pub struct Rect {
    pub xy: Coord,
    pub width: f64,
    pub height: f64,
}

impl Rect {
    pub fn new(xy: Coord, width: f64, height: f64) -> Self {
        Self { xy, width, height }
    }

    pub fn with_center(xy: Coord, h: f64, w: f64) -> Rect {
        let x = xy.x - w / 2.;
        let y = xy.y - h / 2.;
        Self::new(coord! { x: x, y: y }, h, w)
    }

    pub fn square_with_center(xy: Coord, l: f64) -> Rect {
        Self::with_center(xy, l, l)
    }

    pub fn min_len(&self) -> f64 {
        f64::min(self.width, self.height)
    }

    pub fn grid(&self, rows: u64, cols: u64) -> Vec<Rect> {
        let w = self.width / cols as f64;
        let h = self.height / rows as f64;
        let mut cells = vec![];
        (0..rows).for_each(|r| {
            (0..cols).for_each(|c| {
                cells.push(Rect::new(coord! {x: c as f64 * w, y: r as f64 * h}, w, h));
            });
        });
        cells
    }

    pub fn to_linestr(&self, close: bool) -> LineStr {
        let mut points = vec![
            self.xy,
            coord! {x: self.xy.x + self.width, y: self.xy.y},
            coord! {x: self.xy.x + self.width, y: self.xy.y + self.height},
            coord! {x: self.xy.x, y: self.xy.y + self.height},
        ];
        if close {
            points.push(self.xy);
        }
        LineStr { points }
    }
}

impl Scale<Rect> for Rect {
    fn scale(&self, perc: f64) -> Rect {
        Rect::new(
            coord! { x: self.xy.x + self.width * ((1. - perc) / 2.), y: self.xy.y + self.height * ((1. - perc) / 2.) },
            self.width * perc,
            self.height * perc,
        )
    }
}

impl Contains for Rect {
    fn contains<T: Centroid>(&self, shape: &T) -> bool {
        (shape.centroid().x > self.xy.x && shape.centroid().x < self.xy.x + self.width)
            && (shape.centroid().y > self.xy.y && shape.centroid().y < self.xy.y + self.height)
    }
}

impl Sample for Rect {
    fn sample_uniform(&self, n: u64) -> Vec<Coord> {
        let mut rng = rand::thread_rng();
        let mut samples = vec![];
        (0..n).for_each(|_| {
            let x = rng.gen::<f64>() * self.width + self.xy.x;
            let y = rng.gen::<f64>() * self.height + self.xy.y;
            samples.push(coord! { x: x, y: y});
        });
        samples
    }
}

impl Centroid for Rect {
    fn centroid(&self) -> Coord {
        coord! {
            x: self.xy.x + self.width * 0.5,
            y: self.xy.y + self.height * 0.5,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Circle {
    pub center: Coord,
    pub radius: f64,
}

impl Circle {
    pub fn new(center: Coord, radius: f64) -> Self {
        Self { center, radius }
    }
    pub fn overlaps(&self, other: &Circle) -> bool {
        self.center.euclidean_distance(&other.center) <= self.radius + other.radius
    }

    pub fn dist(&self, other: &Circle) -> f64 {
        f64::max(
            0.,
            self.center.euclidean_distance(&other.center) - self.radius - other.radius,
        )
    }

    pub fn to_linestr(&self, points: usize) -> LineStr {
        let mut pvec = vec![];
        for i in 0..points {
            let angle = TAU / points as f64 * i as f64;
            let x = angle.cos() * self.radius + self.centroid().x;
            let y = angle.sin() * self.radius + self.centroid().y;
            pvec.push(coord! {x: x, y: y});
        }
        pvec.push(*pvec.first().unwrap());

        LineStr { points: pvec }
    }
}

impl Sample for Circle {
    fn sample_uniform(&self, n: u64) -> Vec<Coord> {
        let mut rng = rand::thread_rng();
        let mut samples = vec![];
        (0..n).for_each(|_| {
            let r_sqrt = (rng.gen::<f64>() * self.radius * self.radius).sqrt();
            let angle = rng.gen::<f64>() * TAU;
            let x = r_sqrt * angle.cos() + self.center.x;
            let y = r_sqrt * angle.sin() + self.center.y;
            samples.push(coord! { x: x, y: y});
        });
        samples
    }
}

impl Centroid for Circle {
    fn centroid(&self) -> Coord {
        self.center
    }
}

impl Contains for Circle {
    fn contains<T: Centroid>(&self, shape: &T) -> bool {
        self.center.euclidean_distance(&shape.centroid()) < self.radius
    }
}

impl Scale<Circle> for Circle {
    fn scale(&self, perc: f64) -> Circle {
        Circle::new(self.center, self.radius * perc)
    }
}

#[derive(Clone, PartialEq)]
pub struct Arc {
    pub center: Coord,
    pub radius: f64,
    pub start: f64,
    pub end: f64,
}

impl Arc {
    pub fn new(center: Coord, radius: f64, start: f64, end: f64) -> Self {
        if start == end {
            panic!("You should use Circle for closed arcs");
        }
        Self {
            center,
            radius,
            start,
            end,
        }
    }

    pub fn to_linestr(&self, points: usize) -> LineStr {
        let mut pvec = vec![];
        let arc_size: f64 = self.end - self.start;
        let step = arc_size / points as f64;
        for i in 0..points {
            let x = (self.start + step * i as f64).cos() * self.radius + self.center.x;
            let y = (self.start + step * i as f64).sin() * self.radius + self.center.y;
            pvec.push(coord! {x: x, y: y});
        }
        LineStr { points: pvec }
    }
}

pub trait Scale<T> {
    fn scale(&self, perc: f64) -> T;
}

pub trait Sample {
    fn sample_uniform(&self, n: u64) -> Vec<Coord>;
}

pub trait Upsample {
    fn upsample(&self, factor: u64) -> Self;
}

pub trait Chaikin {
    fn chaikin(&self, iterations: u64, wrap: bool) -> Self;
}

pub trait Rotate {
    fn rotate(&self, radians: f64) -> Self;
}

pub trait Centroid {
    fn centroid(&self) -> Coord;
}

pub trait Contains {
    fn contains<T: Centroid>(&self, coord: &T) -> bool;
}
