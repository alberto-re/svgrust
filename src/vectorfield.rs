use crate::{angle::Angle, shapes::Circle, vec2::Vec2};

pub trait VectorAt {
    fn vector_at(&self, pos: Vec2) -> Vec2;
}

pub struct RadiusVectorField {
    circle: Circle,
    attractive: bool,
}

impl RadiusVectorField {
    pub fn new(circle: Circle, attractive: bool) -> Self {
        RadiusVectorField { circle, attractive }
    }
}

impl VectorAt for RadiusVectorField {
    fn vector_at(&self, pos: Vec2) -> Vec2 {
        let dx = pos.x - self.circle.center.x;
        let dy = pos.y - self.circle.center.y;
        let val = f64::atan2(dy, dx);
        let mut factor = if self.attractive { -1. } else { 1. };
        if pos.euclidean_distance(&self.circle.center) < self.circle.radius {
            factor *= -1.;
        }
        Vec2::from_angle_length(Angle::from_radians(val), factor)
    }
}
