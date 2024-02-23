use std::f64::consts::TAU;

use crate::grid::SquareGrid;
use geo::algorithm::bool_ops::BooleanOps;
use geo::coord;
use geo::Coord;
use geo::EuclideanDistance;
use geo::MultiLineString;
use geo::MultiPolygon;
use geo::Polygon;

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

    pub fn into_square_grid(&self, square_side: f64) -> SquareGrid {
        SquareGrid::new(self.xy.x, self.xy.y, self.width, self.height, square_side)
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
            let x = angle.cos() * self.radius + self.center.x;
            let y = angle.sin() * self.radius + self.center.y;
            pvec.push(coord! {x: x, y: y});
        }
        pvec.push(*pvec.first().unwrap());

        LineStr { points: pvec }
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
