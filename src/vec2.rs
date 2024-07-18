use crate::angle::Angle;
use crate::traits::Lerp;
use std::f64::consts::TAU;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// A two-dimensional vector.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub const ZERO: Self = Self { x: 0., y: 0. };

    /// Create a new `Vec2` with the provided `x` and `y` values.
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Create a new `Vec2` with the provided `angle` and `distance`.
    pub fn from_polar(angle: Angle, distance: f64) -> Self {
        Vec2::new(angle.cos() * distance, angle.sin() * distance)
    }

    /// Create a new `Vec2` from the first two values in `slice`.
    pub fn from_slice(slice: &[f64]) -> Self {
        Vec2::new(slice[0], slice[1])
    }

    /// Convert `self` to polar coordinates.
    pub fn to_polar(&self) -> (Angle, f64) {
        let distance = self.distance(Vec2::ZERO);
        let signed = f64::atan2(self.y, self.x);
        let angle = if signed.is_sign_negative() {
            Angle::from_radians(TAU - (-1. * signed))
        } else {
            Angle::from_radians(signed)
        };
        (angle, distance)
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
            Angle::from_radians(TAU - (-1. * signed))
        } else {
            Angle::from_radians(signed)
        }
    }

    /// Return a new `Vec2` containing the absolute value of each component of `self`.
    pub fn abs(&self) -> Self {
        Vec2::new(self.x.abs(), self.y.abs())
    }

    /// Compute the sum of all components of `self`.
    pub fn sum(&self) -> f64 {
        self.x + self.y
    }
}

impl Lerp for Vec2 {
    fn lerp(&self, rhs: Self, t: f64) -> Self {
        Vec2::new(self.x.lerp(rhs.x, t), self.y.lerp(rhs.y, t))
    }
}

impl Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, _rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

impl AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Div<Vec2> for Vec2 {
    type Output = Vec2;

    fn div(self, _rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x / _rhs.x,
            y: self.y / _rhs.y,
        }
    }
}

impl DivAssign<Vec2> for Vec2 {
    fn div_assign(&mut self, rhs: Vec2) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}
impl Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, _rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
        }
    }
}

impl SubAssign<Vec2> for Vec2 {
    fn sub_assign(&mut self, rhs: Vec2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<Vec2> for Vec2 {
    type Output = Vec2;

    fn mul(self, _rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x * _rhs.x,
            y: self.y * _rhs.y,
        }
    }
}

impl MulAssign<Vec2> for Vec2 {
    fn mul_assign(&mut self, rhs: Vec2) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl Add<f64> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: f64) -> Vec2 {
        Vec2 {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl AddAssign<f64> for Vec2 {
    fn add_assign(&mut self, rhs: f64) {
        self.x += rhs;
        self.y += rhs;
    }
}

impl Div<f64> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f64) -> Vec2 {
        Vec2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl DivAssign<f64> for Vec2 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
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

impl MulAssign<f64> for Vec2 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Sub<f64> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: f64) -> Vec2 {
        Vec2 {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl SubAssign<f64> for Vec2 {
    fn sub_assign(&mut self, rhs: f64) {
        self.x -= rhs;
        self.y -= rhs;
    }
}

impl Sum<Vec2> for Vec2 {
    fn sum<I: Iterator<Item = Vec2>>(iter: I) -> Vec2 {
        iter.fold(Vec2::ZERO, |a, b| Vec2::new(a.x + b.x, a.y + b.y))
    }
}

#[cfg(test)]
mod tests {
    use crate::angle::Angle;
    use crate::traits::Lerp;
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
        let v = Vec2::from_polar(Angle::from_degrees(angle), length);
        assert_relative_eq!(v.x, expected_x, epsilon = EPSILON);
        assert_relative_eq!(v.y, expected_y, epsilon = EPSILON);
    }

    #[rstest]
    #[case(0., 0., 0., 0.)]
    #[case(2., 2., 45., 2.82842)]
    #[case(-2., 2., 135., 2.82842)]
    #[case(-2., -2., 225., 2.82842)]
    #[case(2., -2., 315., 2.82842)]
    fn to_polar(
        #[case] x1: f64,
        #[case] y1: f64,
        #[case] expected_angle: f64,
        #[case] expected_distance: f64,
    ) {
        let v = Vec2::new(x1, y1);
        let (angle, distance) = v.to_polar();
        assert_eq!(angle, Angle::from_degrees(expected_angle));
        assert_relative_eq!(distance, expected_distance, epsilon = EPSILON);
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
        let v1 = Vec2::new(x1, y1);
        let v2 = Vec2::new(x2, y2);
        assert_relative_eq!(v1.distance_squared(v2), expected, epsilon = EPSILON);
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
        let v = Vec2::new(x, y).rotate(Angle::from_degrees(angle));
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
        let v = Vec2::new(x, y).rotate_around(center, Angle::from_degrees(angle));
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
        let v1 = Vec2::new(x1, y1);
        let v2 = Vec2::new(x2, y2);
        assert_eq!(v1.angle_between(v2), Angle::from_degrees(expected));
    }

    #[rstest]
    #[case(0., 0., 0., 0.)]
    #[case(-1., 1., 1., 1.)]
    #[case(-1., -1., 1., 1.)]
    fn abs(#[case] x: f64, #[case] y: f64, #[case] expected_x: f64, #[case] expected_y: f64) {
        let v = Vec2::new(x, y).abs();
        assert_eq!(v.x, expected_x);
        assert_eq!(v.y, expected_y);
    }

    #[rstest]
    #[case(0., 0., 0.)]
    #[case(2., 2., 4.)]
    #[case(-1., 2., 1.)]
    #[case(-1., -3., -4.)]
    fn sum(#[case] x: f64, #[case] y: f64, #[case] expected: f64) {
        let v = Vec2::new(x, y);
        assert_eq!(v.sum(), expected);
    }

    #[rstest]
    #[case(2., 2., 4., 4., 0.5, 3., 3.)]
    fn lerp(
        #[case] x1: f64,
        #[case] y1: f64,
        #[case] x2: f64,
        #[case] y2: f64,
        #[case] t: f64,
        #[case] expected_x: f64,
        #[case] expected_y: f64,
    ) {
        let v1 = Vec2::new(x1, y1);
        let v2 = Vec2::new(x2, y2);
        let v3 = v1.lerp(v2, t);
        assert_relative_eq!(v3.x, expected_x, epsilon = EPSILON);
        assert_relative_eq!(v3.y, expected_y, epsilon = EPSILON);
    }

    #[test]
    fn add_vec() {
        let v1 = Vec2::new(2., 3.);
        let v2 = Vec2::new(4., 7.);
        assert_eq!(v1 + v2, Vec2::new(6., 10.));
    }

    #[test]
    fn add_assign_vec() {
        let mut v = Vec2::new(2., 3.);
        v += Vec2::new(4., 7.);
        assert_eq!(v, Vec2::new(6., 10.));
    }

    #[test]
    fn div_vec() {
        let v1 = Vec2::new(2., 10.);
        let v2 = Vec2::new(2., 2.5);
        assert_eq!(v1 / v2, Vec2::new(1., 4.));
    }

    #[test]
    fn div_assign_vec() {
        let mut v = Vec2::new(2., 10.);
        v /= Vec2::new(2., 2.5);
        assert_eq!(v, Vec2::new(1., 4.));
    }

    #[test]
    fn mul_vec() {
        let v1 = Vec2::new(2., 10.);
        let v2 = Vec2::new(2., 2.5);
        assert_eq!(v1 * v2, Vec2::new(4., 25.));
    }

    #[test]
    fn mul_assign_vec() {
        let mut v = Vec2::new(2., 10.);
        v *= Vec2::new(2., 2.5);
        assert_eq!(v, Vec2::new(4., 25.));
    }

    #[test]
    fn sub_vec() {
        let v1 = Vec2::new(2., 10.);
        let v2 = Vec2::new(2., 2.5);
        assert_eq!(v1 - v2, Vec2::new(0., 7.5));
    }

    #[test]
    fn sub_assign_vec() {
        let mut v = Vec2::new(2., 10.);
        v -= Vec2::new(2., 2.5);
        assert_eq!(v, Vec2::new(0., 7.5));
    }

    #[test]
    fn add_f64() {
        let v = Vec2::new(2., 3.) + 1.5;
        assert_relative_eq!(v.x, 3.5, epsilon = EPSILON);
        assert_relative_eq!(v.y, 4.5, epsilon = EPSILON);
    }

    #[test]
    fn add_assign_f64() {
        let mut v = Vec2::new(2., 3.);
        v += 1.5;
        assert_relative_eq!(v.x, 3.5, epsilon = EPSILON);
        assert_relative_eq!(v.y, 4.5, epsilon = EPSILON);
    }

    #[test]
    fn div_f64() {
        let v = Vec2::new(2., 3.) / 1.5;
        assert_relative_eq!(v.x, 1.33333, epsilon = EPSILON);
        assert_relative_eq!(v.y, 2., epsilon = EPSILON);
    }

    #[test]
    fn div_assign_f64() {
        let mut v = Vec2::new(2., 3.);
        v /= 1.5;
        assert_relative_eq!(v.x, 1.33333, epsilon = EPSILON);
        assert_relative_eq!(v.y, 2., epsilon = EPSILON);
    }

    #[test]
    fn mul_f64() {
        let v = Vec2::new(2., 3.) * 1.5;
        assert_relative_eq!(v.x, 3.0, epsilon = EPSILON);
        assert_relative_eq!(v.y, 4.5, epsilon = EPSILON);
    }

    #[test]
    fn mul_assign_f64() {
        let mut v = Vec2::new(2., 3.);
        v *= 1.5;
        assert_relative_eq!(v.x, 3.0, epsilon = EPSILON);
        assert_relative_eq!(v.y, 4.5, epsilon = EPSILON);
    }

    #[test]
    fn sub_f64() {
        let v = Vec2::new(2., 3.) - 1.5;
        assert_relative_eq!(v.x, 0.5, epsilon = EPSILON);
        assert_relative_eq!(v.y, 1.5, epsilon = EPSILON);
    }

    #[test]
    fn sub_assign_f64() {
        let mut v = Vec2::new(2., 3.);
        v -= 1.5;
        assert_relative_eq!(v.x, 0.5, epsilon = EPSILON);
        assert_relative_eq!(v.y, 1.5, epsilon = EPSILON);
    }

    #[rstest]
    #[case(0., 0., 0., 0., true)]
    #[case(2., 3., 2., 3., true)]
    #[case(2., 3., 2.00001, 3., false)]
    fn eq(
        #[case] x1: f64,
        #[case] y1: f64,
        #[case] x2: f64,
        #[case] y2: f64,
        #[case] expected: bool,
    ) {
        let v1 = Vec2::new(x1, y1);
        let v2 = Vec2::new(x2, y2);
        assert_eq!(v1 == v2, expected);
    }
}
