use std::f64::consts::TAU;

use anyhow::Result;
use geo::coord;
use geo::Coord;
use geo::EuclideanDistance;
use plt::layout::Orientation::Portrait;
use plt::layout::PageLayout;
use plt::render::render_svg;
use plt::shapes::Circle;
use plt::shapes::LineStr;
use plt::Group;
use plt::Sketch;
use plt::Style;

fn main() -> Result<()> {
    let inner_radius: f64 = 100.;
    let inner_points: usize = 13;
    let other_radius = 70.;
    let start_angle = TAU / 8.;

    let mut sketch = Sketch::new(PageLayout::axidraw_minikit(Portrait));
    let mut group = Group::new();

    let mut in_points: Vec<Coord> = vec![];
    for i in 1..inner_points + 1 {
        let angle = start_angle + TAU / inner_points as f64 * i as f64;
        let x = angle.cos() * inner_radius + sketch.centroid().x;
        let y = angle.sin() * inner_radius + sketch.centroid().y;
        in_points.push(coord! {x: x, y: y});
    }

    let mut circles0: Vec<Circle> = vec![];
    in_points.iter().for_each(|p| {
        let circle = Circle::new(*p, other_radius);
        circles0.push(circle);
    });

    let mut circles: Vec<LineStr> = vec![];
    circles0.iter().for_each(|c| circles.push(c.to_linestr()));

    let mut circles2: Vec<LineStr> = vec![];
    for (i, circle) in circles.iter().enumerate() {
        let other_index = if i == 0 { circles.len() - 1 } else { i - 1 };
        let parts: Vec<LineStr> = vec![circle.clone()];
        let mut partsnew: Vec<LineStr> = vec![];
        for part in &parts {
            part.diff(&circles[other_index])
                .iter()
                .for_each(|p| partsnew.push(p.clone()));
        }
        partsnew.iter().for_each(|p| circles2.push(p.clone()));
    }

    circles2.iter().for_each(|c| group.add_lstr(&c));

    sketch.add_group(&group, &Style::new("black", "1.5px"));

    render_svg(&sketch, "./samples/arcs_de_cercles.svg")?;
    Ok(())
}
