use anyhow::Result;
use noise::NoiseFn;
use noise::Perlin;
use plt::layout::Orientation::Portrait;
use plt::layout::PageLayout;
use plt::map_range;
use plt::shapes::LineString;
use plt::vec2::Vec2;
use plt::Group;
use plt::Sketch;
use plt::Style;
use rand::Rng;

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::axidraw_minikit(Portrait));
    let rows = 30;
    let ydist = sketch.as_rect().height / rows as f64;

    let mut rng = rand::thread_rng();
    let perlin = Perlin::new(19);

    let mut lines = Group::new();
    let mut lines2 = Group::new();

    for rowi in 0..rows {
        let mut x = rng.gen::<f64>() * 3.;
        let val = perlin.get([rowi as f64 * 0.25, (rows - rowi) as f64 * 0.005]);
        let val = map_range(val, -1., 1., 0., 1.);
        while x < sketch.as_rect().width - rng.gen::<f64>() {
            let y1 = ydist * rowi as f64;
            let y2 = y1 + ydist * 0.9;
            let x1 = x;
            let x2 = x;
            if rng.gen::<f64>() > 0.005 {
                lines.add_lstr(&LineString::new(vec![
                    Vec2 { x: x1, y: y1 },
                    Vec2 { x: x2, y: y2 },
                ]));
            } else {
                lines2.add_lstr(&LineString::new(vec![
                    Vec2 { x: x1, y: y1 },
                    Vec2 { x: x2, y: y2 },
                ]));
            }
            let val = f64::abs(sketch.as_rect().width * val - x);
            x += map_range(val.sqrt(), 0., sketch.as_rect().width / 3., 1.2, 20.);
        }
    }
    sketch.add_group(&lines, &Style::new("black", "0.45mm"));
    //sketch.add_group(&lines2, &Style::new("red", "0.45mm"));
    sketch
        .debug()
        .render()
        .save_to_file("./samples/straight_lines_2.svg")?;
    Ok(())
}
