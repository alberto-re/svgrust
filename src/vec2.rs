use crate::angle::Angle;
use crate::traits::{Lerp, Translate};
use std::f64::consts::PI;
use std::ops::{Add, AddAssign, Div, Mul, Sub};

/// A two-dimensional vector.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    /// Create a new `Vec2` with the provided `x` and `y` values.
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Create a new `Vec2` with the provided `angle` and `length`.
    pub fn from_polar(angle: Angle, length: f64) -> Self {
        Vec2::new(angle.cos() * length, angle.sin() * length)
    }

    /// Create a new `Vec2` from the first two values in `slice`.
    pub fn from_slice(slice: &[f64]) -> Self {
        Vec2::new(slice[0], slice[1])
    }

    /// Compute the Euclidean distance between `self` and another `Vec2`.
    pub fn distance(&self, other: Self) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    /// Compute the squared Euclidean distance between `self` and another `Vec2`.
    pub fn distance_squared(&self, other: Self) -> f64 {
        (self.x - other.x).powi(2) + (self.y - other.y).powi(2)
    }

    /// Rotate `self` around the origin by `angle`.
    pub fn rotate(&self, angle: Angle) -> Self {
        Vec2::new(
            angle.cos() * self.x - angle.sin() * self.y,
            angle.sin() * self.x + angle.cos() * self.y,
        )
    }

    /// Rotate `self` around `center` by `angle`.
    pub fn rotate_around(&self, center: Vec2, angle: Angle) -> Self {
        (*self - center).rotate(angle) + center
    }

    /// Compute the angle between `self` and `rhs`.
    pub fn angle_between(&self, rhs: Vec2) -> Angle {
        let signed = f64::atan2(rhs.y - self.y, rhs.x - self.x);
        if signed.is_sign_negative() {
            Angle::radians(PI + PI - (-1. * signed))
        } else {
            Angle::radians(signed)
        }
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

impl AddAssign<Vec2> for Vec2 {
    /// Add another Vec2 and assign the result to Self.
    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Vec2;

    /// Subtract another Vec2.
    fn sub(self, _rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
        }
    }
}

impl Mul<Vec2> for Vec2 {
    type Output = Vec2;

    /// Multiply with another Vec2.
    fn mul(self, _rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x * _rhs.x,
            y: self.y * _rhs.y,
        }
    }
}

impl Mul<f64> for Vec2 {
    type Output = Vec2;

    /// Multiply with a f64 scalar
    fn mul(self, rhs: f64) -> Vec2 {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<f64> for Vec2 {
    type Output = Vec2;

    /// Divide with a f64 scalar
    fn div(self, rhs: f64) -> Vec2 {
        Vec2 {
            x: self.x / rhs,
            y: self.y / rhs,
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
    use approx::assert_relative_eq;
    use rstest::rstest;

    const EPSILON: f64 = 0.00001;

    #[rstest]
    #[case(0., 0., 0., 0.)]
    #[case(45., 2.82843, 2., 2.)]
    #[case(-123.69, 3.60555, -2., -3.)]
    #[case(56.3099, 3.60555, 2., 3.)]
    fn from_polar(
        #[case] angle: f64,
        #[case] length: f64,
        #[case] expected_x: f64,
        #[case] expected_y: f64,
    ) {
        let v = Vec2::from_polar(Angle::degrees(angle), length);
        assert_relative_eq!(v.x, expected_x, epsilon = EPSILON);
        assert_relative_eq!(v.y, expected_y, epsilon = EPSILON);
    }

    #[rstest]
    #[case(2., 2., 2., 3., 1.)]
    #[case(3., 2., 4., 1., 1.4142135)]
    #[case(2., -1., -2., 2., 5.)]
    fn distance(
        #[case] x1: f64,
        #[case] y1: f64,
        #[case] x2: f64,
        #[case] y2: f64,
        #[case] expected: f64,
    ) {
        let a = Vec2 { x: x1, y: y1 };
        let b = Vec2 { x: x2, y: y2 };
        assert_relative_eq!(a.distance(b), expected, epsilon = EPSILON);
    }

    #[rstest]
    #[case(2., 2., 2., 3., 1.)]
    #[case(3., 2., 4., 1., 2.)]
    #[case(2., -1., -2., 2., 25.)]
    fn distance_squared(
        #[case] x1: f64,
        #[case] y1: f64,
        #[case] x2: f64,
        #[case] y2: f64,
        #[case] expected: f64,
    ) {
        let a = Vec2 { x: x1, y: y1 };
        let b = Vec2 { x: x2, y: y2 };
        assert_relative_eq!(a.distance_squared(b), expected, epsilon = EPSILON);
    }

    #[rstest]
    #[case(0., 2., 90., -2., 0.)]
    #[case(0., 2., 180., 0., -2.)]
    #[case(0., 2., 270., 2., 0.)]
    #[case(0., 2., 360., 0., 2.)]
    fn rotate(
        #[case] x: f64,
        #[case] y: f64,
        #[case] angle: f64,
        #[case] expected_x: f64,
        #[case] expected_y: f64,
    ) {
        let v = Vec2::new(x, y).rotate(Angle::degrees(angle));
        assert_relative_eq!(v.x, expected_x, epsilon = EPSILON);
        assert_relative_eq!(v.y, expected_y, epsilon = EPSILON);
    }

    #[rstest]
    #[case(0., 2., 0., 4., 90., 2., 4.)]
    #[case(0., 2., 0., 4., 180., 0., 6.)]
    #[case(0., 2., 0., 4., 270., -2., 4.)]
    #[case(0., 2., 0., 4., 360., 0., 2.)]
    fn rotate_around(
        #[case] x: f64,
        #[case] y: f64,
        #[case] center_x: f64,
        #[case] center_y: f64,
        #[case] angle: f64,
        #[case] expected_x: f64,
        #[case] expected_y: f64,
    ) {
        let center = Vec2::new(center_x, center_y);
        let v = Vec2::new(x, y).rotate_around(center, Angle::degrees(angle));
        assert_relative_eq!(v.x, expected_x, epsilon = EPSILON);
        assert_relative_eq!(v.y, expected_y, epsilon = EPSILON);
    }

    #[rstest]
    #[case(0., 0., 100., 0., 0.)]
    #[case(0., 0., 100., 100., 45.)]
    #[case(0., 0., 0., 100., 90.)]
    #[case(0., 0., -100., 100., 135.)]
    #[case(0., 0., -100., 0., 180.)]
    #[case(0., 0., -100., -100., 225.)]
    #[case(0., 0., 0., -100., 270.)]
    #[case(0., 0., 100., -100., 315.)]
    fn angle_between(
        #[case] x1: f64,
        #[case] y1: f64,
        #[case] x2: f64,
        #[case] y2: f64,
        #[case] expected: f64,
    ) {
        let a = Vec2 { x: x1, y: y1 };
        let b = Vec2 { x: x2, y: y2 };
        assert_eq!(a.angle_between(b), Angle::degrees(expected));
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
