use std::f64::consts::TAU;

use crate::shapes::polygon::Polygon;
use crate::vec2::Vec2;

/// A circle represented by a center and a radius
#[derive(Clone, PartialEq, Copy)]
pub struct Circle {
    pub center: Vec2,
    pub radius: f64,
}

impl Circle {
    pub fn new(center: Vec2, radius: f64) -> Self {
        Self { center, radius }
    }

    pub fn overlaps(&self, other: &Circle) -> bool {
        self.center.distance(other.center) <= self.radius + other.radius
    }

    pub fn dist(&self, other: &Circle) -> f64 {
        f64::max(
            0.,
            self.center.distance(other.center) - self.radius - other.radius,
        )
    }

    pub fn to_polygon(&self, points: usize) -> Polygon {
        let mut pvec = vec![];
        for i in 0..points {
            let angle = TAU / points as f64 * i as f64;
            let x = angle.cos() * self.radius + self.center.x;
            let y = angle.sin() * self.radius + self.center.y;
            pvec.push(Vec2 { x, y });
        }
        Polygon { points: pvec }
    }
}
