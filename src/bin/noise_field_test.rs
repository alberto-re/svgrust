use anyhow::Result;
use noise::NoiseFn;
use noise::Perlin;
use plt::prelude::*;

fn perlin_angle(perlin: &Perlin, smooth: f64, pos: Vec2) -> f64 {
    let val = perlin.get([pos.x * smooth, pos.y * smooth]);
    let start_from = -1.;
    let stop_from = 1.;
    let start_to = 0.;
    let stop_to = TAU;
    (val - start_from) / (stop_from - start_from) * (stop_to - start_to) + start_to
}

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::axidraw_minikit(Portrait), false);
    let mut group1 = Group::new();

    let perlin = Perlin::new(12);
    let square_side = 10.;
    let smooth = 0.004;
    let bbox = sketch.as_rect().scale_perc(0.98);

    let grid = bbox.into_square_grid(square_side);

    grid.iter_centers().for_each(|center| {
        let angle = perlin_angle(&perlin, smooth, *center);
        let move_to = Vec2 {
            x: center.x + angle.cos() * square_side,
            y: center.y + angle.sin() * square_side,
        };
        let arrow = LineString::new(vec![*center, move_to]);
        group1.add_linestring(&arrow);
    });

    sketch.add_group(&group1, &Style::new("black", "1.0px"));

    sketch.render().save_default()?;
    Ok(())
}
