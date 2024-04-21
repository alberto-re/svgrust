use anyhow::Result;
use plt::prelude::*;

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::a4(Portrait), Uom::Mm, false);
    let mut group = Group::new();

    let mut n: f64 = -10.0;
    let nend = 10.;
    let mend = 10.;
    let stepn = 0.6;
    let stepm = 0.05;
    
    while n <= nend {
        n += stepn;
        let mut points: Vec<Vec2> = vec![];
        let mut m: f64 = -10.0;
        while m <= mend {
            m += stepm;
            let mut x = n;
            let mut y = m;
            x = x + PI * f64::sin(y) * f64::cos(y);
            y = y + PI * f64::sin(x);
            x = x * 8.;
            y = y * 8.;
            points.push(Vec2::new(x, y) + Vec2::new(105., 150.));
        }
        group.add(LineString::new(points).chaikin(2, false));
    }

    sketch.add_group(&group, &Style::new("rgba(16, 56, 125, 0.7)", "0.5mm"));
    sketch.render().save_default()?;
    Ok(())
}
