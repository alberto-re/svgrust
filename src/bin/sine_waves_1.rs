use anyhow::Result;
use plt::prelude::*;

const PADDING_Y: f64 = 20.;
const PADDING_X: f64 = 30.;
const LINES: usize = 80;
const BASE_AMPLITUDE: f64 = 8.;
const BASE_FREQUENCY: f64 = 0.13;
const AMPLITUDE_VARIATION: f64 = 4.;
const FREQUENCY_VARIATION: f64 = 0.05;
const INFLUENCE_GROWTH: f64 = 0.010;

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::a4(Portrait), Uom::Mm, Debug::On);
    sketch.group(0).set_pen(&Pen::pigma_micron_05_black());

    for i in 0..LINES {
        let influence = 1. + i as f64 * INFLUENCE_GROWTH;
        let amplitude = BASE_AMPLITUDE + AMPLITUDE_VARIATION * influence;
        let frequency = BASE_FREQUENCY + FREQUENCY_VARIATION * influence;

        let mut points = vec![];

        let start_x = PADDING_X + (sketch.width() - PADDING_X * 2.) / LINES as f64 * i as f64;

        let mut y = PADDING_Y;
        while y < sketch.height() - PADDING_Y {
            let x = start_x + amplitude * f64::sin(frequency * y - PADDING_Y * 3.);
            // let x = start_x;
            points.push(Vec2::new(x, y));
            y += 1.;
        }

        sketch.group(0).add(LineString::new(points));
    }

    sketch.render().save_default()?;
    Ok(())
}
