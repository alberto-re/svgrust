use anyhow::Result;
use noise::NoiseFn;
use noise::Perlin;
use plt::prelude::*;

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::axidraw_minikit(Landscape), Uom::Px, Debug::Off);
    let seed = Seed::number(300);
    let perlin = Perlin::new(seed.into());

    let dx: isize = 35;
    let dy: isize = 42;
    let spcy: isize = 4;
    let spcx: isize = 8;
    let minrad: usize = 0;
    let maxrad: usize = 5;
    let noiseratio: f64 = 0.008;

    let mut lstrs: Vec<LineString> = vec![];
    let center = sketch.as_rect().centroid();
    (-dy..dy).for_each(|j| {
        let mut points = vec![];
        (-dx..dx).for_each(|i| {
            let mut x = (spcx * i) as f64;
            let mut y = (spcy * j) as f64;
            x += center.x;
            y += center.y;
            let val = perlin.get([x * noiseratio, y * noiseratio]);
            let val = (val + 1.0) / 2.;
            let val = minrad + 1 + (val * (maxrad - minrad) as f64) as usize;
            let val = val * 12;
            points.push(Vec2 {
                x,
                y: y + val as f64,
            });
        });
        let lstr = LineString::new(points);
        lstrs.push(lstr);
    });

    let mut lstrs2: Vec<LineString> = vec![];
    for i in 1..lstrs.len() - 1 {
        let mut segments: Vec<LineString> = vec![lstrs[i].clone()];
        for other in lstrs.iter().take(lstrs.len() - 1).skip(i + 1) {
            let mut points: Vec<Vec2> = other.points.clone();
            let first = points[0];
            let last = points[points.len() - 1];
            points.insert(
                0,
                Vec2 {
                    x: first.x,
                    y: -1000.,
                },
            );
            points.push(Vec2 {
                x: last.x,
                y: -1000.,
            });
            let other: LineString = LineString::new(points);
            let mut newsegments: Vec<LineString> = vec![];
            segments.iter().for_each(|s| {
                let subs = s.clip(&other, false);
                subs.iter().for_each(|subseg| {
                    newsegments.push(subseg.clone());
                });
            });
            segments.clone_from(&newsegments);
        }
        segments.iter().for_each(|s| lstrs2.push(s.clone()));
    }
    lstrs2.push(lstrs.last().unwrap().clone());

    let mut lstrs3: Vec<LineString> = vec![];
    lstrs2.iter().for_each(|l| {
        let s = l.clip(&sketch.as_rect().to_linestr(true), false);
        s.iter().for_each(|s1| lstrs3.push(s1.clone()));
    });

    sketch.group(0).add_many(lstrs3);
    sketch.group(0).set_style(Style::new("blue", "2.5px"));

    sketch.render().save_default()?;
    Ok(())
}
