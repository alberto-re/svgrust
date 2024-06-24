pub mod angle;
pub mod field;
pub mod grid;
pub mod group;
pub mod layout;
pub mod pen;
pub mod prelude;
pub mod render;
pub mod seed;
pub mod shapes;
pub mod sketch;
pub mod style;
pub mod traits;
pub mod uom;
pub mod vec2;
pub mod vec3;

use traits::Lerp;

#[derive(Clone)]
pub enum Shape {
    Arc(shapes::Arc),
    Circle(shapes::Circle),
    Rectangle(shapes::Rect),
    LineString(shapes::LineString),
    Polygon(shapes::Polygon),
    MultiPolygon(shapes::MultiPolygon),
    Text(shapes::Text),
}

pub fn map_range(n: f64, start1: f64, stop1: f64, start2: f64, stop2: f64) -> f64 {
    (n - start1) / (stop1 - start1) * (stop2 - start2) + start2
}

pub fn clamp(n: f64, min: f64, max: f64) -> f64 {
    f64::max(min, f64::min(max, n))
}

impl Lerp for f64 {
    fn lerp(&self, other: Self, t: f64) -> Self {
        self + t * (other - self)
    }
}
