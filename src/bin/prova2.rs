use anyhow::Result;
use geo::coord;
use geo::Coord;
use plt::layout::Orientation::Portrait;
use plt::layout::PageLayout;
use plt::render::render_svg;
use plt::shapes::LineString;
use plt::shapes::Rect;
use plt::shapes::Scale;
use plt::Layer;
use plt::Sketch;
use plt::Style;
use rand::Rng;

// https://openprocessing.org/sketch/2008342

fn main() -> Result<()> {
    let mut sketch = Sketch::new(PageLayout::axidraw_minikit(Portrait));
    let mut layer = Layer::new().set_style(Style::new("black", "1px"));
    let enclosing =
        Rect::square_with_center(sketch.centroid(), sketch.as_rect().min_len()).scale(0.80);
    layer.add_rect(&enclosing);
    let cols: u8 = 50;
    let rows: u8 = 50;
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
    sketch.add_layer(&layer);
    render_svg(&sketch, "/Users/are/Desktop/prova.svg")?;
    Ok(())
}
