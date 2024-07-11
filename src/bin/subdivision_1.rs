use anyhow::Result;
use plt::prelude::*;

const SEED: u64 = 123;
const POISSON_RADIUS: f64 = 5.;
const RADIUS_TO_MINLEN_RATIO: f64 = 0.4;
const CIRCUMFERENCE_N_SAMPLE: usize = 40;

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::a4(Portrait), Uom::Mm, Debug::On);
    sketch.group(0).set_pen(&Pen::pigma_micron_05_black());
    sketch.group(1).set_pen(&Pen::pigma_micron_05_red());

    let circle = Circle::new(sketch.center(), sketch.min_len() * RADIUS_TO_MINLEN_RATIO);

    let mut points = sketch
        .as_rect()
        .sample_poisson2d(POISSON_RADIUS, SEED)
        .iter()
        .filter(|p| p.distance(circle.center) < circle.radius)
        .copied()
        .collect::<Vec<Vec2>>();

    points.append(&mut circle.to_polygon(CIRCUMFERENCE_N_SAMPLE).points);

    let triangles = points.triangulate();

    sketch.group(0).add_many(triangles);

    sketch.render().save_default()?;
    Ok(())
}
