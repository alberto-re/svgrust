use anyhow::Result;
use geo::coord;
use noise::NoiseFn;
use noise::Perlin;
use plt::layout::Orientation::Landscape;
use plt::layout::PageLayout;
use plt::render::render_svg;
use plt::shapes::LineStr;
use plt::Group;
use plt::Sketch;
use plt::Style;

fn main() -> Result<()> {
    let mut sketch = Sketch::new(PageLayout::axidraw_minikit(Landscape));
    let perlin = Perlin::new(29);

    let dx: isize = 35;
    let dy: isize = 40;
    let spcy: isize = 4;
    let spcx: isize = 8;
    let minrad: usize = 0;
    let maxrad: usize = 5;
    let noiseratio: f64 = 0.005;

    let mut group = Group::new();
    (-dy..dy).for_each(|j| {
        let mut points = vec![];
        (-dx..dx).for_each(|i| {
            let mut x = (spcx * i) as f64;
            let mut y = (spcy * j) as f64;
            x += sketch.centroid().x;
            y += sketch.centroid().y;
            let val = perlin.get([x * noiseratio, y * noiseratio]);
            let val = (val + 1.0) / 2.;
            let val = minrad + 1 + (val * (maxrad - minrad) as f64) as usize;
            let val = val * 12;
            points.push(coord! { x: x, y: y + val as f64 });
        });
        let lstr = LineStr::new(points);
        group.add_lstr(&lstr);
        // let lstrs = lstr.clip(&sketch.as_rect().scale(0.98).to_linestr());
        // lstrs.iter().for_each(|l| {
        //     group.add_lstr(&l);
        // });
    });
    sketch.add_group(&group, &Style::new("black", "2.5px"));

    render_svg(&sketch, "./samples/noise_waves_3.svg")?;
    Ok(())
}
