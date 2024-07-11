use crate::shapes::polygon::Polygon;
use crate::vec2::Vec2;

/// A triangle represented by three vertexes.
#[derive(Clone, PartialEq, Copy)]
pub struct Triangle {
    pub a: Vec2,
    pub b: Vec2,
    pub c: Vec2,
}

impl Triangle {
    pub fn new(a: Vec2, b: Vec2, c: Vec2) -> Self {
        Self { a, b, c }
    }

    pub fn to_polygon(&self) -> Polygon {
        Polygon::new(vec![self.a, self.b, self.c])
    }
}
