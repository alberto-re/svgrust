use std::f64::consts::TAU;

use anyhow::Result;
use plt::layout::Orientation::Landscape;
use plt::layout::PageLayout;
use plt::map_range;
use plt::render::render_svg;
use plt::shapes::LineStr;
use plt::traits::Centroid;
use plt::traits::Contains;
use plt::traits::Sample;
use plt::traits::Scale;
use plt::vec2::Vec2;
use plt::Group;
use plt::Sketch;
use plt::Style;
use rand::rngs::StdRng;
use rand::SeedableRng;

fn focal_dist_angle(focal: Vec2, max_dist: f64, pos: Vec2) -> f64 {
    // The idea comes from:
    // https://damoonrashidi.me/articles/flow-field-methods#noise-function-alternatives
    let dx = pos.x - focal.x;
    let dy = pos.y - focal.y;
    let val = f64::sqrt(dx.powi(2) + dy.powi(2));
    map_range(val, 0., max_dist, 0., TAU)
}

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::axidraw_minikit(Landscape));
    let mut field = Group::new();
    let mut trails = Group::new();

    let mut rng = StdRng::seed_from_u64(48);

    let square_side = 10.;
    let focal_max_dist = 300.;
    let bbox = sketch.as_rect().scale(0.98);

    let grid = bbox.into_square_grid(square_side);

    grid.iter_centers().for_each(|center| {
        let angle = focal_dist_angle(bbox.centroid(), focal_max_dist, *center);
        let move_to = Vec2 {
            x: center.x + angle.cos() * 5.,
            y: center.y + angle.sin() * 5.,
        };
        let arrow = LineStr::new(vec![*center, move_to]);
        field.add_lstr(&arrow);
    });

    bbox.sample_uniform(&mut rng, 100)
        .iter()
        .for_each(|center| {
            let mut pos = center.clone();
            let mut trail_points: Vec<Vec2> = vec![pos];
            for _ in 0..200 {
                let angle = focal_dist_angle(bbox.centroid(), focal_max_dist, pos);
                pos = pos + Vec2::from_angle_length(angle, 5.);
                if !bbox.contains(&pos) {
                    break;
                }
                if trails
                    .linestrings()
                    .iter()
                    .flat_map(|l| l.clone().points)
                    .any(|p| p.euclidean_distance(&pos) < 5.)
                {
                    break;
                }
                trail_points.push(pos);
            }
            let trail = LineStr::new(trail_points);
            trails.add_lstr(&trail);
        });

    trails.elements = trails
        .linestrings()
        .iter()
        .filter(|t| t.points.len() > 8)
        .map(|l| plt::Shape::LineString(l.clone()))
        .collect();

    sketch.add_group(&field, &Style::new("black", "1.0px"));
    sketch.add_group(&trails, &Style::new("black", "1.0px"));

    render_svg(&sketch, "./samples/noise_fields.svg")?;
    Ok(())
}
