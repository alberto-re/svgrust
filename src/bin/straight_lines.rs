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
    let rows = 40;
    let ydist = sketch.as_rect().height / rows as f64;

    let mut rng = rand::thread_rng();
    let perlin = Perlin::new(rng.gen::<u32>());

    let mut lines = Group::new();
    for rowi in 0..rows {
        //let maxdist = sketch.as_rect().width / 2.;
        //let maxdist = sketch.as_rect().width * rng.gen::<f64>();
        let val = perlin.get([rowi as f64 * 0.08, rowi as f64 * 0.08]);
        let val = map_range(val, -1., 1., 0., 1.);
        let maxdist = sketch.as_rect().width * val;
        for coli in (0..sketch.as_rect().width as usize).step_by(2) {
            let dist = f64::abs(maxdist - coli as f64);
            let random = rng.gen::<f64>();

            let p = random * (dist / maxdist);

            if p > 0.1 {
                continue;
            }
            let x = coli as f64;
            let y1 = ydist * rowi as f64;
            let y2 = y1 + ydist * 0.95;
            lines.add_lstr(&LineString::new(vec![Vec2 { x, y: y1 }, Vec2 { x, y: y2 }]));
        }
    }
    sketch.add_group(&lines, &Style::new("black", "1.5px"));
    sketch
        .debug()
        .render()
        .save_to_file("./samples/straight_lines.svg")?;
    Ok(())
}
