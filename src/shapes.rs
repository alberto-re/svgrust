use std::f64::consts::TAU;

use geo::MultiLineString;
use geo::MultiPolygon;
use geo::Polygon;
use geo::coord;
use geo::Coord;
use geo::EuclideanDistance;
use rand::Rng;
use geo::algorithm::bool_ops::BooleanOps;

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

impl Clippable<LineString> for LineString {
    fn clipped(&self, _: &Rectangle) -> LineString {
        let ls = geo::LineString(self.points.clone());
        let mls = MultiLineString::new(vec![ls]);
        let poly_lstr = geo::LineString::new(vec![
            coord!{x: 100., y: 100.},
            coord!{x: 200., y: 100.},
            coord!{x: 200., y: 200.},
            coord!{x: 100., y: 200.},
        ]);
        let poly = Polygon::new(poly_lstr, vec![]);
        let mpoly = MultiPolygon::new(vec![poly]);
        let res = mpoly.clip(&mls, false);
        let mut newcoords: Vec<Coord> = vec![]; 
        for coord in res.0.first().unwrap().coords() {
            newcoords.push(coord.clone());
        }
        LineString::new(newcoords)
    }
}

#[derive(Clone, PartialEq)]
pub struct Rectangle {
    pub xy: Coord,
    pub width: f64,
    pub height: f64,
    stroke: String,
    stroke_width: String,
}

impl Rectangle {
    pub fn new(xy: Coord, width: f64, height: f64) -> Self {
        Self {
            xy,
            width,
            height,
            stroke: "black".to_string(),
            stroke_width: "".to_string(),
        }
    }
}

impl Scalable<Rectangle> for Rectangle {
    fn scaled(&self, perc: f64) -> Rectangle {
        Rectangle::new(
            coord! { x: self.xy.x + self.width * ((1. - perc) / 2.), y: self.xy.y + self.height * ((1. - perc) / 2.) },
            self.width * perc,
            self.height * perc,
        )
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

    pub fn rnd_uniform(&self) -> Coord {
        let mut rng = rand::thread_rng();
        let r_sqrt = (rng.gen::<f64>() * self.radius * self.radius).sqrt();
        let angle = rng.gen::<f64>() * TAU;
        let x = r_sqrt * angle.cos() + self.center.x;
        let y = r_sqrt * angle.sin() + self.center.y;
        coord! { x: x, y: y }
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

pub trait Scalable<T> {
    fn scaled(&self, perc: f64) -> T;
}

pub trait Clippable<T> {
    fn clipped(&self, bbox: &Rectangle) -> T;
}
