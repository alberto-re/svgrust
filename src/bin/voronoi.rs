use anyhow::Result;
use plt::layout::Orientation::Landscape;
use plt::layout::PageLayout;
use plt::Sketch;
use plt::render_svg;
use plt::Layer;
use plt::Style;
use plt::shapes::Scalable;

fn main() -> Result<()> {
    let mut sketch = Sketch::new(PageLayout::axidraw_minikit(Landscape));
    let rect = sketch.as_rect().scaled(0.95);
    let mut layer = Layer::new().set_style(Style::new("black", "1px"));
    layer.add_rect(rect);
    sketch.add_layer(layer);
    render_svg(&sketch, "/Users/are/Desktop/shapes.svg")?;
    Ok(())
}
