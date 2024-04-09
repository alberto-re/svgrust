use anyhow::Result;
use plt::prelude::*;

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::axidraw_minikit(Portrait), true);

    let polygon = Rect::square_with_center(sketch.center(), 200.).to_polygon(true);

    let mut group = Group::new();

    group.add(polygon.clone());

    let scaled = &polygon.scale_dist(-2.);

    group.add(scaled.clone());

    sketch.add_group(&group, &Style::new("black", "0.5mm"));
    sketch.render().save_default()?;
    Ok(())
}
