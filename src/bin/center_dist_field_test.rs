use std::f64::consts::TAU;

use anyhow::Result;
use plt::layout::Orientation::Portrait;
use plt::layout::PageLayout;
use plt::map_range;
use plt::shapes::LineString;
use plt::sketch::Sketch;
use plt::traits::Centroid;
use plt::traits::Scale;
use plt::vec2::Vec2;
use plt::Group;
use plt::Style;

fn focal_dist_angle(focal: Vec2, max_dist: f64, pos: Vec2) -> f64 {
    // The idea comes from:
    // https://damoonrashidi.me/articles/flow-field-methods#noise-function-alternatives
    let dx = pos.x - focal.x;
    let dy = pos.y - focal.y;
    let val = f64::sqrt(dx.powi(2) + dy.powi(2));
    map_range(val, 0., max_dist, 0., TAU)
}

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::axidraw_minikit(Portrait), false);
    let mut group1 = Group::new();

    let square_side = 10.;
    let bbox = sketch.as_rect().scale_perc(0.98);

    let grid = bbox.into_square_grid(square_side);

    grid.iter_centers().for_each(|center| {
        let angle = focal_dist_angle(bbox.centroid(), bbox.centroid().y / 2., *center);
        let move_to = Vec2 {
            x: center.x + angle.cos() * square_side,
            y: center.y + angle.sin() * square_side,
        };
        let arrow = LineString::new(vec![*center, move_to]);
        group1.add_linestring(&arrow);
    });

    sketch.add_group(&group1, &Style::new("black", "1.0px"));
    sketch.render().save_default()?;
    Ok(())
}
