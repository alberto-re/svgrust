use anyhow::Result;
use geo::coord;
use geo::Coord;
use plt::layout::Orientation::Portrait;
use plt::layout::PageLayout;
use plt::render::render_svg;
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
    let mut layer = Group::new().set_style(Style::new("black", "1px"));
    let enclosing =
        Rect::square_with_center(sketch.centroid(), sketch.as_rect().scale(0.99).min_len());
    let cols: u8 = 70;
    let rows: u8 = 70;
    let side: f64 = enclosing.min_len() / cols as f64;
    let mut rng = rand::thread_rng();
    (0..cols).for_each(|c| {
        (0..rows).for_each(|r| {
            let mut points: Vec<Coord> = vec![];

            if rng.gen::<f64>() < 0.5 {
                points.push(coord! { x: c as f64 * side, y: r as f64 * side });
                points.push(coord! { x: c as f64 * side + side, y: r as f64 * side + side });
            } else {
                points.push(coord! { x: c as f64 * side + side, y: r as f64 * side });
                points.push(coord! { x: c as f64 * side, y: r as f64 * side + side });
            }

            points.iter_mut().for_each(|p| {
                p.x += enclosing.xy.x;
                p.y += enclosing.xy.y;
            });

            layer.add_lstr(&LineString::new(points));
        })
    });
    let (mut l1, mut l2) = layer.split_shape(Circle::new(
        sketch.centroid(),
        enclosing.scale(0.75).min_len() / 2.,
    ));
    let l0 = Group::new().set_style(Style::new("black", "1px"));
    // l0.add_rect(&enclosing);
    sketch.add_group(&l0);
    l1.set_style(Style::new("black", "1px"));
    sketch.add_group(&l1);

    let (mut l3, l4) = l2.split_shape(Circle::new(
        sketch.centroid(),
        enclosing.scale(0.98).min_len() / 2.,
    ));
    l3.set_style(Style::new("red", "1px"));

    l3.elements = l3
        .elements
        .iter()
        .filter(|e| rng.gen::<f64>() < 0.25)
        .map(|e| e.clone())
        .collect::<Vec<Shape>>();
    sketch.add_group(&l3);
    render_svg(&sketch, "/Users/are/Desktop/prova.svg")?;
    Ok(())
}
