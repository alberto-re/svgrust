use anyhow::Result;
use plt::layout::Orientation::Landscape;
use plt::layout::PageLayout;
use plt::render::render_svg;
use plt::shapes::Scale;
use plt::Layer;
use plt::Sketch;
use plt::Style;

fn main() -> Result<()> {
    let mut sketch = Sketch::new(PageLayout::axidraw_minikit(Landscape));
    let rect = sketch.as_rect().scale(0.95);
    let mut layer = Layer::new("1").set_style(Style::new("black", "10px"));
    layer.add_rect(&rect);
    sketch.add_layer(&layer);
    render_svg(&sketch, "/Users/are/Desktop/shapes.svg")?;
    Ok(())
}
