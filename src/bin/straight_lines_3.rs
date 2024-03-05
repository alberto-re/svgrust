use std::f64::consts::TAU;

use anyhow::Result;
use plt::layout::Orientation::Portrait;
use plt::layout::PageLayout;
use plt::render::render_svg;
use plt::shapes::Circle;
use plt::shapes::LineStr;
use plt::traits::Centroid;
use plt::vec2::Vec2;
use plt::Group;
use plt::Sketch;
use plt::Style;

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::axidraw_minikit(Portrait));
    let mut lines = Group::new();
    for x in (0..600).step_by(4) {
        lines.add_lstr(&LineStr::new(vec![
            Vec2 { x: x as f64, y: 0. },
            Vec2 {
                x: x as f64,
                y: 600.,
            },
        ]));
    }
    for x in (0..600).step_by(4) {
        lines.add_lstr(&LineStr::new(vec![
            Vec2 { x: x as f64, y: 0. },
            Vec2 {
                x: 0. + x as f64,
                y: 600.,
            },
        ]));
    }
    sketch.add_group(&lines, &Style::new("black", "0.45mm"));
    render_svg(&sketch, "./samples/straight_lines_3.svg")?;
    Ok(())
}
