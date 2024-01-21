pub mod layout;
pub mod render;
pub mod shapes;

use geo::coord;
use layout::PageLayout;
use shapes::Rectangle;
use svg::node::element::path::Data;
use svg::node::element::Path;

pub fn path(data: Data) -> Path {
    Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", "1px")
        .set("d", data)
}

#[derive(Clone)]
pub enum Shape {
    Circle(shapes::Circle),
    Rectangle(shapes::Rectangle),
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
    pub fn new() -> Self {
        Self {
            elements: vec![],
            style: None,
        }
    }

    pub fn add_circle(&mut self, circle: shapes::Circle) {
        self.elements.push(Shape::Circle(circle));
    }

    pub fn add_rect(&mut self, rect: shapes::Rectangle) {
        self.elements.push(Shape::Rectangle(rect));
    }

    pub fn set_style(&mut self, style: Style) -> Self {
        self.style = Some(style);
        self.clone()
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
