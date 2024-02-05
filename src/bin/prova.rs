use anyhow::Result;
use geo::coord;
use plt::layout::Orientation::Portrait;
use plt::layout::PageLayout;
use plt::render::render_svg;
use plt::shapes::Rect;
use plt::Group;
use plt::Sketch;
use plt::Style;

fn main() -> Result<()> {
    let mut sketch = Sketch::new(PageLayout::axidraw_minikit(Portrait));
    let mut group = Group::new();
    let rect = Rect::new(coord! {x: 80., y: 80.}, 200., 200.);
    group.add_lstr(&rect.to_linestr());
    sketch.add_group(&group, &Style::new("black", "1px"));
    render_svg(&sketch, "./samples/prova.svg")?;
    Ok(())
}
