use anyhow::Result;
use noise::NoiseFn;
use noise::Perlin;
use plt::layout::Orientation::Landscape;
use plt::layout::PageLayout;
use plt::render::render_svg;
use plt::shapes::LineStr;
use plt::traits::Scale;
use plt::vec2::Vec2;
use plt::Group;
use plt::Sketch;
use plt::Style;

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::axidraw_minikit(Landscape));
    let perlin = Perlin::new(37);
    let rows = 70;
    let cols = 120;
    let noise_ratio_x: f64 = 0.02;
    let noise_ratio_y: f64 = 0.02;
    let noise_mul: f64 = 10.;
    let mut line_set1 = Group::new();
    let mut line_set2 = Group::new();
    let mut line_set3 = Group::new();
    let mut line_set4 = Group::new();
    let mut line_set5 = Group::new();

    let col_size = sketch.width() / cols as f64;
    let row_size = sketch.height() / rows as f64;

    for y in -5..rows+5 {
        let mut point_set1 = vec![];
        let mut point_set2 = vec![];
        let mut point_set3 = vec![];
        let mut point_set4 = vec![];
        for x in -5..cols+5 {
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
                y: y as f64 * row_size + noise_val1 as f64,
            });
            point_set2.push(Vec2 {
                x: x as f64 * col_size,
                y: y as f64 * row_size + noise_val2 as f64,
            });
            point_set3.push(Vec2 {
                x: x as f64 * col_size,
                y: y as f64 * row_size + noise_val3 as f64,
            });
            point_set4.push(Vec2 {
                x: x as f64 * col_size,
                y: y as f64 * row_size + noise_val4 as f64,
            });
        }
        let _ = &LineStr::new(point_set1).clip(&sketch.as_rect().scale_perc(0.985).to_linestr(true), false).iter().for_each(|l| {
            line_set1.add_lstr(&l);
        });
        let _ = &LineStr::new(point_set2).clip(&sketch.as_rect().scale_perc(0.985).to_linestr(true), false).iter().for_each(|l| {
            line_set2.add_lstr(&l);
        });
        let _ = &LineStr::new(point_set3).clip(&sketch.as_rect().scale_perc(0.985).to_linestr(true), false).iter().for_each(|l| {
            line_set3.add_lstr(&l);
        });
        let _ = &LineStr::new(point_set4).clip(&sketch.as_rect().scale_perc(0.985).to_linestr(true), false).iter().for_each(|l| {
            line_set4.add_lstr(&l);
        });
    }

    line_set5.add_rect(&sketch.as_rect().scale_unit(1.));
    line_set5.add_rect(&sketch.as_rect().scale_unit(3.));
    line_set5.add_rect(&sketch.as_rect().scale_unit(5.));
    line_set5.add_rect(&sketch.as_rect().scale_unit(7.));
    line_set5.add_rect(&sketch.as_rect().scale_unit(9.));

    sketch.add_group(&line_set1, &Style::new("blue", "0.4mm"));
    sketch.add_group(&line_set2, &Style::new("red", "0.4mm"));
    sketch.add_group(&line_set3, &Style::new("black", "0.4mm"));
    sketch.add_group(&line_set4, &Style::new("yellow", "0.4mm"));
    sketch.add_group(&line_set5, &Style::new("black", "0.5mm"));
    render_svg(&sketch, "./samples/noise_waves_1.svg")?;
    Ok(())
}
