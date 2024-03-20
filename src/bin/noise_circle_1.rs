use std::f64::consts::TAU;

use anyhow::Result;
use noise::NoiseFn;
use noise::Perlin;
use plt::angle::Angle;
use plt::layout::Orientation::Portrait;
use plt::layout::PageLayout;
use plt::render::render_svg;
use plt::shapes::LineString;
use plt::shapes::Polygon;
use plt::shapes::Rect;
use plt::traits::Rotate;
use plt::traits::Scale;
use plt::vec2::Vec2;
use plt::Group;
use plt::Sketch;
use plt::Style;

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::axidraw_minikit(Portrait));
    let perlin = Perlin::new(109);
    let noise_ratio: f64 = 0.0005;
    let mut group = Group::new();
    let mut angle;
    let mut polygons: Vec<Polygon> = vec![];
    let side = 32.;

    let y_step = 50.;
    let x_step = 3.5;

    let mut y = 10.;
    let mut x = 5.;
    let mut t = 0.;
    while y < sketch.height() - side / 2. {
        angle = perlin.get([x * noise_ratio, y * noise_ratio, t]) * TAU;
        while x < sketch.width() - side - x_step * 4. {
            let xy = Vec2::new(x, y);
            let polygon = &Rect::new(xy, side, side).to_polygon(true);
            let noise_value = perlin.get([x * noise_ratio, y * noise_ratio, t]) * TAU / 20.;
            angle += noise_value;
            let polygon = polygon.rotate(Angle::from_radians(angle));
            polygons.push(polygon);
            x += x_step;
        }
        y += y_step;
        x = 10.;
        t += 0.001;
    }

    let mut clipped: Vec<LineString> = vec![];

    for (i, polygon) in polygons.iter().enumerate() {
        let segments = polygon.clip_many(&polygons[i + 1..], true);
        segments.iter().for_each(|s| clipped.push(s.clone()));
    }

    clipped.iter().for_each(|p| group.add_lstr(&p.clone()));

    // group.add_rect(&sketch.as_rect().scale_perc(0.99));

    sketch.add_group(&group, &Style::new("black", "0.2mm"));
    render_svg(&sketch, "./samples/noise_circle_1.svg")?;
    Ok(())
}
