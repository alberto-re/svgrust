use anyhow::Result;
use noise::NoiseFn;
use noise::Perlin;
use plt::prelude::*;

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::axidraw_minikit(Landscape), Uom::Px, false);
    let seed = Seed::from_number(37);
    let perlin = Perlin::new(seed.into());
    let rows = 70;
    let cols = 120;
    let noise_ratio_x: f64 = 0.02;
    let noise_ratio_y: f64 = 0.02;
    let noise_mul: f64 = 15.;
    let mut line_set1 = Group::new();
    let mut line_set2 = Group::new();
    let mut line_set3 = Group::new();
    let mut line_set4 = Group::new();
    let mut line_set5 = Group::new();

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
                line_set1.add(l);
            });
        let _ = &LineString::new(point_set2)
            .clip(&sketch.as_rect().scale_perc(0.985).to_linestr(true), false)
            .iter()
            .for_each(|l| {
                line_set2.add(l);
            });
        let _ = &LineString::new(point_set3)
            .clip(&sketch.as_rect().scale_perc(0.985).to_linestr(true), false)
            .iter()
            .for_each(|l| {
                line_set3.add(l);
            });
        let _ = &LineString::new(point_set4)
            .clip(&sketch.as_rect().scale_perc(0.985).to_linestr(true), false)
            .iter()
            .for_each(|l| {
                line_set4.add(l);
            });
    }

    line_set5.add(sketch.as_rect().scale_perc(1.));
    line_set5.add(sketch.as_rect().scale_perc(3.));
    line_set5.add(sketch.as_rect().scale_perc(5.));
    line_set5.add(sketch.as_rect().scale_perc(7.));
    line_set5.add(sketch.as_rect().scale_perc(9.));

    sketch.add_group(&line_set1, &Style::new("blue", "0.4mm"));
    sketch.add_group(&line_set2, &Style::new("red", "0.4mm"));
    sketch.add_group(&line_set3, &Style::new("black", "0.4mm"));
    sketch.add_group(&line_set4, &Style::new("yellow", "0.4mm"));
    sketch.add_group(&line_set5, &Style::new("black", "0.5mm"));
    sketch.render().save_default()?;
    Ok(())
}
