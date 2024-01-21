use std::f64::consts::TAU;

use geo::coord;
use geo::Coord;
use geo::EuclideanDistance;
use rand::Rng;

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
