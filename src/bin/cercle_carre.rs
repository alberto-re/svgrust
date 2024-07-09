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
        let rect = rect.to_polygon();
        let rect = rect.rotate(Angle::from_radians(start_angle + TAU * scale));
        let rect = rect.upsample(1);
        let rect = rect.chaikin(5, true);
        group.add(rect);
    });
}

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::axidraw_minikit(Portrait), Uom::Px, Debug::Off);
    add_square_spiral_with_center(
        sketch.as_rect().centroid(),
        sketch.as_rect().scale_perc(0.80).min_len(),
        60,
        TAU * 0.25,
        sketch.group(0),
    );
    add_square_spiral_with_center(
        sketch.as_rect().centroid(),
        sketch.as_rect().scale_perc(0.80).min_len(),
        60,
        TAU * 0.25 + 0.05,
        sketch.group(1),
    );
    add_square_spiral_with_center(
        sketch.as_rect().centroid(),
        sketch.as_rect().scale_perc(0.80).min_len(),
        60,
        TAU * 0.25 + 0.1,
        sketch.group(2),
    );
    add_square_spiral_with_center(
        sketch.as_rect().centroid(),
        sketch.as_rect().scale_perc(0.80).min_len(),
        60,
        TAU * 0.25 + 0.15,
        sketch.group(3),
    );
    add_square_spiral_with_center(
        sketch.as_rect().centroid(),
        sketch.as_rect().scale_perc(0.80).min_len(),
        60,
        TAU * 0.25 + 0.2,
        sketch.group(4),
    );
    sketch.group(0).set_style(Style::new("orange", "1.0px"));
    sketch.group(1).set_style(Style::new("azure", "1.0px"));
    sketch.group(2).set_style(Style::new("blue", "1.0px"));
    sketch.group(3).set_style(Style::new("violet", "1.0px"));
    sketch.group(4).set_style(Style::new("black", "1.0px"));
    sketch.render().save_default()?;
    Ok(())
}
