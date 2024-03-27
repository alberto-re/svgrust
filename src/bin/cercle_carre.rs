use anyhow::Result;
use plt::prelude::*;

fn add_square_spiral_with_center(
    center: Vec2,
    side: f64,
    n: usize,
    start_angle: f64,
    group: &mut Group,
) {
    let rect = Rect::square_with_center(center, side);
    (0..n).for_each(|factor| {
        let scale = if factor > 0 {
            factor as f64 / n as f64
        } else {
            1.0
        };
        let rect = rect.scale_perc(scale);
        let rect = rect.to_linestr(true);
        let rect = rect.rotate(Angle::from_radians(start_angle + TAU * scale));
        let rect = rect.upsample(1);
        let rect = rect.chaikin(5, true);
        group.add_linestring(&rect);
    });
}

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::axidraw_minikit(Portrait), false);
    let mut group = Group::new();
    add_square_spiral_with_center(
        sketch.as_rect().centroid(),
        sketch.as_rect().scale_perc(0.80).min_len(),
        60,
        TAU * 0.25,
        &mut group,
    );
    sketch.add_group(&group, &Style::new("black", "1.0px"));
    sketch.render().save_default()?;
    Ok(())
}
