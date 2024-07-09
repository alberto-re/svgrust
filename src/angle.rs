use std::f64::consts::{PI, TAU};
use std::ops::{Add, AddAssign, Mul, Sub};

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

    /// Create an angle of 0 radians - 0 degrees.
    pub fn zero() -> Self {
        Angle { radians: 0. }
    }

    /// Create an angle of 2PI radians - 360 degrees.
    pub fn tau() -> Self {
        Angle { radians: TAU }
    }

    /// Return this angle expressed in degrees.
    pub fn to_degrees(&self) -> f64 {
        self.radians * 180. / PI
    }

    /// Return this angle expressed in radians.
    pub fn to_radians(&self) -> f64 {
        self.radians
    }

    /// Calculate the sine of this angle.
    pub fn sin(&self) -> f64 {
        self.radians.sin()
    }

    /// Calculate the cosine of this angle.
    pub fn cos(&self) -> f64 {
        self.radians.cos()
    }

    /// Interpolate linearly between two Angles.
    pub fn lerp(&self, other: Angle, t: f64) -> Self {
        *self + (other - *self) * t
    }
}

impl Add<Angle> for Angle {
    type Output = Angle;

    /// Add another Angle.
    fn add(self, _rhs: Angle) -> Angle {
        Angle {
            radians: self.radians + _rhs.radians,
        }
    }
}

impl AddAssign<Angle> for Angle {
    /// Add another Angle and assign the result to Self.
    fn add_assign(&mut self, rhs: Angle) {
        self.radians += rhs.radians
    }
}

impl Sub<Angle> for Angle {
    type Output = Angle;

    /// Subtract another Angle.
    fn sub(self, _rhs: Angle) -> Angle {
        Angle {
            radians: self.radians - _rhs.radians,
        }
    }
}

impl Mul<f64> for Angle {
    type Output = Angle;

    /// Multiply with another Angle.
    fn mul(self, rhs: f64) -> Angle {
        Angle {
            radians: self.radians * rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::angle::Angle;
    use rstest::rstest;
    use std::f64::consts::PI;

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

    #[test]
    fn add() {
        let a = Angle::from_degrees(90.);
        let b = Angle::from_degrees(90.);
        assert_eq!(a + b, Angle::from_degrees(180.));
    }

    #[test]
    fn sub() {
        let a = Angle::from_degrees(90.);
        let b = Angle::from_degrees(90.);
        assert_eq!(a - b, Angle::from_degrees(0.));
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
}
