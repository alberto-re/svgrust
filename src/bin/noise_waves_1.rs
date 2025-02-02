use anyhow::Result;
use noise::NoiseFn;
use noise::Perlin;
use plt::prelude::*;

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::axidraw_minikit(Landscape), Uom::Px, Debug::Off);
    let seed = Seed::number(37);
    let perlin = Perlin::new(seed.into());
    let rows = 70;
    let cols = 120;
    let noise_ratio_x: f64 = 0.02;
    let noise_ratio_y: f64 = 0.02;
    let noise_mul: f64 = 15.;

    let col_size = sketch.width() / cols as f64;
    let row_size = sketch.height() / rows as f64;

    for y in -5..rows + 5 {
        let mut point_set1 = vec![];
        let mut point_set2 = vec![];
        let mut point_set3 = vec![];
        let mut point_set4 = vec![];
        for x in -5..cols + 5 {
            let noise_val1 = perlin.get([x as f64 * noise_ratio_x, y as f64 * noise_ratio_y, 0.]);
            let noise_val1 = noise_val1 * noise_mul;
            let noise_val2 = perlin.get([x as f64 * noise_ratio_x, y as f64 * noise_ratio_y, 0.25]);
            let noise_val2 = noise_val2 * noise_mul;
            let noise_val3 = perlin.get([x as f64 * noise_ratio_x, y as f64 * noise_ratio_y, 0.5]);
            let noise_val3 = noise_val3 * noise_mul;
            let noise_val4 = perlin.get([x as f64 * noise_ratio_x, y as f64 * noise_ratio_y, 0.75]);
            let noise_val4 = noise_val4 * noise_mul;
            point_set1.push(Vec2 {
                x: x as f64 * col_size,
                y: y as f64 * row_size + noise_val1,
            });
            point_set2.push(Vec2 {
                x: x as f64 * col_size,
                y: y as f64 * row_size + noise_val2,
            });
            point_set3.push(Vec2 {
                x: x as f64 * col_size,
                y: y as f64 * row_size + noise_val3,
            });
            point_set4.push(Vec2 {
                x: x as f64 * col_size,
                y: y as f64 * row_size + noise_val4,
            });
        }
        let _ = &LineString::new(point_set1)
            .clip(&sketch.as_rect().scale_perc(0.985).to_linestr(true), false)
            .iter()
            .for_each(|l| {
                sketch.group(0).add(l);
            });
        let _ = &LineString::new(point_set2)
            .clip(&sketch.as_rect().scale_perc(0.985).to_linestr(true), false)
            .iter()
            .for_each(|l| {
                sketch.group(1).add(l);
            });
        let _ = &LineString::new(point_set3)
            .clip(&sketch.as_rect().scale_perc(0.985).to_linestr(true), false)
            .iter()
            .for_each(|l| {
                sketch.group(2).add(l);
            });
        let _ = &LineString::new(point_set4)
            .clip(&sketch.as_rect().scale_perc(0.985).to_linestr(true), false)
            .iter()
            .for_each(|l| {
                sketch.group(3).add(l);
            });
    }

    sketch.group(0).set_style(Style::new("blue", "0.4mm"));
    sketch.group(1).set_style(Style::new("red", "0.4mm"));
    sketch.group(2).set_style(Style::new("black", "0.4mm"));
    sketch.group(3).set_style(Style::new("yellow", "0.4mm"));
    sketch.render().save_default()?;
    Ok(())
}
