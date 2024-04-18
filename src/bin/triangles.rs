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
    ]);
    let lshape2 = Polygon::new(vec![
        k,
        e,
        b,
        a,
        g,
        k + Vec2::from_angle_length(Angle::degrees(300.), width),
    ]);
    let mut lshape3 = Polygon::new(vec![
        i,
        j,
        a,
        g,
        g + Vec2::from_angle_length(Angle::degrees(180.), width),
        c,
    ]);
    lshape3.points.reverse();
    vec![lshape1, lshape2, lshape3]
}

fn hatch_fill(polygon: &Polygon) -> Vec<LineString> {
    let mut lines = vec![];
    let bbox = polygon.scale_dist(-2.0);
    lines.push(bbox.to_linestring());
    lines.push(polygon.scale_dist(-1.0).to_linestring());
    let mut miny = 1000000.;
    let mut maxy = 0.;
    let mut minx = 1000000.;
    let mut maxx = 0.;
    polygon.points.iter().for_each(|p| {
        if p.x > maxx {
            maxx = p.x;
        }
        if p.y > maxy {
            maxy = p.y;
        }
        if p.x < minx {
            minx = p.x;
        }
        if p.y < miny {
            miny = p.y;
        }
    });
    let mut cury = miny;
    while cury < maxy {
        let p1 = Vec2::new(minx, cury);
        let p2 = Vec2::new(maxx, cury);
        for segment in LineString::line(p1, p2).clip(&bbox, false) {
            lines.push(segment);
        }
        cury += 1.;
    }
    lines
}

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::axidraw_minikit(Portrait), Uom::Px, true);

    let mut group = Group::new();
    let mut hatch = Group::new();

    let triangle = penrose_triangle(sketch.center(), 140., 50.);
    group.add(triangle[0].clone());
    group.add(triangle[1].clone());
    group.add(triangle[2].clone());
    for line in &hatch_fill(&triangle[0]) {
        hatch.add(line);
    }

    sketch.add_group(&group, &Style::new("black", "0.5mm"));
    sketch.add_group(&hatch, &Style::new("black", "0.5mm"));
    sketch.render().save_default()?;
    Ok(())
}
