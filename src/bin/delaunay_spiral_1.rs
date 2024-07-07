use anyhow::Result;
use plt::prelude::*;
use plt::traits::Triangulate;

const SEED: u64 = 123;
const POISSON_RADIUS: f64 = 2.;
const DISPLACEMENT: f64 = 2.0;
const FOCAL_MAX_DIST: f64 = 20.;

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::a4(Portrait), Uom::Mm, Debug::On);
    sketch.group(0).set_pen(&Pen::pigma_micron_05_black());

    let points = sketch
        .as_rect()
        .sample_poisson2d(POISSON_RADIUS, SEED)
        .iter()
        .map(|p| {
            let angle = focal_dist_angle(sketch.center(), FOCAL_MAX_DIST, *p);
            *p + Vec2::from_angle_length(angle, DISPLACEMENT)
        })
        .collect::<Vec<_>>();

    let triangles = points.triangulate();

    let bbox = sketch.as_rect().scale_perc(0.9).to_polygon();
    sketch.group(0).add_many(triangles.clip(&bbox));
    sketch.render().save_default()?;
    Ok(())
}

fn focal_dist_angle(focal: Vec2, max_dist: f64, pos: Vec2) -> Angle {
    let dx = pos.x - focal.x;
    let dy = pos.y - focal.y;
    let val = f64::sqrt(dx.powi(2) + dy.powi(2));
    Angle::radians(map_range(val, 0., max_dist, 0., TAU))
}
