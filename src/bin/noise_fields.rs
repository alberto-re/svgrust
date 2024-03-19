use std::f64::consts::TAU;

use anyhow::Result;
use plt::angle::Angle;
use plt::layout::Orientation::Landscape;
use plt::layout::PageLayout;
use plt::map_range;
use plt::render::render_svg;
use plt::shapes::Circle;
use plt::shapes::LineString;
use plt::traits::packing::CirclePacking;
use plt::traits::Centroid;
use plt::traits::Contains;
use plt::traits::Sample;
use plt::traits::Scale;
use plt::vec2::Vec2;
use plt::Group;
use plt::Shape;
use plt::Sketch;
use plt::Style;
use rand::rngs::StdRng;
use rand::SeedableRng;

fn focal_dist_angle(focal: Vec2, max_dist: f64, pos: Vec2) -> Angle {
    // The idea comes from:
    // https://damoonrashidi.me/articles/flow-field-methods#noise-function-alternatives
    let dx = pos.x - focal.x;
    let dy = pos.y - focal.y;
    let val = f64::sqrt(dx.powi(2) + dy.powi(2));
    Angle::from_radians(map_range(val, 0., max_dist, 0., TAU))
}

fn main() -> Result<()> {
    let mut layout = PageLayout::axidraw_minikit(Landscape);
    let layout = layout.set_style("background-color: white");
    let mut sketch = Sketch::new(layout);
    let mut field = Group::new();
    let mut trails = Group::new();
    let mut glyphs = Group::new();
    let mut frame = Group::new();

    let mut rng = StdRng::seed_from_u64(48);

    let square_side = 10.;
    let focal_max_dist = 310.;
    let bbox = sketch.as_rect().scale_perc(0.98);

    let grid = bbox.into_square_grid(square_side);

    grid.iter_centers().for_each(|center| {
        let angle = focal_dist_angle(bbox.centroid(), focal_max_dist, *center);
        let move_to = Vec2 {
            x: center.x + angle.cos() * 5.,
            y: center.y + angle.sin() * 5.,
        };
        let arrow = LineString::new(vec![*center, move_to]);
        field.add_lstr(&arrow);
    });

    bbox.sample_uniform(&mut rng, 400)
        .iter()
        .for_each(|center| {
            let mut pos = *center;
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
            let trail = LineString::new(trail_points);
            trails.add_lstr(&trail);
        });

    trails.elements = trails
        .linestrings()
        .iter()
        .filter(|t| t.points.len() > 12)
        .map(|l| plt::Shape::LineString(l.clone()))
        .collect();

    let mut circles: Vec<Circle> = vec![];
    let mut trails_to_prune: Vec<usize> = vec![];
    trails
        .linestrings()
        .iter()
        .enumerate()
        .for_each(|(index, trail)| {
            let radius = (trails.linestrings().len() - index) as f64 / 35_f64 + 1.;
            let dist = radius - 1.;
            let mut candidates = trail.pack_with_circles(radius, &mut circles, dist);
            if candidates.len() < 12 {
                trails_to_prune.push(index);
            } else {
                circles.append(&mut candidates);
            }
        });

    circles.iter().for_each(|circle| {
        glyphs.add_circle(circle);
        glyphs.add_circle(&circle.scale_perc(0.6));
        if circle.radius > 2. {
            glyphs.add_circle(&circle.scale_perc(0.3));
        }
    });

    trails.elements = trails
        .linestrings()
        .iter()
        .enumerate()
        .filter(|(index, _)| !trails_to_prune.contains(index))
        .map(|(_, linestring)| Shape::LineString(linestring.clone()))
        .collect::<Vec<Shape>>();

    frame.add_lstr(&bbox.scale_unit(50.).to_linestr(true));
    frame.add_lstr(&bbox.scale_unit(52.).to_linestr(true));
    frame.add_lstr(&bbox.scale_unit(54.).to_linestr(true));
    frame.add_lstr(&bbox.scale_unit(56.).to_linestr(true));
    frame.add_lstr(&bbox.scale_unit(58.).to_linestr(true));

    sketch.add_group(&trails, &Style::new("black", "1.5px"));
    sketch.add_group(&glyphs, &Style::new("black", "1.5px"));
    sketch.add_group(&frame, &Style::new("silver", "1.5px"));

    render_svg(&sketch, "./samples/noise_fields.svg")?;
    Ok(())
}
