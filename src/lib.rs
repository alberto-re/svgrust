pub mod angle;
pub mod field;
pub mod grid;
pub mod layout;
pub mod prelude;
pub mod render;
pub mod seed;
pub mod shapes;
pub mod sketch;
pub mod traits;
pub mod uom;
pub mod vec2;
pub mod vec3;

use shapes::{Circle, LineString};
use traits::{Contains, Lerp, ToShape};

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

    pub fn add<T: ToShape>(&mut self, element: T) {
        let shape = element.to_shape();
        match shape {
            Shape::Circle(s) => self.elements.push(Shape::Circle(s)),
            Shape::Arc(s) => self.elements.push(Shape::Arc(s)),
            Shape::Rectangle(s) => self.elements.push(Shape::Rectangle(s)),
            Shape::LineString(s) => self.elements.push(Shape::LineString(s)),
            Shape::Polygon(s) => self.elements.push(Shape::Polygon(s)),
            Shape::MultiPolygon(s) => self.elements.push(Shape::MultiPolygon(s)),
            Shape::Text(s) => self.elements.push(Shape::Text(s)),
        }
    }

    pub fn add_many<T: ToShape>(&mut self, elements: Vec<T>) {
        for element in elements {
            let shape = element.to_shape();
            match shape {
                Shape::Circle(s) => self.elements.push(Shape::Circle(s)),
                Shape::Arc(s) => self.elements.push(Shape::Arc(s)),
                Shape::Rectangle(s) => self.elements.push(Shape::Rectangle(s)),
                Shape::LineString(s) => self.elements.push(Shape::LineString(s)),
                Shape::Polygon(s) => self.elements.push(Shape::Polygon(s)),
                Shape::MultiPolygon(s) => self.elements.push(Shape::MultiPolygon(s)),
                Shape::Text(s) => self.elements.push(Shape::Text(s)),
            }
        }
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
                circles.push(*s)
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
                    inside.add(s.clone());
                } else {
                    outside.add(s.clone());
                }
            }
            Shape::LineString(s) => {
                if bbox.contains(s) {
                    inside.add(s.clone());
                } else {
                    outside.add(s.clone());
                }
            }
            Shape::Polygon(_) => {
                unreachable!();
            }
            Shape::MultiPolygon(_) => {
                unreachable!();
            }
            Shape::Text(_) => {
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

impl Lerp for f64 {
    fn lerp(&self, other: Self, t: f64) -> Self {
        self + t * (other - self)
    }
}