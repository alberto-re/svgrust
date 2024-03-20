use std::f64::consts::PI;
use std::f64::consts::TAU;

use crate::Shape;
use crate::Sketch;
use geo::coord;
use svg::node::element::path::Data;
use svg::Document;

pub fn render_svg(sketch: &Sketch) -> Document {
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

    for (id, (l, s)) in sketch.groups.iter().enumerate() {
        let mut group = svg::node::element::Group::new();
        group = group.set("fill", "none");
        group = group.set("id", (id + 1).to_string());
        group = group.set("stroke", s.stroke.clone());
        group = group.set("stroke-width", s.stroke_width.clone());
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
                Shape::LineString(s) => {
                    let mut data = Data::new().move_to((s.points[0].x, s.points[0].y));
                    for p in s.points[1..].iter() {
                        data = data.line_to((p.x, p.y));
                    }
                    let e = svg::node::element::Path::new().set("d", data);
                    group = group.add(e);
                }
                Shape::Polygon(s) => {
                    let mut data = Data::new().move_to((s.points[0].x, s.points[0].y));
                    for p in s.points[1..].iter() {
                        data = data.line_to((p.x, p.y));
                    }
                    let e = svg::node::element::Path::new().set("d", data);
                    group = group.add(e);
                }
                Shape::Arc(s) => {
                    let p1 = coord! {
                        x: s.center.x + s.start.cos() * s.radius,
                        y: s.center.y + s.start.sin() * s.radius,
                    };
                    let p2 = coord! {
                        x: s.center.x + s.end.cos() * s.radius,
                        y: s.center.y + s.end.sin() * s.radius,
                    };
                    let arc_size = if s.end > s.start { s.end } else { s.end + TAU } - s.start;
                    let large_arc = if arc_size > PI { 1 } else { 0 };
                    let arc_parameters = (s.radius, s.radius, 0., large_arc, 1, p2.x, p2.y);
                    let data = Data::new()
                        .move_to((p1.x, p1.y))
                        .elliptical_arc_to(arc_parameters);
                    let e = svg::node::element::Path::new().set("d", data);
                    group = group.add(e);
                }
            }
        }
        doc = doc.add(group);
    }
    doc
}
