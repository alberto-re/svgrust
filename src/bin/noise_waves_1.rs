use anyhow::Result;
use geo::coord;
use noise::Cylinders;
use noise::NoiseFn;
use plt::layout::Orientation::Landscape;
use plt::layout::PageLayout;
use plt::render::render_svg;
use plt::shapes::Circle;
use plt::Group;
use plt::Sketch;
use plt::Style;

fn main() -> Result<()> {
    let mut sketch = Sketch::new(PageLayout::axidraw_minikit(Landscape));
    let mut group = Group::new();
    let cyl = Cylinders::new();

    (3..70).for_each(|i| {
        (4..70).for_each(|j| {
            let x = (8 * i) as f64;
            let y = (5 * j) as f64;
            let val = cyl.get([x * 0.02, y * 0.02]);
            let circle = Circle::new(coord! {x: x, y: y}, val * 20.);
            group.add_circle(&circle);
        })
    });

    sketch.add_group(&group, &Style::new("black", "1px"));
    render_svg(&sketch, "/Users/are/Desktop/noise_waves_1.svg")?;
    Ok(())
}
