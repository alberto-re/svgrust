use std::f64::consts::PI;
use std::f64::consts::TAU;

use crate::angle::Angle;
use crate::map_range;
use crate::vec2::Vec2;
use crate::vec3::Vec3;
use noise::NoiseFn;
use noise::Perlin;

pub trait Scalar2 {
    fn number2(&self, pos: Vec2) -> f64;
    fn angle2(&self, pos: Vec2, increments: f64) -> Angle;
}

pub trait Scalar3 {
    fn number3(&self, pos: Vec3) -> f64;
    fn angle3(&self, pos: Vec3, increments: f64) -> Angle;
}

pub trait Vector2to2 {
    fn vec2(&self, pos: Vec2) -> Vec2;
}

pub trait Vector3to2 {
    fn vec3(&self, pos: Vec3) -> Vec2;
}

pub struct PerlinField {
    noise_fn: Perlin,
}

impl PerlinField {
    // TODO: make seed any T that implements into<u32>
    pub fn new(seed: u32) -> Self {
        let perlin = Perlin::new(seed);
        PerlinField { noise_fn: perlin }
    }
}

impl Scalar2 for PerlinField {
    fn number2(&self, pos: Vec2) -> f64 {
        self.noise_fn.get([pos.x, pos.y])
    }

    fn angle2(&self, pos: Vec2, increment: f64) -> Angle {
        let noise_val = self.number2(pos);
        if increment == 0. {
            return Angle::from_radians(map_range(noise_val, -1., 1., 0., TAU));
        }
        let angle = PI * noise_val;
        Angle::from_radians((angle / increment).round() * increment)
    }
}

impl Scalar3 for PerlinField {
    fn number3(&self, pos: Vec3) -> f64 {
        self.noise_fn.get([pos.x, pos.y, pos.z])
    }

    fn angle3(&self, pos: Vec3, increment: f64) -> Angle {
        let noise_val = self.number3(pos);
        if increment == 0. {
            return Angle::from_radians(map_range(noise_val, -1., 1., 0., TAU));
        }
        let angle = PI * noise_val;
        Angle::from_radians((angle / increment).round() * increment)
    }
}

impl Vector2to2 for PerlinField {
    fn vec2(&self, pos: Vec2) -> Vec2 {
        let noise_val = self.number2(pos);
        let angle = Angle::from_radians(map_range(noise_val, -1., 1., 0., TAU));
        Vec2::from_polar(angle, 1.)
    }
}

impl Vector3to2 for PerlinField {
    fn vec3(&self, pos: Vec3) -> Vec2 {
        let noise_val = self.number3(pos);
        let angle = Angle::from_radians(map_range(noise_val, -1., 1., 0., TAU));
        Vec2::from_polar(angle, 1.)
    }
}

pub struct SpiralField {
    center: Vec2,
}

impl SpiralField {
    pub fn new(center: Vec2) -> Self {
        SpiralField { center }
    }
}

impl Scalar2 for SpiralField {
    fn number2(&self, pos: Vec2) -> f64 {
        let dx = pos.x - self.center.x;
        let dy = pos.y - self.center.y;
        f64::atan2(dy, dx)
    }

    fn angle2(&self, pos: Vec2, increment: f64) -> Angle {
        let noise_val = self.number2(pos);
        if increment == 0. {
            return Angle::from_radians(map_range(noise_val, -1., 1., 0., TAU));
        }
        let angle = PI * noise_val;
        Angle::from_radians((angle / increment).round() * increment)
    }
}

impl Vector2to2 for SpiralField {
    fn vec2(&self, pos: Vec2) -> Vec2 {
        let val = self.number2(pos);
        let angle = map_range(val, -PI, PI, 0., TAU);
        let angle = angle + PI / 2.5;
        Vec2::from_polar(Angle::from_radians(angle), 0.5)
    }
}
