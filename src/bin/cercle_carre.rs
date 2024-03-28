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
        let rect = rect.to_polygon(true);
        let rect = rect.rotate(Angle::from_radians(start_angle + TAU * scale));
        let rect = rect.upsample(1);
        let rect = rect.chaikin(5, true);
        group.add_polygon(&rect);
    });
}

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::axidraw_minikit(Portrait), false);
    let mut group1 = Group::new();
    let mut group2 = Group::new();
    let mut group3 = Group::new();
    add_square_spiral_with_center(
        sketch.as_rect().centroid(),
        sketch.as_rect().scale_perc(0.80).min_len(),
        60,
        TAU * 0.25,
        &mut group1,
    );
    add_square_spiral_with_center(
        sketch.as_rect().centroid(),
        sketch.as_rect().scale_perc(0.80).min_len(),
        60,
        TAU * 0.25 + 0.1,
        &mut group2,
    );
    add_square_spiral_with_center(
        sketch.as_rect().centroid(),
        sketch.as_rect().scale_perc(0.80).min_len(),
        60,
        TAU * 0.25 + 0.2,
        &mut group3,
    );
    sketch.add_group(&group1, &Style::new("orange", "1.0px"));
    sketch.add_group(&group2, &Style::new("blue", "1.0px"));
    sketch.add_group(&group3, &Style::new("black", "1.0px"));
    sketch.render().save_default()?;
    Ok(())
}
