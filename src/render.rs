use crate::sketch::Sketch;
use crate::uom::Uom;
use crate::vec2::Vec2;
use crate::Shape;
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

    for (id, l) in sketch.groups.iter().enumerate() {
        let mut group = svg::node::element::Group::new();
        group = group.set("inkscape:groupmode", "layer");
        group = group.set("inkscape:label", format!("{}", id + 1));
        group = group.set("fill", "none");
        group = group.set("id", format!("layer{}", id + 1));
        group = group.set("stroke", l.style.stroke.clone());
        group = group.set("stroke-width", l.style.stroke_width.clone());
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
                Shape::Hexagon(s) => {
                    // TODO: maybe use polygon instead of path?
                    // TODO: avoid code duplication
                    let poly = s.to_polygon();
                    let points_uom = poly
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
                Shape::Triangle(s) => {
                    // TODO: maybe use polygon instead of path?
                    // TODO: avoid code duplication
                    let poly = s.to_polygon();
                    let points_uom = poly
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
