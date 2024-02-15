use anyhow::Result;
use geo::coord;
use geo::Coord;
use plt::layout::Orientation::Portrait;
use plt::layout::PageLayout;
use plt::render::render_svg;
use plt::shapes::Circle;
use plt::shapes::LineStr;
use plt::Group;
use plt::Sketch;
use plt::Style;
use std::f64::consts::TAU;

fn main() -> Result<()> {
    let inner_radius: f64 = 110.;
    let other_radius = 75.;

    let mut sketch = Sketch::new(&PageLayout::axidraw_minikit(Portrait));
    let mut group = Group::new();

    let angles: Vec<f64> = vec![
        TAU * 0.05,
        TAU * 0.10,
        TAU * 0.16,
        TAU * 0.23,
        TAU * 0.31,
        TAU * 0.40, //
        TAU * 0.50, // middle left
        TAU * 0.60, //
        TAU * 0.69,
        TAU * 0.77,
        TAU * 0.84,
        TAU * 0.90,
        TAU * 0.95,
        TAU * 1.0,
    ];

    let mut in_points: Vec<Coord> = vec![];

    for angle in angles {
        let x = angle.cos() * inner_radius + sketch.centroid().x;
        let y = angle.sin() * inner_radius + sketch.centroid().y;
        in_points.push(coord! {x: x, y: y});
    }

    let mut circles: Vec<Vec<Circle>> = vec![];
    in_points.iter().for_each(|p| {
        circles.push(vec![
            Circle::new(*p, other_radius),
            Circle::new(*p, other_radius * 0.98),
            Circle::new(*p, other_radius * 0.96),
            Circle::new(*p, other_radius * 0.94),
            Circle::new(*p, other_radius * 0.88),
            Circle::new(*p, other_radius * 0.80),
            Circle::new(*p, other_radius * 0.70),
            Circle::new(*p, other_radius * 0.58),
            Circle::new(*p, other_radius * 0.44),
        ]);
    });

    let mut circles_lstr: Vec<Vec<LineStr>> = vec![];
    circles.iter().for_each(|circle| {
        circles_lstr.push(
            circle
                .iter()
                .map(|subcircle| subcircle.to_linestr(150))
                .collect(),
        )
    });

    let mut circles_lstr2: Vec<Vec<LineStr>> = vec![];
    for (i, circle) in circles_lstr.iter().enumerate() {
        let other_index = if i == 0 {
            circles_lstr.len() - 1
        } else {
            i - 1
        };
        let mut newlist: Vec<LineStr> = vec![];
        for subc in circle {
            let parts: Vec<LineStr> = vec![subc.clone()];
            let mut partsnew: Vec<LineStr> = vec![];
            for part in &parts {
                part.clip(&circles_lstr[other_index][0], true)
                    .iter()
                    .for_each(|p| partsnew.push(p.clone()));
            }
            partsnew.iter().for_each(|p| newlist.push(p.clone()));
        }
        circles_lstr2.push(newlist);
    }

    circles_lstr2
        .iter()
        .flatten()
        .for_each(|c| group.add_lstr(c));

    sketch.add_group(&group, &Style::new("black", "1.5px"));

    render_svg(&sketch, "./samples/arcs_de_cercles.svg")?;
    Ok(())
}
