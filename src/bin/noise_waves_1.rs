use anyhow::Result;
use geo::coord;
use noise::NoiseFn;
use noise::Perlin;
use plt::layout::Orientation::Landscape;
use plt::layout::PageLayout;
use plt::render::render_svg;
use plt::shapes::Rect;
use plt::Group;
use plt::Sketch;
use plt::Style;
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

fn main() -> Result<()> {
    let mut sketch = Sketch::new(PageLayout::axidraw_minikit(Landscape));
    let perlin = Perlin::new(5);
    let mut rng = StdRng::seed_from_u64(42);

    let dx: isize = 27;
    let dy: isize = 18;
    let spc: isize = 10;
    let minrad: usize = 1;
    let maxrad: usize = 8;
    let noiseratio: f64 = 0.008;

    let mut group1 = Group::new();
    let mut group2 = Group::new();
    (-dx..dx).for_each(|i| {
        (-dy..dy).for_each(|j| {
            let mut x = (spc * i) as f64;
            let mut y = (spc * j) as f64;
            x += sketch.centroid().x;
            y += sketch.centroid().y;
            let val = perlin.get([x * noiseratio, y * noiseratio]);
            let val = (val + 1.0) / 2.;
            let val = minrad + 1 + (val * (maxrad - minrad) as f64) as usize;
            if (rng.gen::<f64>() + 0.1) * sketch.as_rect().width < x {
                (minrad..val).for_each(|v| {
                    let rect = Rect::square_with_center(coord! {x: x, y: y}, v as f64);
                    group1.add_rect(&rect);
                })
            } else {
                (minrad..val).for_each(|v| {
                    let rect = Rect::square_with_center(coord! {x: x, y: y}, v as f64);
                    group2.add_rect(&rect);
                })
            }
        });
    });
    sketch.add_group(&group1, &Style::new("red", "1.5px"));
    sketch.add_group(&group2, &Style::new("blue", "1.5px"));

    render_svg(&sketch, "./samples/noise_waves_1.svg")?;
    Ok(())
}
