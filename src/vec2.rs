#[derive(Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn euclidean_distance(&self, other: &Vec2) -> f64 {
        f64::abs(self.x - other.x + self.y - other.y)
    }
}
