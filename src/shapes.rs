use std::f64::consts::TAU;

use geo::coord;
use geo::Coord;
use geo::EuclideanDistance;
use rand::Rng;

#[derive(Clone, PartialEq)]
pub struct LineString {
    pub points: Vec<Coord>,
    stroke: String,
    stroke_width: String,
}

impl LineString {
    pub fn new(points: Vec<Coord>) -> Self {
        Self {
            points,
            stroke: "black".to_string(),
            stroke_width: "".to_string(),
        }
    }

    pub fn from_tuples(points: Vec<(f64, f64)>) -> Self {
        Self::new(
            points
                .iter()
                .map(|p| coord! { x: p.0, y: p.1})
                .collect::<Vec<Coord>>(),
        )
    }
}

#[derive(Clone, PartialEq)]
pub struct Rect {
    pub xy: Coord,
    pub width: f64,
    pub height: f64,
    stroke: String,
    stroke_width: String,
}

impl Rect {
    pub fn new(xy: Coord, width: f64, height: f64) -> Self {
        Self {
            xy,
            width,
            height,
            stroke: "black".to_string(),
            stroke_width: "".to_string(),
        }
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
    stroke: String,
    stroke_width: String,
}

impl Circle {
    pub fn new(center: Coord, radius: f64) -> Self {
        Self {
            center,
            radius,
            stroke: "black".to_string(),
            stroke_width: "".to_string(),
        }
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

pub trait Scale<T> {
    fn scale(&self, perc: f64) -> T;
}

pub trait Sample {
    fn sample_uniform(&self, n: u64) -> Vec<Coord>;
}

pub trait Centroid {
    fn centroid(&self) -> Coord;
}
