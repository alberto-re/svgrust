use std::{f64::consts::PI, ops};

use crate::traits::Translate;

/// A 2 dimensional vector
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    /// Calculates the euclidean distance between two points on a plane.
    /// 
    /// # Arguments
    ///
    /// * `other` - A Vec2 struct representing the other point
    pub fn euclidean_distance(&self, other: &Vec2) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    pub fn from_angle_length(angle: f64, length: f64) -> Self {
        Vec2 {
            x: angle.cos() * length,
            y: angle.sin() * length,
        }
    }

    pub fn rotate(&self, center: Vec2, theta: f64) -> Self {
        let x = theta.cos() * (self.x - center.x) - theta.sin() * (self.y - center.y) + center.x;
        let y = theta.sin() * (self.x - center.x) + theta.cos() * (self.y - center.y) + center.y;
        Vec2 { x, y }
    }

    pub fn signed_angle(self, target: Vec2) -> f64 {
        f64::atan2(target.y - self.y, target.x - self.x)
    }

    /// Calculates the angle in radians respect another point on a plane.
    /// 
    /// The returned value range is 0 <= n < TAU.
    ///
    /// # Arguments
    ///
    /// * `target` - A Vec2 struct representing the target point
    pub fn angle(self, target: Vec2) -> f64 {
        let signed = self.signed_angle(target);
        if signed.is_sign_negative() {
            PI + PI - (-1. * signed)
        } else {
            signed
        }
    }
}

impl ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, _rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

impl ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, _rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
        }
    }
}

impl Translate for Vec2 {
    fn translate(&self, displacement: Vec2) -> Self {
        *self + displacement
    }
}

#[cfg(test)]
mod tests {
    use crate::Vec2;
    use std::f64::consts::PI;

    #[test]
    fn signed_angle() {
        let a = Vec2 { x: 0., y: 0. };
        let b = Vec2 { x: 100., y: 0. };
        assert_eq!(a.signed_angle(b), 0.);

        let a = Vec2 { x: 0., y: 0. };
        let b = Vec2 { x: 100., y: 100. };
        assert_eq!(a.signed_angle(b), PI / 4.);

        let a = Vec2 { x: 0., y: 0. };
        let b = Vec2 { x: 0., y: 100. };
        assert_eq!(a.signed_angle(b), PI / 2.);

        let a = Vec2 { x: 0., y: 0. };
        let b = Vec2 { x: -100., y: 100. };
        assert_eq!(a.signed_angle(b), 3. * PI / 4.);

        let a = Vec2 { x: 0., y: 0. };
        let b = Vec2 { x: -100., y: 0. };
        assert_eq!(a.signed_angle(b), PI);

        let a = Vec2 { x: 0., y: 0. };
        let b = Vec2 { x: -100., y: -100. };
        assert_eq!(a.signed_angle(b), -(3. * PI / 4.));

        let a = Vec2 { x: 0., y: 0. };
        let b = Vec2 { x: 0., y: -100. };
        assert_eq!(a.signed_angle(b), -(PI / 2.));

        let a = Vec2 { x: 0., y: 0. };
        let b = Vec2 { x: 100., y: -100. };
        assert_eq!(a.signed_angle(b), -(PI / 4.));
    }

    #[test]
    fn unsigned_angle() {
        let a = Vec2 { x: 0., y: 0. };
        let b = Vec2 { x: 100., y: 0. };
        assert_eq!(a.angle(b), 0.);

        let a = Vec2 { x: 0., y: 0. };
        let b = Vec2 { x: 100., y: 100. };
        assert_eq!(a.angle(b), PI / 4.);

        let a = Vec2 { x: 0., y: 0. };
        let b = Vec2 { x: 0., y: 100. };
        assert_eq!(a.angle(b), PI / 2.);

        let a = Vec2 { x: 0., y: 0. };
        let b = Vec2 { x: -100., y: 100. };
        assert_eq!(a.angle(b), 3. * PI / 4.);

        let a = Vec2 { x: 0., y: 0. };
        let b = Vec2 { x: -100., y: 0. };
        assert_eq!(a.angle(b), PI);

        let a = Vec2 { x: 0., y: 0. };
        let b = Vec2 { x: -100., y: -100. };
        assert_eq!(a.angle(b), 5. * PI / 4.);

        let a = Vec2 { x: 0., y: 0. };
        let b = Vec2 { x: 0., y: -100. };
        assert_eq!(a.angle(b), 3. * PI / 2.);

        let a = Vec2 { x: 0., y: 0. };
        let b = Vec2 { x: 100., y: -100. };
        assert_eq!(a.angle(b), 7. * PI / 4.);
    }
}
