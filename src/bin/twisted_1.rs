use anyhow::Result;
use noise::NoiseFn;
use noise::Perlin;
use plt::prelude::*;
use rayon::prelude::*;

const SEED: u32 = 200;
const NOISE_FACTOR: f64 = 0.008;
const SIDE: f64 = 32.;
const Y_STEP: f64 = 50.;
const X_STEP: f64 = 3.;
const MIN_X: f64 = 7.;
const MIN_Y: f64 = 20.;
const T_INC: f64 = 0.001;
const MAX_ROTATE_PER_SHIFT: f64 = TAU * 0.05;

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::axidraw_minikit(Portrait), Uom::Px, Debug::On);
    let seed = Seed::number(SEED);
    let perlin = Perlin::new(seed.into());

    let mut rotation;
    let mut polygons: Vec<Polygon> = vec![];
    let mut y = MIN_Y;
    let mut x = MIN_X;
    let mut t = 0.;

    while y < sketch.height() - SIDE / 2. {
        rotation = Angle::radians(perlin.get([x * NOISE_FACTOR, y * NOISE_FACTOR, t]) * TAU);
        while x < sketch.width() - SIDE - X_STEP * 3. {
            let polygon = Rect::new(Vec2::new(x, y), SIDE, SIDE).to_polygon();
            let noise_value = perlin.get([x * NOISE_FACTOR, y * NOISE_FACTOR, t]);
            rotation = rotation + Angle::radians(noise_value * MAX_ROTATE_PER_SHIFT);
            let polygon = polygon.rotate(rotation);
            polygons.push(polygon);
            x += X_STEP;
        }
        y += Y_STEP;
        x = MIN_X;
        t += T_INC;
    }

    let clipped: Vec<LineString> = polygons.par_iter().enumerate().flat_map(|(i, p)| {
        p.clip_many(&polygons[i+1..], true)
    }).collect();

    sketch.group(0).add_many(clipped);
    sketch.group(0).set_style(Style::new("black", "0.2mm"));
    sketch.render().save_default()?;
    Ok(())
}
