use anyhow::Result;
use noise::NoiseFn;
use noise::Perlin;
use plt::prelude::*;

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::axidraw_minikit(Portrait), false);
    let seed = Seed::from_number(1944909223);
    let perlin = Perlin::new(seed.into());

    let noise_ratio: f64 = 0.01;
    let mut group = Group::new();
    let mut angle;
    let mut polygons: Vec<Polygon> = vec![];
    let side = 35.;

    let y_step = 60.;
    let x_step = 6.;

    let mut y = 20.;
    let mut x = 7.;
    let mut t = 0.;
    while y < sketch.height() - side / 2. {
        angle = perlin.get([x * noise_ratio, y * noise_ratio, t]) * TAU;
        while x < sketch.width() - side - x_step * 3. {
            let xy = Vec2::new(x, y);
            let polygon = &Rect::new(xy, side, side).to_polygon(true);
            let noise_value =
                perlin.get([x * noise_ratio, y * noise_ratio, t * 50000.]) * TAU / 20.;
            angle += noise_value;
            let polygon = polygon.rotate(Angle::radians(angle));
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

    clipped.iter().for_each(|p| group.add(p));

    sketch.add_group(&group, &Style::new("black", "0.2mm"));
    sketch.render().save_default()?;
    Ok(())
}
