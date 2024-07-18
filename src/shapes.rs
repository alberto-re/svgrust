pub mod circle;
pub mod edge;
pub mod hexagon;
pub mod linestring;
pub mod polygon;
pub mod rectangle;
pub mod triangle;

use crate::shapes::linestring::LineString;
use crate::vec2::Vec2;

#[derive(Clone, PartialEq)]
pub struct Text {
    pub pos: Vec2,
    pub string: String,
}

impl Text {
    pub fn new(pos: Vec2, string: &str) -> Self {
        Self {
            pos,
            string: string.to_string(),
        }
    }
}
