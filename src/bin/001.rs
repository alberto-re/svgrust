use anyhow::Result;
use plt::layout::Orientation::Portrait;
use plt::layout::PageLayout;
use plt::render::render_svg;
use plt::shapes::Circle;
use plt::shapes::LineStr;
use plt::traits::Centroid;
use plt::traits::Translate;
use plt::vec2::Vec2;
use plt::Group;
use plt::Sketch;
use plt::Style;

fn sample_point(t: f64, l: f64, a: f64, b: f64, c: f64) -> Vec2 {
    let x = (f64::cos(a * t) + f64::cos(b * t) / 2.0 + f64::sin(c * t) / 3.0) * l;
    let y = (f64::sin(a * t) + f64::sin(b * t) / 2.0 + f64::cos(c * t) / 3.0) * l;
    Vec2 { x, y }
}

fn get_curve_points(iterations: usize, step: f64, l: f64, a: f64, b: f64, c: f64) -> Vec<Vec2> {
    let mut points: Vec<Vec2> = vec![];
    let mut t = 0f64;
    for _ in 0..iterations {
        let next_point = sample_point(t as f64, l, a, b, c);
        points.push(next_point);
        t += step;
    }
    points
}

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::axidraw_minikit(Portrait));

    let points = get_curve_points(711, 1., 105., 1., 5., 7.)
        .iter()
        .map(|p| p.translate(sketch.as_rect().centroid()))
        .collect::<Vec<Vec2>>();

    let mut curve = Group::new();
    curve.add_lstr(&LineStr::new(points));

    let mut plane = Group::new();
    sketch.as_rect().grid(50, 30).iter().for_each(|rect| {
        plane.add_circle(&Circle::new(rect.centroid(), 0.1));
    });
    sketch.add_group(&plane, &Style::new("black", "0.2mm"));
    sketch.add_group(&curve, &Style::new("#093c80", "0.2mm"));

    render_svg(&sketch, "./samples/001.svg")?;
    Ok(())
}
