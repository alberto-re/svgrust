use anyhow::Result;
use noise::NoiseFn;
use noise::Perlin;
use plt::prelude::*;
use plt::traits::Triangulate;

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::a4(Portrait), false);
    let seed = Seed::from_number(4292437263);
    let perlin = Perlin::new(seed.clone().into());
    let noise_ratio: f64 = 0.006;
    let displacement = 30.;
    let distance = 10.;

    let mut group1 = Group::new();
    let mut group2 = Group::new();

    let points = sketch.as_rect().sample_poisson2d(distance, seed.into());

    let points = points.iter().map(|p| {
        let noise_val = perlin.get([p.x as f64 * noise_ratio, p.y as f64 * noise_ratio, 1.0]);
        let radians = map_range(noise_val, -1., 1., 0., TAU);
        *p + Vec2::from_angle_length(Angle::radians(radians), displacement)
    }).collect::<Vec<Vec2>>();

    let triangles = points.triangulate();
    let bbox = sketch.as_rect().scale_perc(0.9).to_polygon(true);

    group1.add_many(triangles.clip(&bbox));

    // let points2: Vec<Point> = points1
    //     .iter()
    //     .map(|p| {
    //         let noise_val = perlin.get([
    //             p.x as f64 * noise_ratio,
    //             p.x as f64 * noise_ratio,
    //             p.x as f64 * noise_ratio,
    //         ]);
    //         let radians = map_range(noise_val, -1., 1., 0., TAU);
    //         let displacement = Vec2::from_angle_length(Angle::radians(radians), displacement);
    //         Point {
    //             x: p.x + displacement.x,
    //             y: p.y + displacement.y,
    //         }
    //     })
    //     .collect();

    // let triangulation = triangulate(&points2);

    // let mut triangles: Vec<Polygon> = vec![];
    // for i in (0..triangulation.triangles.len()).step_by(3) {
    //     let p1 = &points2[triangulation.triangles[i]];
    //     let p2 = &points2[triangulation.triangles[i + 1]];
    //     let p3 = &points2[triangulation.triangles[i + 2]];
    //     triangles.push(Polygon::triangle(
    //         Vec2::new(p1.x, p1.y),
    //         Vec2::new(p2.x, p2.y),
    //         Vec2::new(p3.x, p3.y),
    //     ));
    // }

    // triangles.iter().for_each(|t| {
    //     let segments = t.clip(&sketch.as_rect().scale_perc(0.9).to_polygon(true), false);
    //     for segment in segments {
    //         group2.add(segment);
    //     }
    // });

    sketch.add_group(&group1, &Style::new("black", "0.4mm"));
    sketch.add_group(&group2, &Style::new("black", "0.4mm"));
    sketch.render().save_default()?;
    Ok(())
}
