use std::f64::consts::TAU;

use anyhow::Result;
use plt::layout::Orientation::Portrait;
use plt::layout::PageLayout;
use plt::render::render_svg;
use plt::shapes::Arc;
use plt::shapes::LineStr;
use plt::shapes::Rect;
use plt::traits::Centroid;
use plt::traits::Scale;
use plt::Group;
use plt::Sketch;
use plt::Style;
use rand::Rng;

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::axidraw_minikit(Portrait));
    let center = sketch.as_rect().centroid();
    let mut rng = rand::thread_rng();

    let step = 2.;
    let arcs = 150;

    let mut lines: Vec<LineStr> = vec![];
    (0..arcs).for_each(|i| {
        let start = rng.gen::<f64>() * TAU;
        let end = rng.gen::<f64>() * TAU;
        let arc = Arc::new(center, i as f64 * step, start, end);
        let arc_lstr = arc.to_linestr(100);
        lines.push(arc_lstr.clone());
    });

    let frame = Rect::square_with_center(
        sketch.as_rect().centroid(),
        sketch.as_rect().scale(0.97).min_len(),
    );

    let lines = lines
        .iter()
        .flat_map(|l| l.clip(&frame.to_linestr(true), false))
        .collect::<Vec<_>>();

    let mut group = Group::new();
    (0..3).for_each(|i| {
        group.add_rect(&frame.scale((100. + i as f64) / 100.));
    });
    group.add_lstrs(&lines);
    sketch.add_group(&group, &Style::new("black", "1px"));
    render_svg(&sketch, "./samples/arcs_spiral_1.svg")?;
    Ok(())
}
