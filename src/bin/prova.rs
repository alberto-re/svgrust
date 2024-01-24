use anyhow::Result;
use plt::layout::Orientation::Portrait;
use plt::layout::PageLayout;
use plt::render::render_svg;
use plt::shapes::Circle;
use plt::shapes::Sample;
use plt::shapes::Scale;
use plt::Layer;
use plt::Sketch;
use plt::Style;

fn main() -> Result<()> {
    let mut sketch = Sketch::new(PageLayout::axidraw_minikit(Portrait));
    let mut layer = Layer::default().set_style(Style::new("black", "2px"));
    let bbox = sketch.as_rect().scale(0.80);
    bbox.sample_uniform(1000).iter().for_each(|xy| {
        let c = Circle::new(*xy, 5.);
        layer.add_circle(c);
    });
    sketch.add_layer(layer);
    render_svg(&sketch, "/Users/are/Desktop/prova.svg")?;
    Ok(())
}
