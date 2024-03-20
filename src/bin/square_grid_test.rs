use anyhow::Result;
use plt::layout::Orientation::Portrait;
use plt::layout::PageLayout;
use plt::shapes::{Circle, Rect};
use plt::traits::Scale;
use plt::Group;
use plt::Sketch;
use plt::Style;

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::axidraw_minikit(Portrait));
    let mut group = Group::new();

    let square_side = 5.;
    let grid_box = sketch.as_rect();
    let grid_box = grid_box.scale_perc(0.90);

    let grid = grid_box.into_square_grid(square_side);

    grid.iter_vertexes().for_each(|vertex| {
        group.add_rect(&Rect::new(*vertex, square_side, square_side));
    });

    grid.iter_centers().for_each(|center| {
        group.add_circle(&Circle::new(*center, 1.));
    });

    group.add_rect(&grid_box);

    sketch.add_group(&group, &Style::new("black", "1.0px"));

    sketch
        .debug()
        .render()
        .save_to_file("./samples/square_grid_test.svg")?;
    Ok(())
}
