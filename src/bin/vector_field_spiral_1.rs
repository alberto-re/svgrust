use anyhow::Result;
use plt::prelude::*;
use plt::vectorfield::Spiral2dVectorField;
use plt::vectorfield::VectorAt;
use rand::rngs::StdRng;
use rand::SeedableRng;

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::axidraw_minikit(Portrait), false);
    let mut group = Group::new();

    let vector_field = Spiral2dVectorField::new(sketch.center());
    let vector_factor = 2.;
    let mut rng = StdRng::seed_from_u64(52);

    let bbox = sketch.as_rect().scale_perc(0.99);

    let mut linestrings: Vec<LineString> = vec![];
    bbox.sample_uniform(&mut rng, 10000)
        .iter()
        .for_each(|center| {
            let mut pos = *center;
            let mut trail_points: Vec<Vec2> = vec![pos];
            for _ in 0..1000 {
                let force = vector_field.vector_at(pos);
                pos = pos + force.mul(vector_factor);
                if pos.euclidean_distance(&sketch.center()) < 4. {
                    break;
                }
                if !bbox.contains(&pos) {
                    break;
                }
                if linestrings
                    .iter()
                    .flat_map(|l| l.points.clone())
                    .any(|p| p.euclidean_distance(&pos) < 3.)
                {
                    break;
                }
                trail_points.push(pos);
            }
            if trail_points.len() > 20 {
                linestrings.push(LineString::new(trail_points));
            }
        });

    let clip_box = sketch
        .as_rect()
        .scale_perc(0.7)
        .to_polygon(true)
        .rotate(Angle::from_degrees(-15.));
    let mut outside: Vec<LineString> = vec![];
    linestrings.iter().for_each(|linestring| {
        linestring
            .clip(&clip_box, true)
            .iter()
            .for_each(|segment| outside.push(segment.clone()));
    });
    outside
        .iter()
        .filter(|l| l.points.len() > 5)
        .for_each(|linestring| {
            group.add_linestring(linestring);
        });

    let clip_box = sketch.as_rect().scale_perc(0.55).to_linestr(true);
    let mut inside: Vec<LineString> = vec![];
    linestrings.iter().for_each(|linestring| {
        linestring
            .clip(&clip_box, false)
            .iter()
            .for_each(|segment| inside.push(segment.clone()));
    });

    let inside = inside.rotate(Angle::from_degrees(-18.));
    inside
        .iter()
        .filter(|l| l.points.len() > 5)
        .for_each(|linestring| {
            group.add_linestring(linestring);
        });

    sketch.add_group(&group, &Style::new("black", "0.5mm"));

    sketch.render().save_default()?;
    Ok(())
}
