pub mod angle;
pub mod grid;
pub mod layout;
pub mod render;
pub mod shapes;
pub mod traits;
pub mod vec2;

use layout::PageLayout;
use shapes::{Circle, LineStr, Rect};
use traits::{Centroid, Contains};
use vec2::Vec2;

#[derive(Clone)]
pub enum Shape {
    Arc(shapes::Arc),
    Circle(shapes::Circle),
    Rectangle(shapes::Rect),
    LineString(shapes::LineStr),
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

    pub fn add_lstr(&mut self, linestr: &shapes::LineStr) {
        self.elements.push(Shape::LineString(linestr.clone()));
    }

    pub fn add_lstrs(&mut self, linestr: &[shapes::LineStr]) {
        linestr.iter().for_each(|l| {
            self.elements.push(Shape::LineString(l.clone()));
        });
    }

    pub fn linestrings(&self) -> Vec<LineStr> {
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
                    inside.add_lstr(&s.clone());
                } else {
                    outside.add_lstr(&s.clone());
                }
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

pub struct Sketch {
    pub layout: PageLayout,
    groups: Vec<(Group, Style)>,
}

impl Sketch {
    pub fn new(layout: &PageLayout) -> Self {
        Self {
            layout: layout.clone(),
            groups: vec![],
        }
    }

    pub fn add_group(&mut self, layer: &Group, style: &Style) {
        self.groups.push((layer.clone(), style.clone()));
    }

    pub fn as_rect(&self) -> Rect {
        Rect::new(Vec2 { x: 0., y: 0. }, self.layout.width, self.layout.height)
    }

    pub fn center(&self) -> Vec2 {
        self.as_rect().centroid()
    }

    pub fn width(&self) -> f64 {
        self.as_rect().width
    }

    pub fn height(&self) -> f64 {
        self.as_rect().height
    }

    pub fn top_middle(&self, margin: f64) -> Vec2 {
        Vec2 {
            x: self.as_rect().width / 2.,
            y: margin,
        }
    }

    pub fn bottom_middle(&self, margin: f64) -> Vec2 {
        Vec2 {
            x: self.as_rect().width / 2.,
            y: self.as_rect().height - margin,
        }
    }

    pub fn left_middle(&self, margin: f64) -> Vec2 {
        Vec2 {
            x: margin,
            y: self.as_rect().height / 2.,
        }
    }

    pub fn right_middle(&self, margin: f64) -> Vec2 {
        Vec2 {
            x: self.as_rect().width - margin,
            y: self.as_rect().height / 2.,
        }
    }
}

pub fn map_range(n: f64, start1: f64, stop1: f64, start2: f64, stop2: f64) -> f64 {
    (n - start1) / (stop1 - start1) * (stop2 - start2) + start2
}
