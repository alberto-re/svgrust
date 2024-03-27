pub mod angle;
pub mod grid;
pub mod layout;
pub mod prelude;
pub mod render;
pub mod shapes;
pub mod sketch;
pub mod traits;
pub mod vec2;
pub mod vectorfield;

use shapes::{Circle, LineString};
use traits::Contains;

#[derive(Clone)]
pub enum Shape {
    Arc(shapes::Arc),
    Circle(shapes::Circle),
    Rectangle(shapes::Rect),
    LineString(shapes::LineString),
    Polygon(shapes::Polygon),
}

#[derive(Clone)]
pub struct Style {
    stroke: String,
    stroke_width: String,
}

impl Style {
    pub fn new(stroke: &str, stroke_width: &str) -> Self {
        Self {
            stroke: stroke.to_string(),
            stroke_width: stroke_width.to_string(),
        }
    }
}

#[derive(Clone)]
pub struct Group {
    pub elements: Vec<Shape>,
}

impl Group {
    pub fn new() -> Self {
        Self { elements: vec![] }
    }

    pub fn add_circle(&mut self, circle: &shapes::Circle) {
        self.elements.push(Shape::Circle(circle.clone()));
    }

    pub fn add_arc(&mut self, arc: &shapes::Arc) {
        self.elements.push(Shape::Arc(arc.clone()));
    }

    pub fn add_rect(&mut self, rect: &shapes::Rect) {
        self.elements.push(Shape::Rectangle(rect.clone()));
    }

    pub fn add_linestring(&mut self, linesting: &shapes::LineString) {
        self.elements.push(Shape::LineString(linesting.clone()));
    }

    pub fn add_polygon(&mut self, polygon: &shapes::Polygon) {
        self.elements.push(Shape::Polygon(polygon.clone()));
    }

    pub fn add_linestrings(&mut self, linestr: &[shapes::LineString]) {
        linestr.iter().for_each(|l| {
            self.elements.push(Shape::LineString(l.clone()));
        });
    }

    pub fn linestrings(&self) -> Vec<LineString> {
        let mut lstrs = vec![];
        self.elements.iter().for_each(|e| {
            if let Shape::LineString(s) = e {
                lstrs.push(s.clone())
            }
        });
        lstrs
    }

    pub fn circles(&self) -> Vec<Circle> {
        let mut circles = vec![];
        self.elements.iter().for_each(|e| {
            if let Shape::Circle(s) = e {
                circles.push(s.clone())
            }
        });
        circles
    }

    pub fn split_shape<T: Contains>(&self, bbox: &T) -> (Group, Group) {
        let mut inside = Group::default();
        let mut outside = Group::default();
        self.elements.iter().for_each(|e| match e {
            Shape::Circle(_) => {
                unreachable!();
            }
            Shape::Arc(_) => {
                unreachable!();
            }
            Shape::Rectangle(s) => {
                if bbox.contains(s) {
                    inside.add_rect(&s.clone());
                } else {
                    outside.add_rect(&s.clone());
                }
            }
            Shape::LineString(s) => {
                if bbox.contains(s) {
                    inside.add_linestring(&s.clone());
                } else {
                    outside.add_linestring(&s.clone());
                }
            }
            Shape::Polygon(_) => {
                unreachable!();
            }
        });
        (inside, outside)
    }
}

impl Default for Group {
    fn default() -> Self {
        Self::new()
    }
}

pub fn map_range(n: f64, start1: f64, stop1: f64, start2: f64, stop2: f64) -> f64 {
    (n - start1) / (stop1 - start1) * (stop2 - start2) + start2
}

pub fn clamp(n: f64, min: f64, max: f64) -> f64 {
    f64::max(min, f64::min(max, n))
}
