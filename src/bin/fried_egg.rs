use anyhow::Result;
use noise::NoiseFn;
use noise::Perlin;
use plt::layout::Orientation::Landscape;
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
    let mut sketch = Sketch::new(&PageLayout::axidraw_minikit(Landscape), false);

    let curve1 = ParametricCurve::new(50., 1., 1., 1.);

    let perlin = Perlin::new(7);

    let mut points1 = curve1
        .points(1300, 0.005)
        .iter()
        .map(|p| p.translate(sketch.as_rect().centroid() + Vec2 { x: -20., y: 0. }))
        .collect::<Vec<Vec2>>();

    let mut curve1 = Group::new();
    let mut curve2 = Group::new();
    let mut curve3 = Group::new();

    for z in 0..40 {
        points1 = points1
            .clone()
            .iter()
            .map(|p| {
                let angle = sketch.center().angle(*p);
                let noise_val = perlin.get([p.x * 0.006, p.y * 0.006, z as f64 * 0.05]);
                let noise_val = (noise_val + 1.) / 2.;
                *p + Vec2::from_angle_length(angle, 6. * noise_val)
            })
            .collect::<Vec<Vec2>>();
        curve1.add_lstr(&LineString::new(points1.clone()));
        curve2.add_lstr(&LineString::new(
            points1
                .iter()
                .map(|p| *p + Vec2 { x: 3., y: 1. })
                .collect::<Vec<_>>()
                .clone(),
        ));

        if z == 0 {
            for factor in 1..16 {
                let points2 = points1
                    .clone()
                    .iter()
                    .map(|p| {
                        let angle = sketch.center().angle(*p);
                        *p - Vec2::from_angle_length(angle, 6. * factor as f64)
                    })
                    .collect::<Vec<Vec2>>();
                curve3.add_lstr(&LineString::new(points2.clone()));
            }
        }
    }

    let mut plane = Group::new();
    sketch.as_rect().grid(60, 40).iter().for_each(|rect| {
        plane.add_circle(&Circle::new(rect.centroid(), 0.1));
    });

    sketch.add_group(&plane, &Style::new("black", "0.2mm"));
    sketch.add_group(&curve1, &Style::new("#093c80", "0.3mm"));
    sketch.add_group(&curve2, &Style::new("#a32784", "0.3mm"));
    sketch.add_group(&curve3, &Style::new("#c9a71e", "2mm"));
    sketch.render().save_default()?;
    Ok(())
}
