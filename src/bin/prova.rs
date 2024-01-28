use anyhow::Result;
use plt::layout::Orientation::Portrait;
use plt::layout::PageLayout;
use plt::render::render_svg;
use plt::shapes::Circle;
use plt::shapes::Sample;
use plt::shapes::Scale;
use plt::Group;
use plt::Sketch;
use plt::Style;

fn main() -> Result<()> {
    let mut sketch = Sketch::new(PageLayout::axidraw_minikit(Portrait));
    let mut layer = Group::new();
    let bbox = sketch.as_rect().scale(0.8);
    bbox.sample_uniform(1000).iter().for_each(|xy| {
        layer.add_circle(&Circle::new(*xy, 3.));
    });
    sketch.add_group(&layer, &Style::new("black", "1px"));
    let mut layer = Group::new();
    let bbox = bbox.scale(0.5);
    bbox.sample_uniform(1000).iter().for_each(|xy| {
        layer.add_circle(&Circle::new(*xy, 3.));
    });
    sketch.add_group(&layer, &Style::new("black", "1px"));
    render_svg(&sketch, "/Users/are/Desktop/prova.svg")?;
    Ok(())
}
