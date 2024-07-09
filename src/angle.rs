use crate::traits::Lerp;
use std::f64::consts::{PI, TAU};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// An abstract representation of an angle.
#[derive(Clone, Copy, PartialEq, Debug, PartialOrd)]
pub struct Angle {
    pub radians: f64,
}

impl Angle {
    /// Create a new `Angle` with the provided `degrees`.
    pub fn from_degrees(degrees: f64) -> Self {
        Angle {
            radians: degrees * PI / 180.,
        }
    }

    /// Create a new `Angle` with the provided `radians`.
    pub fn from_radians(radians: f64) -> Self {
        Angle { radians }
    }

    /// Create a new `Angle` that measures 0 degrees.
    pub fn zero() -> Self {
        Angle { radians: 0. }
    }

    /// Create a new `Angle` that measures 360 degrees.
    pub fn tau() -> Self {
        Angle { radians: TAU }
    }

    /// Return the measure of the `Angle` expressed in degrees.
    pub fn to_degrees(&self) -> f64 {
        self.radians * 180. / PI
    }

    /// Return the measure of the `Angle` expressed in radians.
    pub fn to_radians(&self) -> f64 {
        self.radians
    }

    /// Calculate the sine of this `Angle`.
    pub fn sin(&self) -> f64 {
        self.radians.sin()
    }

    /// Calculate the cosine of this `Angle`.
    pub fn cos(&self) -> f64 {
        self.radians.cos()
    }
}

impl Lerp for Angle {
    fn lerp(&self, rhs: Self, t: f64) -> Self {
        *self + (rhs - *self) * t
    }
}

impl Add<Angle> for Angle {
    type Output = Angle;

    fn add(self, _rhs: Angle) -> Angle {
        Angle {
            radians: self.radians + _rhs.radians,
        }
    }
}

impl AddAssign<Angle> for Angle {
    fn add_assign(&mut self, rhs: Angle) {
        self.radians += rhs.radians
    }
}

impl Sub<Angle> for Angle {
    type Output = Angle;

    fn sub(self, _rhs: Angle) -> Angle {
        Angle {
            radians: self.radians - _rhs.radians,
        }
    }
}

impl SubAssign<Angle> for Angle {
    fn sub_assign(&mut self, rhs: Angle) {
        self.radians -= rhs.radians
    }
}

impl Div<f64> for Angle {
    type Output = Angle;

    fn div(self, rhs: f64) -> Angle {
        Angle::from_radians(self.radians / rhs)
    }
}

impl DivAssign<f64> for Angle {
    fn div_assign(&mut self, rhs: f64) {
        self.radians /= rhs;
    }
}

impl Mul<f64> for Angle {
    type Output = Angle;

    fn mul(self, rhs: f64) -> Angle {
        Angle::from_radians(self.radians * rhs)
    }
}

impl MulAssign<f64> for Angle {
    fn mul_assign(&mut self, rhs: f64) {
        self.radians *= rhs;
    }
}

#[cfg(test)]
mod tests {
    use crate::angle::Angle;
    use crate::traits::Lerp;
    use rstest::rstest;
    use std::f64::consts::PI;

    #[test]
    fn from_degrees() {
        let a = Angle::from_degrees(90.);
        assert_eq!(a.radians, PI / 2.);
    }

    #[test]
    fn to_degrees() {
        let a = Angle::from_degrees(90.);
        let b = Angle::from_radians(PI / 2.);
        assert_eq!(a.to_degrees(), 90.);
        assert_eq!(b.to_degrees(), 90.);
    }

    #[test]
    fn to_radians() {
        let a = Angle::from_degrees(90.);
        let b = Angle::from_radians(PI / 2.);
        assert_eq!(a.to_radians(), PI / 2.);
        assert_eq!(b.to_radians(), PI / 2.);
    }

    #[rstest]
    #[case(90., 270., 0.5, 180.)]
    #[case(0., 360., 0.5, 180.)]
    #[case(0., 360., 0.1, 36.)]
    fn lerp(#[case] a: f64, #[case] b: f64, #[case] t: f64, #[case] expected: f64) {
        assert_eq!(
            Angle::from_degrees(a).lerp(Angle::from_degrees(b), t),
            Angle::from_degrees(expected)
        );
    }

    #[rstest]
    #[case(90., 90., 180.)]
    #[case(90., 270., 360.)]
    #[case(90., -180., -90.)]
    #[case(270., 270., 540.)]
    fn add_angle(#[case] a: f64, #[case] b: f64, #[case] expected: f64) {
        assert_eq!(
            Angle::from_degrees(a) + Angle::from_degrees(b),
            Angle::from_degrees(expected)
        );
    }

    #[rstest]
    #[case(90., 90., 180.)]
    #[case(90., 270., 360.)]
    #[case(90., -180., -90.)]
    #[case(270., 270., 540.)]
    fn add_assign_angle(#[case] a: f64, #[case] b: f64, #[case] expected: f64) {
        let mut theta = Angle::from_degrees(a);
        theta += Angle::from_degrees(b);
        assert_eq!(theta, Angle::from_degrees(expected));
    }

    #[rstest]
    #[case(90., 90., 0.)]
    #[case(90., 270., -180.)]
    #[case(90., -180., 270.)]
    #[case(270., 270., 0.)]
    fn sub_angle(#[case] a: f64, #[case] b: f64, #[case] expected: f64) {
        assert_eq!(
            Angle::from_degrees(a) - Angle::from_degrees(b),
            Angle::from_degrees(expected)
        );
    }

    #[rstest]
    #[case(90., 90., 0.)]
    #[case(90., 270., -180.)]
    #[case(90., -180., 270.)]
    #[case(270., 270., 0.)]
    fn sub_assign_angle(#[case] a: f64, #[case] b: f64, #[case] expected: f64) {
        let mut theta = Angle::from_degrees(a);
        theta -= Angle::from_degrees(b);
        assert_eq!(theta, Angle::from_degrees(expected));
    }

    #[rstest]
    #[case(90., 2., 45.)]
    #[case(360., 2., 180.)]
    fn div_f64(#[case] a: f64, #[case] rhs: f64, #[case] expected: f64) {
        assert_eq!(Angle::from_degrees(a) / rhs, Angle::from_degrees(expected));
    }

    #[rstest]
    #[case(90., 2., 45.)]
    #[case(360., 2., 180.)]
    fn div_assign_f64(#[case] a: f64, #[case] rhs: f64, #[case] expected: f64) {
        let mut theta = Angle::from_degrees(a);
        theta /= rhs;
        assert_eq!(theta, Angle::from_degrees(expected));
    }

    #[rstest]
    #[case(90., 2., 180.)]
    #[case(360., 2., 720.)]
    fn mul_f64(#[case] a: f64, #[case] rhs: f64, #[case] expected: f64) {
        assert_eq!(Angle::from_degrees(a) * rhs, Angle::from_degrees(expected));
    }

    #[rstest]
    #[case(90., 2., 180.)]
    #[case(360., 2., 720.)]
    fn mul_assign_f64(#[case] a: f64, #[case] rhs: f64, #[case] expected: f64) {
        let mut theta = Angle::from_degrees(a);
        theta *= rhs;
        assert_eq!(theta, Angle::from_degrees(expected));
    }
}
