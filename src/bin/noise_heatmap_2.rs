use anyhow::Result;
use noise::NoiseFn;
use noise::Perlin;
use plt::layout::Orientation::Portrait;
use plt::layout::PageLayout;
use plt::shapes::Circle;
use plt::traits::Scale;
use plt::Group;
use plt::Sketch;
use plt::Style;

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::axidraw_minikit(Portrait));
    let mut group1 = Group::new();
    let mut group2 = Group::new();

    let perlin = Perlin::new(2);
    let square_side = 8.;
    let noise_ratio = 0.005;
    let radius_min = 0.0;
    let radius_interval = 7.;
    let grid_box = sketch.as_rect();
    let grid_box = grid_box.scale_perc(0.98);

    let grid = grid_box.into_square_grid(square_side);

    grid.iter_centers().for_each(|center| {
        let noise_val = perlin.get([center.x * noise_ratio, center.y * noise_ratio]);
        let noise_val = (noise_val + 1.) / 2.;
        group1.add_circle(&Circle::new(
            *center,
            radius_min + radius_interval * noise_val,
        ));
        let mut inverse_noise_val = 1. - (noise_val + 1.) / 2.;
        if f64::abs(noise_val - inverse_noise_val) < 0.1 {
            inverse_noise_val -= 0.2;
        }
        group2.add_circle(&Circle::new(
            *center,
            radius_min + radius_interval * inverse_noise_val,
        ));
    });

    sketch.add_group(&group1, &Style::new("blue", "1.0px"));
    sketch.add_group(&group2, &Style::new("violet", "1.0px"));

    sketch
        .debug()
        .render()
        .save_to_file("./samples/noise_heatmap_2.svg")?;
    Ok(())
}
