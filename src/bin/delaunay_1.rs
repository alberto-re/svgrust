use anyhow::Result;
use delaunator::{triangulate, Point};
use fast_poisson::Poisson2D;
use noise::NoiseFn;
use noise::Perlin;
use plt::prelude::*;

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::a4(Portrait), false);
    let seed = Seed::from_number(4292437263);
    let perlin = Perlin::new(seed.clone().into());
    let noise_ratio: f64 = 0.005;
    let displacement = 30.;
    let distance = 8.;

    let mut group1 = Group::new();
    let mut group2 = Group::new();
    let mut group3 = Group::new();
    let mut group4 = Group::new();
    let mut group5 = Group::new();

    let samples = Poisson2D::new()
        .with_seed(seed.into())
        .with_dimensions([sketch.width(), sketch.height()], distance);

    // samples
    //     .iter()
    //     .for_each(|s| group.add(Circle::new(Vec2::from_slice(&s), 0.5)));

    let points1: Vec<Point> = samples.iter().map(|s| Point { x: s[0], y: s[1] }).collect();

    let triangulation = triangulate(&points1);

    let mut triangles: Vec<Polygon> = vec![];
    for i in (0..triangulation.triangles.len()).step_by(3) {
        let p1 = &points1[triangulation.triangles[i]];
        let p2 = &points1[triangulation.triangles[i + 1]];
        let p3 = &points1[triangulation.triangles[i + 2]];
        triangles.push(Polygon::triangle(
            Vec2::new(p1.x, p1.y),
            Vec2::new(p2.x, p2.y),
            Vec2::new(p3.x, p3.y),
        ));
    }

    triangles.iter().for_each(|t| {
        let segments = t.clip(&sketch.as_rect().scale_perc(0.9).to_polygon(true), false);
        for segment in segments {
            group1.add(segment);
        }
    });

    let points2: Vec<Point> = points1
        .iter()
        .map(|p| {
            let noise_val = perlin.get([p.x as f64 * noise_ratio, p.y as f64 * noise_ratio, 0.15]);
            let radians = map_range(noise_val, -1., 1., 0., TAU);
            let displacement = Vec2::from_angle_length(Angle::radians(radians), displacement);
            Point {
                x: p.x + displacement.x,
                y: p.y + displacement.y,
            }
        })
        .collect();

    let triangulation = triangulate(&points2);

    let mut triangles: Vec<Polygon> = vec![];
    for i in (0..triangulation.triangles.len()).step_by(3) {
        let p1 = &points2[triangulation.triangles[i]];
        let p2 = &points2[triangulation.triangles[i + 1]];
        let p3 = &points2[triangulation.triangles[i + 2]];
        triangles.push(Polygon::triangle(
            Vec2::new(p1.x, p1.y),
            Vec2::new(p2.x, p2.y),
            Vec2::new(p3.x, p3.y),
        ));
    }

    triangles.iter().for_each(|t| {
        let segments = t.clip(&sketch.as_rect().scale_perc(0.9).to_polygon(true), false);
        for segment in segments {
            group2.add(segment);
        }
    });

    let points3: Vec<Point> = points1
        .iter()
        .map(|p| {
            let noise_val = perlin.get([p.x as f64 * noise_ratio, p.y as f64 * noise_ratio, 0.3]);
            let radians = map_range(noise_val, -1., 1., 0., TAU);
            let displacement = Vec2::from_angle_length(Angle::radians(radians), displacement);
            Point {
                x: p.x + displacement.x,
                y: p.y + displacement.y,
            }
        })
        .collect();

    let triangulation = triangulate(&points3);

    let mut triangles: Vec<Polygon> = vec![];
    for i in (0..triangulation.triangles.len()).step_by(3) {
        let p1 = &points3[triangulation.triangles[i]];
        let p2 = &points3[triangulation.triangles[i + 1]];
        let p3 = &points3[triangulation.triangles[i + 2]];
        triangles.push(Polygon::triangle(
            Vec2::new(p1.x, p1.y),
            Vec2::new(p2.x, p2.y),
            Vec2::new(p3.x, p3.y),
        ));
    }

    triangles.iter().for_each(|t| {
        let segments = t.clip(&sketch.as_rect().scale_perc(0.9).to_polygon(true), false);
        for segment in segments {
            group3.add(segment);
        }
    });

    let points4: Vec<Point> = points1
        .iter()
        .map(|p| {
            let noise_val = perlin.get([p.x as f64 * noise_ratio, p.y as f64 * noise_ratio, 0.45]);
            let radians = map_range(noise_val, -1., 1., 0., TAU);
            let displacement = Vec2::from_angle_length(Angle::radians(radians), displacement);
            Point {
                x: p.x + displacement.x,
                y: p.y + displacement.y,
            }
        })
        .collect();

    let triangulation = triangulate(&points4);

    let mut triangles: Vec<Polygon> = vec![];
    for i in (0..triangulation.triangles.len()).step_by(3) {
        let p1 = &points4[triangulation.triangles[i]];
        let p2 = &points4[triangulation.triangles[i + 1]];
        let p3 = &points4[triangulation.triangles[i + 2]];
        triangles.push(Polygon::triangle(
            Vec2::new(p1.x, p1.y),
            Vec2::new(p2.x, p2.y),
            Vec2::new(p3.x, p3.y),
        ));
    }

    triangles.iter().for_each(|t| {
        let segments = t.clip(&sketch.as_rect().scale_perc(0.9).to_polygon(true), false);
        for segment in segments {
            group4.add(segment);
        }
    });

    let points5: Vec<Point> = points1
        .iter()
        .map(|p| {
            let noise_val = perlin.get([p.x as f64 * noise_ratio, p.y as f64 * noise_ratio, 0.6]);
            let radians = map_range(noise_val, -1., 1., 0., TAU);
            let displacement = Vec2::from_angle_length(Angle::radians(radians), displacement);
            Point {
                x: p.x + displacement.x,
                y: p.y + displacement.y,
            }
        })
        .collect();

    let triangulation = triangulate(&points5);

    let mut triangles: Vec<Polygon> = vec![];
    for i in (0..triangulation.triangles.len()).step_by(3) {
        let p1 = &points5[triangulation.triangles[i]];
        let p2 = &points5[triangulation.triangles[i + 1]];
        let p3 = &points5[triangulation.triangles[i + 2]];
        triangles.push(Polygon::triangle(
            Vec2::new(p1.x, p1.y),
            Vec2::new(p2.x, p2.y),
            Vec2::new(p3.x, p3.y),
        ));
    }

    triangles.iter().for_each(|t| {
        let segments = t.clip(&sketch.as_rect().scale_perc(0.9).to_polygon(true), false);
        for segment in segments {
            group5.add(segment);
        }
    });

    // sketch.add_group(&group1, &Style::new("rgba(191, 196, 26, 0.6)", "0.4mm"));
    sketch.add_group(&group2, &Style::new("rgba(38, 118, 150, 0.6)", "0.4mm"));
    sketch.add_group(&group3, &Style::new("rgba(173, 33, 28, 0.6)", "0.4mm"));
    sketch.add_group(&group4, &Style::new("rgba(181, 18, 148, 0.6)", "0.4mm"));
    sketch.add_group(&group5, &Style::new("rgba(0, 0, 0, 0.6)", "0.4mm"));
    sketch.render().save_default()?;
    Ok(())
}
