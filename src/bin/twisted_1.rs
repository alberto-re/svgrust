use anyhow::Result;
use noise::NoiseFn;
use noise::Perlin;
use plt::prelude::*;
use rayon::prelude::*;

const SEED: u32 = 200;
const MARGIN_Y_MIN: f64 = 5.;
const MARGIN_X_MIN: f64 = 5.;
const NOISE_FACTOR: f64 = 0.008;
const SIDE: f64 = 10.;
const ROW_SPACING: f64 = 7.;
const X_STEP: f64 = 1.1;
const T_INC: f64 = 0.001;
const MAX_ROTATE_PER_SHIFT: f64 = TAU * 0.05;

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::axidraw_minikit(Portrait), Uom::Mm, Debug::On);
    let perlin = Perlin::new(SEED);

    sketch.group(0).set_pen(&Pen::pigma_micron_005_black());

    let mut rotation;
    let mut polygons: Vec<Polygon> = vec![];
    let mut t = 0.;
    let height_margin = sketch.height() - MARGIN_Y_MIN * 2.;
    let row_count = height_margin as usize / (SIDE + ROW_SPACING) as usize;
    let min_y = MARGIN_Y_MIN
        + (height_margin - row_count as f64 * SIDE - (row_count - 1) as f64 * ROW_SPACING) / 2.;

    for row_index in 0..row_count {
        let y = min_y + row_index as f64 * (SIDE + ROW_SPACING);
        let mut x = MARGIN_X_MIN;
        rotation = Angle::from_radians(perlin.get([x * NOISE_FACTOR, y * NOISE_FACTOR, t]) * TAU);
        while x < sketch.width() - MARGIN_X_MIN - SIDE * 0.9 {
            let polygon = Rect::new(Vec2::new(x, y), SIDE, SIDE).to_polygon();
            let noise_value = perlin.get([x * NOISE_FACTOR, y * NOISE_FACTOR, t]);
            rotation += Angle::from_radians(noise_value * MAX_ROTATE_PER_SHIFT);
            let polygon = polygon.rotate(rotation);
            polygons.push(polygon);
            x += X_STEP;
        }
        t += T_INC;
    }

    let clipped: Vec<LineString> = polygons
        .par_iter()
        .enumerate()
        .flat_map(|(i, p)| p.clip_many(&polygons[i + 1..], true))
        .collect();

    sketch.group(0).add_many(clipped);
    sketch.render().save_default()?;
    Ok(())
}
