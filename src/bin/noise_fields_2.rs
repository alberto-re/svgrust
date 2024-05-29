use anyhow::Result;
use plt::field::PerlinField;
use plt::field::Scalar2;
use plt::prelude::*;
use rand::rngs::StdRng;
use rand::SeedableRng;

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::a4(Portrait), Uom::Px, false);

    let mut trails = Group::new();

    let seed = Seed::from_number(37);
    let mut rng = StdRng::seed_from_u64(seed.clone().into());

    let bbox = Circle::new(sketch.center(), sketch.min_len() * 0.48);

    let noise_scale = 0.009;
    let vector_field = PerlinField::new(seed.into());

    bbox.sample_uniform(&mut rng, 4000)
        .iter()
        .for_each(|center| {
            let mut pos = *center;
            let mut trail_points: Vec<Vec2> = vec![pos];
            for _ in 0..1000 {
                let angle = vector_field.angle2(pos * noise_scale, PI / 4.);
                pos = pos + Vec2::from_angle_length(angle, 5.);
                if !bbox.contains(&pos) {
                    break;
                }
                let mut collision = false;
                for trail in trails.linestrings() {
                    let mut i = 0;
                    while i < trail.points.len() {
                        let point = trail.points[i];
                        let dist_sqrt = point.distance_squared(&pos);
                        if dist_sqrt < 9. {
                            collision = true;
                            break;
                        }
                        i += 1;
                    }
                    if collision {
                        break;
                    }
                }
                if collision {
                    break;
                }
                trail_points.push(pos);
            }
            let trail = LineString::new(trail_points);
            trails.add(&trail);
        });

    sketch.add_group(&trails, &Style::new("black", "1.0px"));

    sketch.render().save_default()?;
    Ok(())
}
