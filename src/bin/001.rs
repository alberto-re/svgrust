use anyhow::Result;
use plt::layout::Orientation::Portrait;
use plt::layout::PageLayout;
use plt::shapes::Circle;
use plt::shapes::LineString;
use plt::traits::Centroid;
use plt::traits::Translate;
use plt::vec2::Vec2;
use plt::Group;
use plt::Sketch;
use plt::Style;

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
    let mut sketch = Sketch::new(&PageLayout::axidraw_minikit(Portrait));

    let curve = ParametricCurve::new(105., 1., 5., 7.);

    let points = curve
        .points(711, 1.)
        .iter()
        .map(|p| p.translate(sketch.as_rect().centroid()))
        .collect::<Vec<Vec2>>();

    let mut curve = Group::new();
    curve.add_lstr(&LineString::new(points));

    let mut plane = Group::new();
    sketch.as_rect().grid(50, 30).iter().for_each(|rect| {
        plane.add_circle(&Circle::new(rect.centroid(), 0.1));
    });

    sketch.add_group(&plane, &Style::new("black", "0.2mm"));
    sketch.add_group(&curve, &Style::new("#093c80", "0.2mm"));

    sketch.debug().render().save_to_file("./samples/001.svg")?;
    Ok(())
}
