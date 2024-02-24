pub mod grid;
pub mod layout;
pub mod render;
pub mod shapes;
pub mod traits;
pub mod vec2;

use layout::PageLayout;
use shapes::Rect;
use traits::Contains;
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
}
