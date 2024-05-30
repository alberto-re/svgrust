use anyhow::Result;
use plt::prelude::*;

// How to draw a Penrose triangle with a compass:
// https://www.instructables.com/Draw-a-Penrose-Triangle/
fn penrose_triangle(center: Vec2, side: f64, width: f64) -> Vec<Polygon> {
    let p1 = center + Vec2::from_angle_length(Angle::degrees(-90.), side);
    let p2 = center + Vec2::from_angle_length(Angle::degrees(30.), side);
    let p3 = center + Vec2::from_angle_length(Angle::degrees(150.), side);

    let a = p1 + Vec2::from_angle_length(Angle::degrees(240.), width);
    let b = p1 + Vec2::from_angle_length(Angle::degrees(300.), width);
    let c = p1 + Vec2::from_angle_length(Angle::degrees(120.), width);
    let d = p1 + Vec2::from_angle_length(Angle::degrees(60.), width);

    let e = p2 + Vec2::from_angle_length(Angle::degrees(0.), width);
    let f = p2 + Vec2::from_angle_length(Angle::degrees(60.), width);
    let g = p2 + Vec2::from_angle_length(Angle::degrees(240.), width);

    let i = p3 + Vec2::from_angle_length(Angle::degrees(120.), width);
    let j = p3 + Vec2::from_angle_length(Angle::degrees(180.), width);
    let k = p3 + Vec2::from_angle_length(Angle::degrees(0.), width);

    let lshape1 = Polygon::new(vec![
        c,
        i,
        f,
        e,
        k,
        d + Vec2::from_angle_length(Angle::degrees(120.), width),
        c,
    ]);
    let lshape2 = Polygon::new(vec![
        k,
        e,
        b,
        a,
        g,
        k + Vec2::from_angle_length(Angle::degrees(300.), width),
        k,
    ]);
    let lshape3 = Polygon::new(vec![
        i,
        j,
        a,
        g,
        g + Vec2::from_angle_length(Angle::degrees(180.), width),
        c,
        i,
    ]);
    vec![lshape1, lshape2, lshape3]
}

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::a4(Portrait), Uom::Px, Debug::Off);

    let mut group = Group::new();

    let triangle = penrose_triangle(sketch.center(), 300., 80.);

    triangle.iter().for_each(|p| group.add(p.clone()));

    sketch.add_group(&group, &Style::new("black", "0.5mm"));
    sketch.render().save_default()?;
    Ok(())
}
