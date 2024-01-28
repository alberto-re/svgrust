use anyhow::Result;
use geo::coord;
use geo::Coord;
use noise::{NoiseFn, Perlin};
use plt::layout::Orientation::Portrait;
use plt::layout::PageLayout;
use plt::render::render_svg;
use plt::shapes::Centroid;
use plt::shapes::Circle;
use plt::shapes::LineString;
use plt::shapes::Rect;
use plt::shapes::Scale;
use plt::Group;
use plt::Shape;
use plt::Sketch;
use plt::Style;
use rand::Rng;

// https://openprocessing.org/sketch/2008342

fn main() -> Result<()> {
    let mut sketch = Sketch::new(PageLayout::axidraw_minikit(Portrait));
    let mut layer = Group::new();
    let enclosing =
        Rect::square_with_center(sketch.centroid(), sketch.as_rect().scale(0.99).min_len());
    let cells: u8 = 60;
    let side: f64 = enclosing.min_len() / cells as f64;
    let mut rng = rand::thread_rng();
    (0..cells).for_each(|c| {
        (0..cells).for_each(|r| {
            let mut points: Vec<Coord> = vec![];
            if rng.gen::<f64>() < 0.5 {
                points.push(coord! { x: c as f64 * side, y: r as f64 * side });
                points.push(coord! { x: c as f64 * side + side, y: r as f64 * side + side });
            } else {
                points.push(coord! { x: c as f64 * side + side, y: r as f64 * side });
                points.push(coord! { x: c as f64 * side, y: r as f64 * side + side });
            }
            layer.add_lstr(&LineString::new(points).add_vec(enclosing.xy));
        })
    });
    let (l1, _) = layer.split_shape(Circle::new(
        sketch.centroid(),
        enclosing.scale(0.96).min_len() / 2.,
    ));
    let mut l2 = Group::new();
    let mut l3 = Group::new();
    let mut l4 = Group::new();

    let perlin = Perlin::new(19);

    l1.elements.iter().for_each(|e| {
        if let Shape::LineString(s) = e {
            let val = perlin.get([s.centroid().x * 0.005, s.centroid().y * 0.015]);
            if val < 0.33 {
                l2.elements.push(e.clone());
            } else if rng.gen::<f64>() < 0.66 {
                l3.elements.push(e.clone());
            } else {
                l4.elements.push(e.clone());
            }
        }
    });
    sketch.add_group(&l2, &Style::new("blue", "1.5px"));
    sketch.add_group(&l3, &Style::new("orange", "1.5px"));
    sketch.add_group(&l4, &Style::new("purple", "1.5px"));
    render_svg(&sketch, "/Users/are/Desktop/prova.svg")?;
    Ok(())
}
