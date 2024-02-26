use std::f64::consts::PI;
use std::f64::consts::TAU;

use anyhow::Result;
use noise::NoiseFn;
use noise::Perlin;
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

fn perlin_angle(perlin: &Perlin, smooth: f64, pos: Vec2) -> f64 {
    let val = perlin.get([pos.x * smooth, pos.y * smooth]);
    map_range(val, -1., 1., 0., TAU)
}

fn focal_dist_angle(focal: Vec2, max_dist: f64, pos: Vec2) -> f64 {
    // The idea comes from:
    // https://damoonrashidi.me/articles/flow-field-methods#noise-function-alternatives
    let dx = pos.x - focal.x;
    let dy = pos.y - focal.y;
    let val = f64::sqrt(dx.powi(2) + dy.powi(2));
    map_range(val, 0., max_dist, 0., TAU)
}

fn focal_angle(focal: Vec2, pos: Vec2) -> f64 {
    // The idea comes from:
    // https://damoonrashidi.me/articles/flow-field-methods#noise-function-alternatives
    let dx = pos.x - focal.x;
    let dy = pos.y - focal.y;
    let val = f64::atan2(dy, dx);
    map_range(val, -PI, PI, 0., TAU)
}

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::axidraw_minikit(Landscape));
    let mut group1 = Group::new();
    let mut group2 = Group::new();

    let mut rng = StdRng::seed_from_u64(48);

    let perlin = Perlin::new(19);
    let square_side = 5.;
    let smooth = 0.003;
    let bbox = sketch.as_rect().scale(0.98);

    let grid = bbox.into_square_grid(square_side);

    grid.iter_centers().for_each(|center| {
        let angle1 = perlin_angle(&perlin, smooth, *center);
        let vec1 = Vec2 {
            x: angle1.cos() * 5.,
            y: angle1.sin() * 5.,
        };
        let angle2 = focal_angle(Vec2 { x: 0., y: 0. }, *center);
        // let angle2 = focal_angle(bbox.centroid(), pos);
        let vec2 = Vec2 {
            x: angle2.cos() * 5.,
            y: angle2.sin() * 5.,
        };
        let move_to = Vec2 {
            x: center.x + vec1.x + vec2.x,
            y: center.y + vec1.y + vec2.y,
        };
        let arrow = LineStr::new(vec![*center, move_to]);
        group1.add_lstr(&arrow);
    });

    let bbox = sketch.as_rect().scale(0.98);

    bbox.sample_uniform(&mut rng, 10).iter().for_each(|center| {
        // grid.iter_centers().step_by(10).for_each(|center| {
        let mut pos = center.clone();
        let mut trail_points: Vec<Vec2> = vec![pos];
        for _ in 0..100 {
            let angle = focal_tangle(bbox.centroid(), *center);
            let vec = Vec2 {
                x: angle.cos() * 5.,
                y: angle.sin() * 5.,
            };
            pos = Vec2 {
                x: pos.x + vec.x,
                y: pos.y + vec.y,
            };
            if !bbox.contains(&pos) {
                break;
            }
            trail_points.push(pos);
        }
        let trail = LineStr::new(trail_points);
        group2.add_lstr(&trail);
    });

    // sketch.add_group(&group1, &Style::new("black", "1.0px"));
    sketch.add_group(&group2, &Style::new("black", "1.5px"));

    render_svg(&sketch, "./samples/noise_fields_2.svg")?;
    Ok(())
}
