pub mod shapes;

use geo::coord;
use geo::Coord;
use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;
use anyhow::Context;
use anyhow::Result;

// This is the default resolution both for web (CSS) and Inkscape:
// - https://developer.mozilla.org/en-US/docs/Web/CSS/resolution
// - https://inkscape.org/forums/beyond/resolution-change/
const DPI: f64 = 96.0;

pub enum Uom {
    In,
    Mm,
    Px,
}

pub enum Orientation {
    Landscape,
    Portrait,
}

pub struct PageLayout {
    pub width: f64,
    pub height: f64,
    pub uom: Uom,
    pub orientation: Orientation,
    pub style: Option<String>,
}

impl PageLayout {
    pub fn new(width: f64, height: f64, uom: Uom, orientation: Orientation) -> Self {
        Self {
            width,
            height,
            uom,
            orientation,
            style: None,
        }
    }

    pub fn axidraw_minikit(orientation: Orientation) -> Self {
        Self::new(6.0 * DPI, 4.0 * DPI, Uom::In, orientation)
    }

    pub fn set_style(&mut self, style: String) -> &Self {
        self.style = Some(style);
        self
    }

    pub fn center(&self) -> Coord {
        coord! { x: self.width / 2., y: self.height / 2. }
    }

    pub fn shortest_side(&self) -> f64 {
        f64::min(self.width, self.height)
    }
}

pub fn path(data: Data) -> Path {
    Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", "1px")
        .set("d", data)
}

pub enum Shape {
    Circle(shapes::Circle),
    Rectangle(shapes::Rectangle),
}

pub struct Sketch {
    pub layout: PageLayout,
    elements: Vec<Shape>,
}

impl Sketch {
    pub fn new(layout: PageLayout) -> Self {
        Self {
            layout,
            elements: vec![],
        }
    }

    pub fn add_circle(&mut self, circle: shapes::Circle) {
        self.elements.push(Shape::Circle(circle));
    }

    pub fn add_rect(&mut self, rect: shapes::Rectangle) {
        self.elements.push(Shape::Rectangle(rect));
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

    for e in sketch.elements.iter() {
        match e {
            Shape::Circle(s) => {
                let mut e = svg::node::element::Circle::new()
                    .set("cx", s.center.x)
                    .set("cy", s.center.y)
                    .set("r", s.radius);
                e = e.set("fill", "none");
                e = e.set("stroke", "black");
                doc = doc.add(e);
            }
            Shape::Rectangle(s) => {
                let mut e = svg::node::element::Rectangle::new()
                    .set("x", s.xy.x)
                    .set("y", s.xy.y)
                    .set("width", s.width)
                    .set("height", s.height);
                e = e.set("fill", "none");
                e = e.set("stroke", "black");
                doc = doc.add(e);
            }
        }
    }
    svg::save(path, &doc).context("Cannot save SVG file")?;
    Ok(())
}
