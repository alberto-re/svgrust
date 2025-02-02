use anyhow::Result;
use plt::prelude::*;

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::a4(Portrait), Uom::Mm, Debug::Off);

    let mut n: f64 = -10.0;
    let nend = 9.5;
    let mend = 10.;
    let stepn = 0.4;
    let stepm = 0.05;

    while n <= nend {
        n += stepn;
        let mut points: Vec<Vec2> = vec![];
        let mut m: f64 = -10.0;
        while m <= mend {
            m += stepm;
            let mut x = n;
            let mut y = m;
            x += PI * f64::sin(y) * f64::cos(y);
            y += PI * f64::sin(x);
            x *= 8.;
            y *= 8.;
            points.push(Vec2::new(x, y) + Vec2::new(105., 150.));
        }
        sketch
            .group(0)
            .add(LineString::new(points).chaikin(2, false));
    }

    sketch
        .group(0)
        .set_style(Style::new("rgba(16, 56, 1, 0.7)", "0.6mm"));
    sketch.render().save_default()?;
    Ok(())
}
