use std::f64::consts::PI;
use std::ops;

/// An abstract represenation of an angle.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Angle {
    pub radians: f64,
}

impl Angle {
    /// Create an angle from a value in degrees.
    pub fn from_degrees(degrees: f64) -> Self {
        Angle {
            radians: degrees * PI / 180.,
        }
    }

    /// Create an angle from a value in radians.
    pub fn from_radians(radians: f64) -> Self {
        Angle { radians }
    }

    /// Return this angle expressed in degrees.
    pub fn as_degrees(&self) -> f64 {
        self.radians * 180. / PI
    }

    /// Return this angle expressed in radians.
    pub fn as_radians(&self) -> f64 {
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
}

impl ops::Add<Angle> for Angle {
    type Output = Angle;

    /// Add another Angle.
    fn add(self, _rhs: Angle) -> Angle {
        Angle {
            radians: self.radians + _rhs.radians,
        }
    }
}

impl ops::Sub<Angle> for Angle {
    type Output = Angle;

    /// Subtract another Angle.
    fn sub(self, _rhs: Angle) -> Angle {
        Angle {
            radians: self.radians - _rhs.radians,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::angle::Angle;
    use std::f64::consts::PI;

    #[test]
    fn as_degrees() {
        let a = Angle::from_degrees(90.);
        let b = Angle::from_radians(PI / 2.);
        assert_eq!(a.as_degrees(), 90.);
        assert_eq!(b.as_degrees(), 90.);
    }

    #[test]
    fn as_radians() {
        let a = Angle::from_degrees(90.);
        let b = Angle::from_radians(PI / 2.);
        assert_eq!(a.as_radians(), PI / 2.);
        assert_eq!(b.as_radians(), PI / 2.);
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
}
