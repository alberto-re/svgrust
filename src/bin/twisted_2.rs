use anyhow::Result;
use noise::NoiseFn;
use noise::Perlin;
use plt::prelude::*;
use rayon::prelude::*;

const SEED: u32 = 120;
const NOISE_FACTOR: f64 = 0.0005;
const RADIUS: f64 = 240.;
const SIDE: f64 = 140.;

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::a4(Portrait), Uom::Px, Debug::Off);
    let perlin = Perlin::new(SEED);

    sketch.group(0).set_pen(&Pen::pigma_micron_005_black());

    let mut polygons: Vec<Polygon> = vec![];

    let mut theta = Angle::radians(0.);
    while theta < Angle::radians(TAU * 0.999) {
        let xy = Vec2::from_polar(theta, RADIUS) + sketch.center() - Vec2::new(65., 70.);
        let rotation = Angle::radians(perlin.get([xy.x * NOISE_FACTOR, xy.y * NOISE_FACTOR]) * TAU);
        let polygon = Rect::new(xy, SIDE, SIDE).to_polygon().rotate(rotation);
        polygons.push(polygon);
        theta += Angle::radians(TAU / 400.);
    }

    let clipped: Vec<LineString> = polygons
        .par_iter()
        .enumerate()
        .flat_map(|(i, p)| {
            let mut others = vec![];
            for i in (i + 1)..(i + 80) {
                others.push(polygons[i % polygons.len()].clone());
            }
            p.clip_many(&others, true)
        })
        .collect();

    sketch.group(0).add_many(clipped);
    sketch.render().save_default()?;
    Ok(())
}
