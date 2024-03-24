use anyhow::Result;
use noise::NoiseFn;
use noise::Perlin;
use plt::layout::Orientation::Portrait;
use plt::layout::PageLayout;
use plt::shapes::Circle;
use plt::sketch::Sketch;
use plt::traits::Scale;
use plt::Group;
use plt::Style;

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::axidraw_minikit(Portrait), false);
    let mut group = Group::new();

    let perlin = Perlin::new(2);
    let square_side = 5.;
    let noise_ratio = 0.005;
    let radius_min = 0.0;
    let radius_interval = 6.;
    let grid_box = sketch.as_rect();
    let grid_box = grid_box.scale_perc(0.98);

    let grid = grid_box.into_square_grid(square_side);

    grid.iter_centers().for_each(|center| {
        let noise_val = perlin.get([center.x * noise_ratio, center.y * noise_ratio]);
        let noise_val = (noise_val + 1.) / 2.;
        group.add_circle(&Circle::new(
            *center,
            radius_min + radius_interval * noise_val,
        ));
    });

    sketch.add_group(&group, &Style::new("black", "1.0px"));

    sketch.render().save_default()?;
    Ok(())
}
