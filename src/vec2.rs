use crate::angle::Angle;
use crate::traits::{Lerp, Translate};
use std::f64::consts::PI;
use std::ops::{Add, Mul, Sub};

/// A 2 dimensional vector
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    /// Construct a new vector using provided x and y values
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Construct a new vector using provided angle and length
    pub fn from_angle_length(angle: Angle, length: f64) -> Self {
        Vec2::new(angle.cos() * length, angle.sin() * length)
    }

    /// Construct a new vector from a slice
    pub fn from_slice(slice: &[f64]) -> Self {
        Vec2::new(slice[0], slice[1])
    }

    /// Calculate the euclidean distance between this and another vector
    pub fn distance(&self, other: Vec2) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    /// Calculate the squared euclidean distance between this and another vector
    pub fn distance_squared(&self, other: Vec2) -> f64 {
        (self.x - other.x).powi(2) + (self.y - other.y).powi(2)
    }

    /// TODO: maybe rotate around origin and chain with translate?
    pub fn rotate(&self, center: Vec2, theta: Angle) -> Self {
        let x = theta.cos() * (self.x - center.x) - theta.sin() * (self.y - center.y) + center.x;
        let y = theta.sin() * (self.x - center.x) + theta.cos() * (self.y - center.y) + center.y;
        Vec2::new(x, y)
    }

    /// Calculate the angle respect another point on the plane
    pub fn angle(&self, target: Vec2) -> Angle {
        let signed = f64::atan2(target.y - self.y, target.x - self.x);
        if signed.is_sign_negative() {
            Angle::radians(PI + PI - (-1. * signed))
        } else {
            Angle::radians(signed)
        }
    }

    /// Multiply this vector with a scalar value
    pub fn mul(&self, scalar: f64) -> Vec2 {
        Vec2::new(self.x * scalar, self.y * scalar)
    }

    /// Divide this vector with a scalar value
    pub fn div(&self, scalar: f64) -> Vec2 {
        Vec2::new(self.x / scalar, self.y / scalar)
    }
}

impl Add<Vec2> for Vec2 {
    type Output = Vec2;

    /// Add another vector.
    fn add(self, _rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Vec2;

    /// Subtract another vector.
    fn sub(self, _rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
        }
    }
}

impl Mul<Vec2> for Vec2 {
    type Output = Vec2;

    /// Multiply another vector.
    fn mul(self, _rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x * _rhs.x,
            y: self.y * _rhs.y,
        }
    }
}

impl Mul<f64> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f64) -> Vec2 {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Lerp for Vec2 {
    fn lerp(&self, other: Self, t: f64) -> Self {
        Vec2::new(self.x.lerp(other.x, t), self.y.lerp(other.y, t))
    }
}

impl Translate for Vec2 {
    fn translate(&self, displacement: Vec2) -> Self {
        *self + displacement
    }
}

#[cfg(test)]
mod tests {
    use crate::angle::Angle;
    use crate::vec2::Vec2;

    #[test]
    fn angle() {
        let a = Vec2 { x: 0., y: 0. };
        let b = Vec2 { x: 100., y: 0. };
        assert_eq!(a.angle(b), Angle::degrees(0.));

        let a = Vec2 { x: 0., y: 0. };
        let b = Vec2 { x: 100., y: 100. };
        assert_eq!(a.angle(b), Angle::degrees(45.));

        let a = Vec2 { x: 0., y: 0. };
        let b = Vec2 { x: 0., y: 100. };
        assert_eq!(a.angle(b), Angle::degrees(90.));

        let a = Vec2 { x: 0., y: 0. };
        let b = Vec2 { x: -100., y: 100. };
        assert_eq!(a.angle(b), Angle::degrees(135.));

        let a = Vec2 { x: 0., y: 0. };
        let b = Vec2 { x: -100., y: 0. };
        assert_eq!(a.angle(b), Angle::degrees(180.));

        let a = Vec2 { x: 0., y: 0. };
        let b = Vec2 { x: -100., y: -100. };
        assert_eq!(a.angle(b), Angle::degrees(225.));

        let a = Vec2 { x: 0., y: 0. };
        let b = Vec2 { x: 0., y: -100. };
        assert_eq!(a.angle(b), Angle::degrees(270.));

        let a = Vec2 { x: 0., y: 0. };
        let b = Vec2 { x: 100., y: -100. };
        assert_eq!(a.angle(b), Angle::degrees(315.));
    }

    #[test]
    fn add() {
        let a = Vec2 { x: 2., y: 3. };
        let b = Vec2 { x: 4., y: 7. };
        assert_eq!(a + b, Vec2 { x: 6., y: 10. });
    }

    #[test]
    fn sub() {
        let a = Vec2 { x: 2., y: 3. };
        let b = Vec2 { x: 4., y: 7. };
        assert_eq!(a - b, Vec2 { x: -2., y: -4. });
    }
}
