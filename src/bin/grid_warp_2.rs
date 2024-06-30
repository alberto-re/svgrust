use anyhow::Result;
use plt::prelude::*;

const NP: f64 = 210.;
const N_ROWS: usize = 20;
const N_COLS: usize = 20;

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::a4(Portrait), Uom::Mm, Debug::On);

    let mut points: Vec<Vec2> = vec![];
    for i in 0..N_ROWS {
        for j in 0..N_COLS {
            let mut x = i as f64 / 10. - 1.;
            let mut y = j as f64 / 10. - 1.;
            let di = f64::sqrt(x * x + y * y);
            let mut an = if x != 0. {
                f64::atan(y / x)
            } else {
                PI / 2. * y.signum()
            };
            if x < 0. {
                an += PI;
            }
            if di < 1. {
                an += PI / 2. * (1. - di);
            }
            x = di * an.cos();
            y = di * an.sin();
            let x_ = NP / 2. * (1. + x);
            let y_ = NP / 2. * (1. + y);
            // sketch.group(0).add(Circle::new(Vec2::new(x_, y_), 1.));
            points.push(Vec2::new(x_, y_));
        }
        // sketch.group(0).add(LineString::new(points).chaikin(3, false));
    }

    for i in 0..N_ROWS {
        let mut pcols: Vec<Vec2> = vec![];
        for j in 0..N_COLS {
            let index = i * N_COLS + j;
            pcols.push(points[index]);
        }
        sketch
            .group(0)
            .add(LineString::new(pcols).chaikin(3, false));
    }

    sketch.group(0).set_pen(Pen::pigma_micron_08_black());
    sketch.render().save_default()?;
    Ok(())
}
