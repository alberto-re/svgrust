use anyhow::Result;
use plt::prelude::*;
use plt::vectorfield::{CurlNoise2dVectorField, VectorAt};

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::axidraw_minikit(Portrait), false);
    let mut group = Group::new();

    let seed = Seed::from_number(37);

    let bbox = sketch.as_rect().scale_perc(0.98);

    let grid = bbox.into_square_grid(5.);

    let vector_field = CurlNoise2dVectorField::new(0.001, 0.005, 0.005, seed.into());

    grid.iter_centers().for_each(|xy| {
        let force = vector_field.vector_at(*xy);
        let move_to = *xy + force.mul(10.);
        let arrow = LineString::new(vec![*xy, move_to]);
        group.add(&arrow);
    });

    sketch.add_group(&group, &Style::new("black", "0.5px"));

    sketch.render().save_default()?;
    Ok(())
}
