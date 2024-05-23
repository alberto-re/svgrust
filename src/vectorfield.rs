use std::f64::consts::PI;
use std::f64::consts::TAU;

use crate::angle::Angle;
use crate::map_range;
use crate::vec2::Vec2;
use noise::NoiseFn;
use noise::Perlin;

pub trait ValueAt {
    fn value_at(&self, pos: Vec2) -> f64;
}

pub trait AngleAt {
    fn angle_at(&self, pos: Vec2, increments: f64) -> Angle;
}

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

impl ValueAt for PerlinNoise2dVectorField {
    fn value_at(&self, pos: Vec2) -> f64 {
        self.noise_fn
            .get([pos.x * self.x_factor, pos.y * self.y_factor])
    }
}

impl AngleAt for PerlinNoise2dVectorField {
    fn angle_at(&self, pos: Vec2, increment: f64) -> Angle {
        let noise_val = self.value_at(pos);
        if increment == 0. {
            return Angle::radians(map_range(noise_val, -1., 1., 0., TAU));
        }
        let angle = PI * noise_val;
        Angle::radians((angle / increment).round() * increment)
    }
}

impl VectorAt for PerlinNoise2dVectorField {
    fn vector_at(&self, pos: Vec2) -> Vec2 {
        let noise_val = self.value_at(pos);
        let angle = Angle::radians(map_range(noise_val, -1., 1., 0., TAU));
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

impl ValueAt for Spiral2dVectorField {
    fn value_at(&self, pos: Vec2) -> f64 {
        let dx = pos.x - self.center.x;
        let dy = pos.y - self.center.y;
        f64::atan2(dy, dx)
    }
}

impl VectorAt for Spiral2dVectorField {
    fn vector_at(&self, pos: Vec2) -> Vec2 {
        let val = self.value_at(pos);
        let angle = map_range(val, -PI, PI, 0., TAU);
        let angle = angle + PI / 2.5;
        Vec2::from_angle_length(Angle::radians(angle), 0.5)
    }
}

pub struct CurlNoise2dVectorField {
    derivative_sample: f64,
    noise_vectorfield: PerlinNoise2dVectorField,
}

impl CurlNoise2dVectorField {
    pub fn new(derivative_sample: f64, x_factor: f64, y_factor: f64, noise_seed: u32) -> Self {
        let noise_vectorfield = PerlinNoise2dVectorField::new(x_factor, y_factor, noise_seed);
        CurlNoise2dVectorField {
            derivative_sample,
            noise_vectorfield,
        }
    }
}

impl VectorAt for CurlNoise2dVectorField {
    fn vector_at(&self, pos: Vec2) -> Vec2 {
        let x1 = self
            .noise_vectorfield
            .value_at(pos + Vec2::new(self.derivative_sample, 0.));
        let x2 = self
            .noise_vectorfield
            .value_at(pos - Vec2::new(self.derivative_sample, 0.));
        let y1 = self
            .noise_vectorfield
            .value_at(pos + Vec2::new(0., self.derivative_sample));
        let y2 = self
            .noise_vectorfield
            .value_at(pos - Vec2::new(0., self.derivative_sample));
        let xd = x2 - x1;
        let yd = y2 - y1;
        let angle = f64::atan2(yd, xd);
        Vec2::from_angle_length(Angle::radians(angle) + Angle::radians(PI), 1.)
    }
}
