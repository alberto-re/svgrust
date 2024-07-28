use anyhow::Result;
use plt::prelude::*;

fn sdf_circle(p: Vec2, circle: &Circle) -> f64 {
    p.distance(circle.center) - circle.radius
}

fn main() -> Result<()> {
    let seed: u64 = 123;
    let poisson_radius: f64 = 5.0;
    let margin: f64 = 0.08;
    let side: f64 = 1.7;

    let mut sketch = Sketch::new(&PageLayout::a4(Portrait), Uom::Mm, Debug::On);
    sketch.group(0).set_pen(&Pen::uniball_signo_broad_white());
    sketch.group(1).set_pen(&Pen::uniball_signo_broad_gold());

    let distribution1 = sketch
        .as_rect()
        .scale_dist(sketch.min_len() * margin)
        .sample_poisson2d(poisson_radius, seed);

    let distribution2 = sketch
        .as_rect()
        .scale_dist(sketch.min_len() * margin)
        .sample_poisson2d(poisson_radius, seed * 147);

    let circle = Circle::new(sketch.center() + Vec2::new(35., 50.), 14.0);

    for point in &distribution1 {
        let dist = sdf_circle(*point, &circle);
        let dist_abs = dist.abs() / 3.;
        sketch.group(0).add(Rect::square_with_center(
            point.centroid(),
            (1.0 - dist_abs.sin()) * side,
        ));
    }

    for point in &distribution2 {
        let dist = sdf_circle(*point, &circle);
        let dist_abs = dist.abs() / 6.;
        sketch.group(1).add(Rect::square_with_center(
            point.centroid(),
            (1.0 - dist_abs.sin()) * side,
        ));
    }

    sketch.render().save_default()?;
    Ok(())
}
