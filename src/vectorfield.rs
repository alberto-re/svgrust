use std::f64::consts::PI;
use std::f64::consts::TAU;

use crate::angle::Angle;
use crate::map_range;
use crate::vec2::Vec2;
use noise::NoiseFn;
use noise::Perlin;

pub trait VectorAt {
    fn vector_at(&self, pos: Vec2) -> Vec2;
}

pub struct PerlinNoise2dVectorField {
    x_factor: f64,
    y_factor: f64,
    noise_fn: Perlin,
}

impl PerlinNoise2dVectorField {
    pub fn new(x_factor: f64, y_factor: f64, noise_seed: u32) -> Self {
        let perlin = Perlin::new(noise_seed);
        PerlinNoise2dVectorField {
            x_factor,
            y_factor,
            noise_fn: perlin,
        }
    }
}

impl VectorAt for PerlinNoise2dVectorField {
    fn vector_at(&self, pos: Vec2) -> Vec2 {
        let noise_val = self
            .noise_fn
            .get([pos.x * self.x_factor, pos.y * self.y_factor]);
        let angle = Angle::from_radians(map_range(noise_val, -1., 1., 0., TAU));
        Vec2::from_angle_length(angle, 1.)
    }
}

pub struct Spiral2dVectorField {
    center: Vec2,
}

impl Spiral2dVectorField {
    pub fn new(center: Vec2) -> Self {
        Spiral2dVectorField { center }
    }
}

impl VectorAt for Spiral2dVectorField {
    fn vector_at(&self, pos: Vec2) -> Vec2 {
        let dx = pos.x - self.center.x;
        let dy = pos.y - self.center.y;
        let val = f64::atan2(dy, dx);
        let angle = map_range(val, -PI, PI, 0., TAU);
        let angle = angle + PI / 2.5;
        Vec2::from_angle_length(Angle::from_radians(angle), 0.5)
    }
}
