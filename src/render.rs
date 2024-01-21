use crate::Shape;
use crate::Sketch;
use anyhow::Context;
use anyhow::Result;
use svg::Document;

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
