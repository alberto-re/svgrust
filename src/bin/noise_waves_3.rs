use anyhow::Result;
use geo::coord;
use geo::Coord;
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
    let mut sketch = Sketch::new(&PageLayout::axidraw_minikit(Landscape));
    let perlin = Perlin::new(29);

    let dx: isize = 35;
    let dy: isize = 42;
    let spcy: isize = 4;
    let spcx: isize = 8;
    let minrad: usize = 0;
    let maxrad: usize = 5;
    let noiseratio: f64 = 0.005;

    let mut group = Group::new();
    let mut lstrs: Vec<LineStr> = vec![];
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
        lstrs.push(lstr);
    });

    let mut lstrs2: Vec<LineStr> = vec![];
    for i in 1..lstrs.len() - 1 {
        let mut segments: Vec<LineStr> = vec![lstrs[i].clone()];
        for other in lstrs.iter().take(lstrs.len() - 1).skip(i + 1) {
            let mut points: Vec<Coord> = other.points.clone();
            let first = points[0];
            let last = points[points.len() - 1];
            points.insert(0, coord! { x: first.x, y: -1000.});
            points.push(coord! { x: last.x, y: -1000.});
            let other: LineStr = LineStr::new(points);
            let mut newsegments: Vec<LineStr> = vec![];
            segments.iter().for_each(|s| {
                let subs = s.clip(&other, false);
                subs.iter().for_each(|subseg| {
                    newsegments.push(subseg.clone());
                });
            });
            segments = newsegments.clone();
        }
        segments.iter().for_each(|s| lstrs2.push(s.clone()));
    }
    lstrs2.push(lstrs.last().unwrap().clone());

    let mut lstrs3: Vec<LineStr> = vec![];
    lstrs2.iter().for_each(|l| {
        let s = l.clip(&sketch.as_rect().to_linestr(true), false);
        s.iter().for_each(|s1| lstrs3.push(s1.clone()));
    });

    lstrs3.iter().for_each(|l| group.add_lstr(l));

    sketch.add_group(&group, &Style::new("black", "2.5px"));

    render_svg(&sketch, "./samples/noise_waves_3.svg")?;
    Ok(())
}
