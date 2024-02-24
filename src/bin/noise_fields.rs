use std::f64::consts::TAU;

use anyhow::Result;
use geo::{coord, Coord};
use noise::NoiseFn;
use noise::Perlin;
use plt::layout::Orientation::Portrait;
use plt::layout::PageLayout;
use plt::render::render_svg;
use plt::shapes::LineStr;
use plt::traits::Sample;
use plt::traits::Scale;
use plt::Group;
use plt::Sketch;
use plt::Style;

fn map_range(n: f64, start: f64, stop: f64, start_to: f64, stop_to: f64) -> f64 {
    (n - start) / (stop - start) * (stop_to - start_to) + start_to
}

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::axidraw_minikit(Portrait));
    let mut group1 = Group::new();
    let mut group2 = Group::new();

    let perlin = Perlin::new(4);
    let square_side = 10.;
    let noise_ratio = 0.004;
    let grid_box = sketch.as_rect();
    let grid_box = grid_box.scale(0.98);

    let grid = grid_box.into_square_grid(square_side);

    grid.iter_centers().for_each(|center| {
        let noise_val = perlin.get([center.x * noise_ratio, center.y * noise_ratio]);
        let angle = map_range(noise_val, -1., 1., 0., TAU);
        let move_to = coord! {x: center.x + angle.cos() * 5., y: center.y + angle.sin() * 5.};
        let arrow = LineStr::new(vec![*center, move_to]);
        group1.add_lstr(&arrow);
    });

    let bbox = sketch.as_rect().scale(0.98);
    let origin_bbox = bbox.scale(0.3);

    for mut pos in origin_bbox.sample_uniform(20) {
        let mut trail_points: Vec<Coord> = vec![pos.clone()];
        for _ in 0..100 {
            let noise_val = perlin.get([pos.x * noise_ratio, pos.y * noise_ratio]);
            let noise_angle = map_range(noise_val, -1., 1., 0., TAU);
            pos = coord! {x: pos.x + noise_angle.cos() * 10., y: pos.y + noise_angle.sin() * 10.};
            if pos.x > bbox.xy.x + bbox.width
                || pos.x < bbox.xy.x
                || pos.y < bbox.xy.y
                || pos.y > bbox.xy.y + bbox.height
            {
                break;
            }
            trail_points.push(pos.clone());
        }
        let trail = LineStr::new(trail_points);
        group2.add_lstr(&trail);
    }

    sketch.add_group(&group1, &Style::new("black", "1.0px"));
    sketch.add_group(&group2, &Style::new("red", "1.5px"));

    render_svg(&sketch, "./samples/noise_fields.svg")?;
    Ok(())
}
