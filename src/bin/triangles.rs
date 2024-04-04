use anyhow::Result;
use plt::prelude::*;

fn penrose_side(center: Vec2, side: f64, w: f64) -> Polygon {
    let x1 = -side * f64::sin(PI / 6.);
    let y1 = side * f64::sin(PI / 6.) / f64::sqrt(3.);
    let x2 = x1 - w;
    let y2 = y1;
    let x3 = x2 + (side + 3.0 * w) * f64::sin(PI / 6.);
    let y3 = y2 - (side + 3.0 * w) * f64::cos(PI / 6.);
    let x4 = x3 + (side + 4.0 * w) * f64::sin(PI / 6.);
    let y4 = y3 + (side + 4.0 * w) * f64::cos(PI / 6.);
    let x5 = x4 - w * f64::sin(PI / 6.);
    let y5 = y4 + w * f64::cos(PI / 6.);
    let x6 = x5 - (side + 3.0 * w) * f64::sin(PI / 6.);
    let y6 = y5 - (side + 3.0 * w) * f64::cos(PI / 6.);
    let points = vec![
        center + Vec2::new(x1, y1),
        center + Vec2::new(x2, y2),
        center + Vec2::new(x3, y3),
        center + Vec2::new(x4, y4),
        center + Vec2::new(x5, y5),
        center + Vec2::new(x6, y6),
    ];
    Polygon::new(points)
}

fn penrose_triangle(center: Vec2, side: f64, w: f64) -> MultiPolygon {
    let side1 = penrose_side(center + Vec2::new(40., 40.), side, w)
        .rotate(Angle::from_radians(TAU / 3. * 2.));
    let side2 = penrose_side(center + Vec2::new(-16., -20.), side, w)
        .rotate(Angle::from_radians(TAU / 3. * 4.));
    let side3 = penrose_side(center + Vec2::new(64., -38.), side, w)
        .rotate(Angle::from_radians(TAU / 3. * 6.));
    MultiPolygon::new(vec![side1, side2, side3])
}

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::axidraw_minikit(Portrait), true);
    let mut group = Group::new();
    group.add(penrose_triangle(sketch.center(), 80., 40.));
    sketch.add_group(&group, &Style::new("black", "0.2mm"));
    sketch.render().save_default()?;
    Ok(())
}
