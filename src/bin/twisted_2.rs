use anyhow::Result;
use noise::NoiseFn;
use noise::Perlin;
use plt::prelude::*;
use rayon::prelude::*;

const SEED: u32 = 200;
const NOISE_FACTOR: f64 = 0.002;
const SIDE: f64 = 55.;

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::axidraw_minikit(Portrait), Uom::Px, Debug::On);
    let seed = Seed::number(SEED);
    let perlin = Perlin::new(seed.into());

    let mut polygons: Vec<Polygon> = vec![];
    let mut rotation;

    let radius = 150.;
    let mut theta = Angle::radians(0.);
    // TODO: Angle - implement +=
    // TODO: Angle - implement < or > comparaisons
    while theta.radians < TAU {
        let xy = Vec2::from_angle_length(theta, radius) + sketch.center() - Vec2::new(23., 0.);
        rotation = Angle::radians(perlin.get([xy.x * NOISE_FACTOR, xy.y * NOISE_FACTOR]) * TAU);
        let polygon = Rect::new(xy, SIDE, SIDE).to_polygon();
        let polygon = polygon.rotate(rotation);
        theta = theta + Angle::radians(TAU / 300.);
        polygons.push(polygon);
    }

    let clipped: Vec<LineString> = polygons
        .par_iter()
        .enumerate()
        .flat_map(|(i, p)| {
            let mut others = vec![];
            for i in (i + 1)..(i + 20) {
                others.push(polygons[i % polygons.len()].clone());
            }
            p.clip_many(&others, true)
        })
        .collect();

    sketch.group(0).add_many(clipped);
    sketch.group(0).set_style(Style::new("black", "0.2mm"));
    sketch.render().save_default()?;
    Ok(())
}
