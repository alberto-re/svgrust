pub mod layout;
pub mod render;
pub mod shapes;

use geo::coord;
use layout::PageLayout;
use shapes::Rectangle;

#[derive(Clone)]
pub enum Shape {
    Circle(shapes::Circle),
    Rectangle(shapes::Rectangle),
    LineString(shapes::LineString),
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
pub struct Layer {
    elements: Vec<Shape>,
    style: Option<Style>,
}

impl Layer {
    pub fn new(elements: Vec<Shape>, style: Option<Style>) -> Self {
        Self { elements, style }
    }

    pub fn add_circle(&mut self, circle: shapes::Circle) {
        self.elements.push(Shape::Circle(circle));
    }

    pub fn add_rect(&mut self, rect: shapes::Rectangle) {
        self.elements.push(Shape::Rectangle(rect));
    }

    pub fn add_linestr(&mut self, linestr: shapes::LineString) {
        self.elements.push(Shape::LineString(linestr));
    }

    pub fn set_style(&mut self, style: Style) -> Self {
        self.style = Some(style);
        self.clone()
    }
}

impl Default for Layer {
    fn default() -> Self {
        Self::new(vec![], None)
    }
}

pub struct Sketch {
    pub layout: PageLayout,
    layers: Vec<Layer>,
}

impl Sketch {
    pub fn new(layout: PageLayout) -> Self {
        Self {
            layout,
            layers: vec![],
        }
    }

    pub fn add_layer(&mut self, layer: Layer) {
        self.layers.push(layer);
    }

    pub fn as_rect(&self) -> Rectangle {
        Rectangle::new(
            coord! { x: 0., y: 0. },
            self.layout.width,
            self.layout.height,
        )
    }
}
