use anyhow::Result;
use plt::prelude::*;

struct ParametricCurve {
    l: f64,
    a: f64,
    b: f64,
    c: f64,
}

impl ParametricCurve {
    fn new(l: f64, a: f64, b: f64, c: f64) -> Self {
        Self { l, a, b, c }
    }

    fn points(&self, n: usize, interval: f64) -> Vec<Vec2> {
        let mut points: Vec<Vec2> = vec![];
        let mut t = 0f64;
        for _ in 0..n {
            let x =
                (f64::cos(self.a * t) + f64::cos(self.b * t) / 2.0 + f64::sin(self.c * t) / 3.0)
                    * self.l;
            let y =
                (f64::sin(self.a * t) + f64::sin(self.b * t) / 2.0 + f64::cos(self.c * t) / 3.0)
                    * self.l;
            points.push(Vec2 { x, y });
            t += interval;
        }
        points
    }
}

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::axidraw_minikit(Portrait), Uom::Px, Debug::Off);

    let curve = ParametricCurve::new(105., 1., 5., 7.);

    let points = curve
        .points(711, 1.)
        .iter()
        .map(|p| p.translate(sketch.as_rect().centroid()))
        .collect::<Vec<Vec2>>();

    sketch.group(0).add(&LineString::new(points));

    sketch.as_rect().grid(50, 30).iter().for_each(|rect| {
        sketch.group(1).add(Circle::new(rect.centroid(), 0.1));
    });

    sketch.group(0).set_style(Style::new("blue", "0.2mm"));
    sketch.group(1).set_style(Style::new("#093c80", "0.2mm"));

    sketch.render().save_default()?;
    Ok(())
}
