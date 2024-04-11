use anyhow::Result;
use plt::prelude::*;
use plt::traits::Triangulate;

fn focal_dist_angle(focal: Vec2, max_dist: f64, pos: Vec2) -> Angle {
    let dx = pos.x - focal.x;
    let dy = pos.y - focal.y;
    let val = f64::sqrt(dx.powi(2) + dy.powi(2));
    Angle::radians(map_range(val, 0., max_dist, 0., TAU))
}

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::a4(Portrait), false);
    let seed = Seed::from_number(4292437263);
    let displacement = 20.;
    let radius = 10.;
    let focal_max_dist = 180.;

    let bbox = sketch.as_rect().scale_perc(0.9).to_polygon();

    let poisson_dist = sketch.as_rect().sample_poisson2d(radius, seed.into());

    let points = poisson_dist
        .iter()
        .map(|p| {
            let angle = focal_dist_angle(sketch.center(), focal_max_dist, *p);
            *p + Vec2::from_angle_length(angle, displacement)
        })
        .collect::<Vec<_>>();

    let triangles = points.triangulate();

    let mut group = Group::new();
    group.add_many(triangles.clip(&bbox));
    sketch.add_group(&group, &Style::new("black", "0.5mm"));
    sketch.render().save_default()?;
    Ok(())
}
