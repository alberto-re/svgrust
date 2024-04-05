use std::hash;

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

// How to draw a Penrose triangle with a compass:
// https://www.instructables.com/Draw-a-Penrose-Triangle/
fn penrose_triangle(center: Vec2, side: f64, width: f64) -> Vec<Polygon> {
    // let area = Circle::new(center, side).to_polygon(100);

    let p1 = center + Vec2::from_angle_length(Angle::degrees(-90.), side);
    let p2 = center + Vec2::from_angle_length(Angle::degrees(30.), side);
    let p3 = center + Vec2::from_angle_length(Angle::degrees(150.), side);
    // let triangle_ext = Polygon::triangle(p1, p2, p3);

    // let c1 = Circle::new(p1, width).to_polygon(50);
    // let c2 = Circle::new(p2, width).to_polygon(50);
    // let c3 = Circle::new(p3, width).to_polygon(50);

    let a = p1 + Vec2::from_angle_length(Angle::degrees(240.), width);
    let b = p1 + Vec2::from_angle_length(Angle::degrees(300.), width);
    let c = p1 + Vec2::from_angle_length(Angle::degrees(120.), width);
    let d = p1 + Vec2::from_angle_length(Angle::degrees(60.), width);

    let e = p2 + Vec2::from_angle_length(Angle::degrees(0.), width);
    let f = p2 + Vec2::from_angle_length(Angle::degrees(60.), width);
    let g = p2 + Vec2::from_angle_length(Angle::degrees(240.), width);
    let h = p2 + Vec2::from_angle_length(Angle::degrees(180.), width);

    let i = p3 + Vec2::from_angle_length(Angle::degrees(120.), width);
    let j = p3 + Vec2::from_angle_length(Angle::degrees(180.), width);
    let k = p3 + Vec2::from_angle_length(Angle::degrees(0.), width);
    let l = p3 + Vec2::from_angle_length(Angle::degrees(300.), width);

    // let adebug = Circle::new(a, 2.).to_polygon(8);
    // let bdebug = Circle::new(b, 2.).to_polygon(8);
    // let cdebug = Circle::new(c, 2.).to_polygon(8);
    // let ddebug = Circle::new(d, 2.).to_polygon(8);

    // let edebug = Circle::new(e, 2.).to_polygon(8);
    // let fdebug = Circle::new(f, 2.).to_polygon(8);
    // let gdebug = Circle::new(g, 2.).to_polygon(8);
    // let hdebug = Circle::new(h, 2.).to_polygon(8);

    // let idebug = Circle::new(i, 2.).to_polygon(8);
    // let jdebug = Circle::new(j, 2.).to_polygon(8);
    // let kdebug = Circle::new(k, 2.).to_polygon(8);
    // let ldebug = Circle::new(l, 2.).to_polygon(8);

    let lshape1 = Polygon::new(vec![c, i , f, e, k, d + Vec2::from_angle_length(Angle::degrees(120.), width), c]);
    let lshape2 = Polygon::new(vec![k, e, b, a, g, k + Vec2::from_angle_length(Angle::degrees(300.), width), k]);
    let lshape3 = Polygon::new(vec![i, j, a, g, g + Vec2::from_angle_length(Angle::degrees(180.), width), c, i]);
    vec![lshape1, lshape2, lshape3]
}

fn hatch_fill(polygon: &Polygon, d: usize) -> Vec<LineString> {
    let miny = polygon.points.iter().map(|p| p.y as usize).min().unwrap();
    let maxy = polygon.points.iter().map(|p| p.y as usize).max().unwrap();
    let minx = polygon.points.iter().map(|p| p.x as usize).min().unwrap();
    let maxx = polygon.points.iter().map(|p| p.x as usize).max().unwrap();
    let mut result = vec![];
    for y in (miny - 10..=maxy + 10).step_by(d) {
        let hatch_line = LineString::line(Vec2::new(minx as f64, y as f64), Vec2::new(maxx as f64, y as f64));
        let segments = hatch_line.clip(&polygon.to_linestring(), false);
        segments.iter().for_each(|s| {
            result.push(s.clone());
        });
    }
    result
}

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::axidraw_minikit(Portrait), true);
    let mut group = Group::new();
    let mut hatch = Group::new();
    let triangle = penrose_triangle(sketch.center(), 120., 30.);
    let fill = hatch_fill(&triangle[0], 1);
    fill.iter().for_each(|p| hatch.add(p.clone()));
    let fill = hatch_fill(&triangle[2], 3);
    fill.iter().for_each(|p| hatch.add(p.clone()));
    triangle.iter().for_each(|p| group.add(p.clone()));
    sketch.add_group(&group, &Style::new("black", "0.5mm"));
    sketch.add_group(&hatch, &Style::new("black", "0.2mm"));
    sketch.render().save_default()?;
    Ok(())
}
