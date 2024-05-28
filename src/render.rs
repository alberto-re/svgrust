use std::f64::consts::PI;
use std::f64::consts::TAU;

use crate::sketch::Sketch;
use crate::uom::Uom;
use crate::vec2::Vec2;
use crate::Shape;
use geo::coord;
use std::time::Instant;
use svg::node::element::path::Data;
use svg::Document;

pub fn render_svg(sketch: &Sketch) -> Document {
    let start = Instant::now();

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
        group = group.set("inkscape:groupmode", "layer");
        group = group.set("inkscape:label", format!("{}", id + 1));
        group = group.set("fill", "none");
        group = group.set("id", format!("layer{}", id + 1));
        group = group.set("stroke", s.stroke.clone());
        group = group.set("stroke-width", s.stroke_width.clone());
        for e in l.elements.iter() {
            match e {
                Shape::Circle(s) => {
                    let center_uom = Uom::convert_vec2(s.center, sketch.uom, Uom::Px);
                    let radius_uom = Uom::convert_scalar(s.radius, sketch.uom, Uom::Px);
                    let e = svg::node::element::Circle::new()
                        .set("cx", center_uom.x)
                        .set("cy", center_uom.y)
                        .set("r", radius_uom);
                    group = group.add(e);
                }
                Shape::Rectangle(s) => {
                    let xy_uom = Uom::convert_vec2(s.xy, sketch.uom, Uom::Px);
                    let width_uom = Uom::convert_scalar(s.width, sketch.uom, Uom::Px);
                    let height_uom = Uom::convert_scalar(s.height, sketch.uom, Uom::Px);
                    let e = svg::node::element::Rectangle::new()
                        .set("x", xy_uom.x)
                        .set("y", xy_uom.y)
                        .set("width", width_uom)
                        .set("height", height_uom);
                    group = group.add(e);
                }
                Shape::LineString(s) => {
                    // TODO: maybe use polyline instead of path?
                    let points_uom = s
                        .points
                        .iter()
                        .map(|p| Uom::convert_vec2(*p, sketch.uom, Uom::Px))
                        .collect::<Vec<Vec2>>();
                    let mut data = Data::new().move_to((points_uom[0].x, points_uom[0].y));
                    for p in points_uom[1..].iter() {
                        data = data.line_to((p.x, p.y));
                    }
                    let e = svg::node::element::Path::new().set("d", data);
                    group = group.add(e);
                }
                Shape::Polygon(s) => {
                    // TODO: maybe use polygon instead of path?
                    let points_uom = s
                        .points
                        .iter()
                        .map(|p| Uom::convert_vec2(*p, sketch.uom, Uom::Px))
                        .collect::<Vec<Vec2>>();
                    let mut data = Data::new().move_to((points_uom[0].x, points_uom[0].y));
                    for p in points_uom[1..].iter() {
                        data = data.line_to((p.x, p.y));
                    }
                    data = data.close();
                    let e = svg::node::element::Path::new().set("d", data);
                    group = group.add(e);
                }
                Shape::MultiPolygon(s) => {
                    for polygon in s.polygons.clone() {
                        let points_uom = polygon
                            .points
                            .iter()
                            .map(|p| Uom::convert_vec2(*p, sketch.uom, Uom::Px))
                            .collect::<Vec<Vec2>>();
                        let mut data = Data::new().move_to((points_uom[0].x, points_uom[0].y));
                        for p in points_uom[1..].iter() {
                            data = data.line_to((p.x, p.y));
                        }
                        data = data.close();
                        let e = svg::node::element::Path::new().set("d", data);
                        group = group.add(e);
                    }
                }
                Shape::Arc(s) => {
                    let center_uom = Uom::convert_vec2(s.center, sketch.uom, Uom::Px);
                    let radius_uom = Uom::convert_scalar(s.radius, sketch.uom, Uom::Px);
                    let p1 = coord! {
                        x: center_uom.x + s.start.cos() * radius_uom,
                        y: center_uom.y + s.start.sin() * radius_uom,
                    };
                    let p2 = coord! {
                        x: center_uom.x + s.end.cos() * radius_uom,
                        y: center_uom.y + s.end.sin() * radius_uom,
                    };
                    let arc_size = if s.end > s.start { s.end } else { s.end + TAU } - s.start;
                    let large_arc = if arc_size > PI { 1 } else { 0 };
                    let arc_parameters = (radius_uom, radius_uom, 0., large_arc, 1, p2.x, p2.y);
                    let data = Data::new()
                        .move_to((p1.x, p1.y))
                        .elliptical_arc_to(arc_parameters);
                    let e = svg::node::element::Path::new().set("d", data);
                    group = group.add(e);
                }
                Shape::Text(s) => {
                    let pos_uom = Uom::convert_vec2(s.pos, sketch.uom, Uom::Px);
                    let e = svg::node::element::Text::new()
                        .add(svg::node::Text::new(s.string.clone()))
                        .set("x", pos_uom.x)
                        .set("y", pos_uom.y);
                    group = group.add(e);
                }
            }
        }
        doc = doc.add(group);
    }
    println!(
        "Time elapsed inside render(): {} milliseconds",
        start.elapsed().as_millis()
    );
    doc
}
