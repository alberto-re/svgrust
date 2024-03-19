use std::f64::consts::TAU;

use anyhow::Result;
use noise::NoiseFn;
use noise::Perlin;
use plt::layout::Orientation::Portrait;
use plt::layout::PageLayout;
use plt::render::render_svg;
use plt::shapes::LineString;
use plt::traits::Scale;
use plt::vec2::Vec2;
use plt::Group;
use plt::Sketch;
use plt::Style;

fn perlin_angle(perlin: &Perlin, smooth: f64, pos: Vec2) -> f64 {
    let val = perlin.get([pos.x * smooth, pos.y * smooth]);
    let start_from = -1.;
    let stop_from = 1.;
    let start_to = 0.;
    let stop_to = TAU;
    (val - start_from) / (stop_from - start_from) * (stop_to - start_to) + start_to
}

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::axidraw_minikit(Portrait));
    let mut group1 = Group::new();

    let perlin = Perlin::new(4);
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
        group1.add_lstr(&arrow);
    });

    sketch.add_group(&group1, &Style::new("black", "1.0px"));

    render_svg(&sketch, "./samples/noise_field_test.svg")?;
    Ok(())
}
