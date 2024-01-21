pub mod layout;
pub mod shapes;

use anyhow::Context;
use anyhow::Result;
use geo::coord;
use layout::PageLayout;
use shapes::Rectangle;
use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;

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
        Rectangle::new(coord! { x: 0., y: 0. }, self.layout.width, self.layout.height)
    }
}

pub fn render_svg(sketch: &Sketch, path: &str) -> Result<()> {
    let mut doc = Document::new()
        .set(
            "xmlns:inkscape",
            "http://www.inkscape.org/namespaces/inkscape",
        )
        .set("viewBox", (0, 0, sketch.layout.width, sketch.layout.height))
        .set("width", format!("{}px", sketch.layout.width))
        .set("height", format!("{}px", sketch.layout.height));

    if let Some(style) = &sketch.layout.style {
        doc = doc.set("style", style.to_owned());
    }

    for l in sketch.layers.iter() {
        let mut group = svg::node::element::Group::new();
        group = group.set("fill", "none");
        if let Some(s) = &l.style {
            group = group.set("stroke", s.stroke.clone());
            group = group.set("stroke-width", s.stroke_width.clone());
        }
        for e in l.elements.iter() {
            match e {
                Shape::Circle(s) => {
                    let e = svg::node::element::Circle::new()
                        .set("cx", s.center.x)
                        .set("cy", s.center.y)
                        .set("r", s.radius);
                    group = group.add(e);
                }
                Shape::Rectangle(s) => {
                    let e = svg::node::element::Rectangle::new()
                        .set("x", s.xy.x)
                        .set("y", s.xy.y)
                        .set("width", s.width)
                        .set("height", s.height);
                    group = group.add(e);
                }
            }
        }
        doc = doc.add(group);
    }
    svg::save(path, &doc).context("Cannot save SVG file")?;
    println!("Output written in '{path}'");
    Ok(())
}
