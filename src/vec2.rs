use std::ops;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn euclidean_distance(&self, other: &Vec2) -> f64 {
        f64::abs(self.x - other.x) + f64::abs(self.y - other.y)
    }

    pub fn from_angle_length(angle: f64, length: f64) -> Self {
        Vec2 {
            x: angle.cos() * length,
            y: angle.sin() * length,
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
