use anyhow::Result;
use plt::prelude::*;
use plt::vectorfield::Spiral2dVectorField;
use plt::vectorfield::VectorAt;
use rand::rngs::StdRng;
use rand::SeedableRng;

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::axidraw_minikit(Portrait), false);
    let mut group2 = Group::new();
    let mut group3 = Group::new();
    let mut group4 = Group::new();

    let bbox = sketch.as_rect().scale_perc(0.99);

    let vector_field = Spiral2dVectorField::new(sketch.center());
    let vector_factor = 2.;

    let mut rng = StdRng::seed_from_u64(52);

    bbox.sample_uniform(&mut rng, 1000)
        .iter()
        .filter(|p| p.euclidean_distance(&sketch.center()) > 50.)
        .for_each(|center| {
            let mut pos = *center;
            let mut trail_points: Vec<Vec2> = vec![pos];
            for _ in 0..200 {
                let force = vector_field.vector_at(pos);
                pos = pos + force.mul(vector_factor);
                if !bbox.contains(&pos) {
                    break;
                }
                if group2
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
            group2.add_linestring(&trail);
        });

    let clip_box_out = sketch
        .as_rect()
        .scale_perc(0.7)
        .to_polygon(true)
        .rotate(Angle::from_degrees(7.));
    let mut outside: Vec<LineString> = vec![];
    group2.linestrings().iter().for_each(|linestring| {
        linestring
            .clip(&clip_box_out, true)
            .iter()
            .for_each(|segment| outside.push(segment.clone()));
    });
    group3.add_linestrings(&outside);

    let clip_box_in = sketch.as_rect().scale_perc(0.55).to_linestr(true);
    let mut inside: Vec<LineString> = vec![];
    group2.linestrings().iter().for_each(|linestring| {
        linestring
            .clip(&clip_box_in, false)
            .iter()
            .for_each(|segment| inside.push(segment.clone()));
    });

    let inside = inside.rotate(Angle::from_degrees(-7.));
    group4.add_linestrings(&inside);

    sketch.add_group(&group3, &Style::new("black", "1mm"));
    sketch.add_group(&group4, &Style::new("black", "1mm"));

    sketch.render().save_default()?;
    Ok(())
}
