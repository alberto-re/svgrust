use anyhow::Result;
use plt::layout::Orientation::Landscape;
use plt::layout::PageLayout;
use plt::render::render_svg;
use plt::shapes::LineString;
use plt::Group;
use plt::Sketch;
use plt::Style;

fn main() -> Result<()> {
    let mut sketch = Sketch::new(PageLayout::axidraw_minikit(Landscape));
    let mut layer = Group::new();
    let linestr = LineString::from_tuples(vec![
        (50., 55.),
        (150., 155.),
        (200., 180.),
        (300., 300.),
        (320., 320.),
    ]);
    layer.add_lstr(&linestr);
    sketch.add_group(&layer, &Style::new("black", "1px"));
    render_svg(&sketch, "/Users/are/Desktop/linestring.svg")?;
    Ok(())
}
