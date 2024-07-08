use anyhow::Result;
use plt::prelude::*;
use rand::Rng;

// https://www.gorillasun.de/blog/a-guide-to-hexagonal-grids-in-p5js/

fn truchet(hexagon: &Polygon) -> Vec<LineString> {
    let mut linestrings = vec![];
    for i in (0..6).step_by(2) {
        let p1 = hexagon.points[i];
        let p2 = hexagon.points[(i + 1) % 6];
        let p3 = hexagon.points[(i + 2) % 6];
        let p12 = p1.lerp(p2, 0.5);
        let p23 = p2.lerp(p3, 0.5);
        let angle_start = p2.angle_between(p12);
        let mut angle_end = p2.angle_between(p23);
        if f64::abs(angle_start.to_degrees() - angle_end.to_degrees()) > 180. {
            angle_end = Angle::degrees(angle_end.to_degrees() - 360.);
        }
        for offset in (-8..=8).step_by(2) {
            let mut points: Vec<Vec2> = vec![];
            for step in 0..=20 {
                let angle_step = angle_start.lerp(angle_end, step as f64 / 20.);
                points.push(p2 + Vec2::from_polar(angle_step, p23.distance(p2) + offset as f64));
            }
            let line = LineString::new(points);
            linestrings.push(line);
        }
    }
    linestrings
}

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::a4(Portrait), Uom::Px, Debug::Off);

    let mut rng = rand::thread_rng();

    let hex_side = 50.;

    let mut count = 0;
    let mut y = 0.;

    let mut lines_tot: Vec<LineString> = vec![];

    while y < 1000. {
        let mut x = 0.;
        while x < 1000. {
            let mut curx = x;
            if count % 2 == 0 {
                curx = x + hex_side * 0.75;
            }
            let center = Vec2::new(curx, y);
            let hx = Hexagon::new(center, hex_side / 2., Angle::zero()).to_polygon();
            let lines = truchet(&hx);
            let mut rotate = Angle::degrees((count % 3) as f64 * 60.);
            if rng.gen::<f64>() < 0.4 {
                rotate = Angle::degrees(((count + 1) % 3) as f64 * 60.);
            }
            lines_tot.extend_from_slice(&lines.rotate(rotate));
            x += hex_side * 1.50;
        }
        y += hex_side / 2.3;
        count += 1;
    }

    let center = sketch.center();
    let min_len = sketch.min_len();
    sketch.group(3).add_many(vec![
        Hexagon::new(center, min_len * 0.45 + 1., Angle::degrees(30.)).to_polygon(),
        Hexagon::new(center, min_len * 0.45 + 2., Angle::degrees(30.)).to_polygon(),
    ]);

    let bbox1 = Polygon::new(vec![
        sketch.center(),
        sketch.center() + Vec2::from_polar(Angle::degrees(330.), sketch.min_len() * 0.45),
        sketch.center() + Vec2::from_polar(Angle::degrees(270.), sketch.min_len() * 0.45),
        sketch.center() + Vec2::from_polar(Angle::degrees(210.), sketch.min_len() * 0.45),
    ]);

    let bbox2 = Polygon::new(vec![
        sketch.center(),
        sketch.center() + Vec2::from_polar(Angle::degrees(330.), sketch.min_len() * 0.45),
        sketch.center() + Vec2::from_polar(Angle::degrees(30.), sketch.min_len() * 0.45),
        sketch.center() + Vec2::from_polar(Angle::degrees(90.), sketch.min_len() * 0.45),
    ]);

    let bbox3 = Polygon::new(vec![
        sketch.center(),
        sketch.center() + Vec2::from_polar(Angle::degrees(90.), sketch.min_len() * 0.45),
        sketch.center() + Vec2::from_polar(Angle::degrees(150.), sketch.min_len() * 0.45),
        sketch.center() + Vec2::from_polar(Angle::degrees(210.), sketch.min_len() * 0.45),
    ]);

    let mut segments: Vec<LineString> = vec![];
    lines_tot
        .iter()
        .for_each(|l| segments.extend_from_slice(&l.clip(&bbox1, false)));
    sketch.group(0).add_many(segments);

    let mut segments: Vec<LineString> = vec![];
    lines_tot
        .iter()
        .for_each(|l| segments.extend_from_slice(&l.clip(&bbox2, false)));
    sketch.group(1).add_many(segments);

    let mut segments: Vec<LineString> = vec![];
    lines_tot
        .iter()
        .for_each(|l| segments.extend_from_slice(&l.clip(&bbox3, false)));
    sketch.group(2).add_many(segments);

    // TODO: it is clear that scale_dist do not always infer what is the interior
    // and what is the exterior of a polygon... BUG!
    sketch.group(3).add(bbox1.scale_dist(0.));
    sketch.group(3).add(bbox1.scale_dist(-1.));
    sketch.group(3).add(bbox1.scale_dist(-2.));
    sketch.group(3).add(bbox2.scale_dist(0.));
    sketch.group(3).add(bbox2.scale_dist(1.));
    sketch.group(3).add(bbox2.scale_dist(2.));
    sketch.group(3).add(bbox3.scale_dist(0.));
    sketch.group(3).add(bbox3.scale_dist(1.));
    sketch.group(3).add(bbox3.scale_dist(2.));

    sketch.group(0).set_style(Style::new("black", "1.0px"));
    sketch.group(1).set_style(Style::new("black", "1.8px"));
    sketch.group(2).set_style(Style::new("black", "0.5px"));
    sketch.group(3).set_style(Style::new("black", "2.0px"));

    sketch.render().save_default()?;
    Ok(())
}
