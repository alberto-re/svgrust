use anyhow::Result;
use noise::NoiseFn;
use noise::Perlin;
use plt::prelude::*;
use rayon::prelude::*;

const SEED: u32 = 34;
const N_LINES: usize = 120;
const N_COLS: usize = 150;

// Starting from there: https://openprocessing.org/sketch/897819

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::a4(Portrait), Uom::Mm, Debug::On);
    let pen1 = Pen::pigma_micron_05_black();
    let pen2 = Pen::pigma_micron_05_blue();
    let seed = Seed::number(SEED.into());
    let perlin = Perlin::new(seed.into());
    let noise_factor = 0.01;

    // How many strokes are needed in order to fill a row?
    let strokes_per_width = (sketch.width() / pen1.thickness).floor();
    println!("stokes_per_width: {}", strokes_per_width);
    let strokes_per_col = ((strokes_per_width / N_COLS as f64).floor()) as usize;
    println!("stokes_per_col: {}", strokes_per_col);
    let line_h = sketch.height() / N_LINES as f64 * 0.6;

    let mut lines: Vec<LineString> = vec![];

    for i in 0..N_LINES {
        let y = f64::lerp(&0., sketch.height(), i as f64 / N_LINES as f64);
        for j in 0..N_COLS {
            for k in 0..strokes_per_col {
                let x =
                    pen1.thickness * strokes_per_col as f64 * j as f64 + pen1.thickness * k as f64;
                let start = Vec2::new(x, y);
                let end = Vec2::new(x, y + line_h);
                let line = LineString::line(start, end);
                lines.push(line);
            }

            let x = pen1.thickness * strokes_per_col as f64 * j as f64;
            let noise_val = perlin.get([x * noise_factor, y * noise_factor, 0.]);
            let val = ((noise_val * 30.) / 4.).round() * 4.;
            for k in 0..strokes_per_col {
                let x =
                    pen1.thickness * strokes_per_col as f64 * j as f64 + pen1.thickness * k as f64;
                let start = Vec2::new(x, y + val);
                let end = Vec2::new(x, y + val + line_h);
                let line = LineString::line(start, end);
                lines.push(line);
            }
        }
    }

    println!("lines {}", lines.len());

    let lines_clipped: Vec<LineString> = lines
        .par_iter()
        .flat_map(|p| p.clip(&Rect::new(Vec2::new(15., 15.), sketch.width() * 0.7, sketch.height() - 30.).to_polygon(), false))
        .collect();
    sketch.group(0).add_many(lines_clipped);

    let lines_clipped: Vec<LineString> = lines
        .par_iter()
        .flat_map(|p| p.clip(&Rect::new(Vec2::new(sketch.width() * 0.7 + 16., 13.), sketch.width() * 0.05, sketch.height() - 26.).to_polygon(), false))
        .collect();
    sketch.group(1).add_many(lines_clipped);

    let lines_clipped: Vec<LineString> = lines
        .par_iter()
        .flat_map(|p| p.clip(&Rect::new(Vec2::new(sketch.width() * 0.75 + 17., 15.), sketch.width() * 0.10, sketch.height() - 30.).to_polygon(), false))
        .collect();
    sketch.group(0).add_many(lines_clipped);

    sketch.group(0).set_pen(pen1);
    sketch.group(1).set_pen(pen2);
    sketch.render().save_default()?;
    Ok(())
}
