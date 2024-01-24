use anyhow::Result;
use plt::layout::Orientation::Landscape;
use plt::layout::PageLayout;
use plt::render::render_svg;
use plt::shapes::LineString;
use plt::Layer;
use plt::Sketch;
use plt::Style;
use plt::shapes::Rectangle;
use geo::coord;
use plt::shapes::Clippable;

fn main() -> Result<()> {
    let mut sketch = Sketch::new(PageLayout::axidraw_minikit(Landscape));
    let mut layer = Layer::default().set_style(Style::new("black", "2px"));
    let linestr =
        LineString::from_tuples(vec![(50., 55.), (150., 155.), (200., 180.), (300., 300.), (320., 320.)]);
    let bbox = Rectangle::new(coord! {x: 100., y: 100.}, 100., 100.); 
    for p in &linestr.points {
        println!("{:?}", p);
    }
    let linestr = linestr.clipped(&bbox);
    println!("");
    println!("");
    println!("");
    for p in &linestr.points {
        println!("{:?}", p);
    }
    layer.add_linestr(linestr);
    layer.add_rect(bbox);
    sketch.add_layer(layer);
    render_svg(&sketch, "/Users/are/Desktop/linestring.svg")?;
    Ok(())
}
