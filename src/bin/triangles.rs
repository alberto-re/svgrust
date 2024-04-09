use delaunator::{triangulate, Point};

use anyhow::Result;
use plt::prelude::*;
use rand::rngs::StdRng;
use rand::SeedableRng;

// How to draw a Penrose triangle with a compass:
// https://www.instructables.com/Draw-a-Penrose-Triangle/
fn penrose_triangle(center: Vec2, side: f64, width: f64) -> Vec<Polygon> {
    let p1 = center + Vec2::from_angle_length(Angle::degrees(-90.), side);
    let p2 = center + Vec2::from_angle_length(Angle::degrees(30.), side);
    let p3 = center + Vec2::from_angle_length(Angle::degrees(150.), side);

    let a = p1 + Vec2::from_angle_length(Angle::degrees(240.), width);
    let b = p1 + Vec2::from_angle_length(Angle::degrees(300.), width);
    let c = p1 + Vec2::from_angle_length(Angle::degrees(120.), width);
    let d = p1 + Vec2::from_angle_length(Angle::degrees(60.), width);

    let e = p2 + Vec2::from_angle_length(Angle::degrees(0.), width);
    let f = p2 + Vec2::from_angle_length(Angle::degrees(60.), width);
    let g = p2 + Vec2::from_angle_length(Angle::degrees(240.), width);

    let i = p3 + Vec2::from_angle_length(Angle::degrees(120.), width);
    let j = p3 + Vec2::from_angle_length(Angle::degrees(180.), width);
    let k = p3 + Vec2::from_angle_length(Angle::degrees(0.), width);

    let lshape1 = Polygon::new(vec![
        c,
        i,
        f,
        e,
        k,
        d + Vec2::from_angle_length(Angle::degrees(120.), width),
    ]);
    let lshape2 = Polygon::new(vec![
        k,
        e,
        b,
        a,
        g,
        k + Vec2::from_angle_length(Angle::degrees(300.), width),
    ]);
    let lshape3 = Polygon::new(vec![
        i,
        j,
        a,
        g,
        g + Vec2::from_angle_length(Angle::degrees(180.), width),
        c,
    ]);
    vec![lshape1, lshape2, lshape3]
}

fn delaunay(polygon: &Polygon, n: usize, rng: &mut StdRng) -> Vec<Polygon> {
    // let mut points = polygon.sample_uniform(rng, n as u64);
    // points.append(&mut polygon.upsample(1).points.clone());
    let mut points = polygon.points.clone();
    println!("len on points {}", points.len());
    let mut triangles = vec![];
    let mut m = n;

    while m > 0 {
        let points2 = points
            .clone()
            .iter()
            .map(|v| Point { x: v.x, y: v.y })
            .collect::<Vec<Point>>();
        let result = triangulate(&points2);
        let mut newpoints: Vec<Vec2> = vec![];
        for i in (0..result.triangles.len()).step_by(3) {
            let p1 = points[result.triangles[i]];
            let p2 = points[result.triangles[i + 1]];
            let p3 = points[result.triangles[i + 2]];
            newpoints.push(p1);
            newpoints.push(p2);
            newpoints.push(p3);
            newpoints.push((p1 + p2).div(2.));
            newpoints.push((p2 + p3).div(2.));
            newpoints.push((p1 + p3).div(2.));
        }
        points = newpoints;
        m = m - 1;
    }

    let points2 = points
        .clone()
        .iter()
        .map(|v| Point { x: v.x, y: v.y })
        .collect::<Vec<Point>>();
    println!("len on points2 {}", points2.len());
    let result = triangulate(&points2);
    println!("result triangles {:?}", result.triangles);
    for i in result.triangles.iter() {
        // println!("- {:?}", i);
    }
    for i in (0..result.triangles.len()).step_by(3) {
        let p1 = points[result.triangles[i]];
        let p2 = points[result.triangles[i + 1]];
        let p3 = points[result.triangles[i + 2]];
        println!("{:?} {:?} {:?}", p1, p2, p3);
        triangles.push(Polygon::triangle(p1, p2, p3));
    }

    // let outer = geo::Polygon::new(polygon.to_geo_linestring(), vec![]);
    // triangles = triangles
    //     .iter()
    //     .filter(|t| {
    //         let inner = geo::Polygon::new(t.to_geo_linestring(), vec![]);
    //         outer.contains(&inner)
    //     })
    //     .cloned()
    //     .collect::<Vec<Polygon>>();

    triangles
}

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::axidraw_minikit(Portrait), true);
    let seed = Seed::new();
    let mut rng = StdRng::seed_from_u64(seed.clone().into());

    let mut group = Group::new();
    let mut hatch = Group::new();

    let triangle = penrose_triangle(sketch.center(), 120., 35.);
    triangle.iter().for_each(|p| group.add(p.clone()));

    // println!("len on &triangle[0].points {}", &triangle[0].points.len());

    // for point in &triangle[0].points {
    //     group.add(Circle::new(*point, 2.));
    // }

    // delaunay(&triangle[0], 0, &mut rng)
    //     .iter()
    //     .for_each(|p| hatch.add(p.clone()));

    // delaunay(&triangle[1], 50, &mut rng)
    //     .iter()
    //     .for_each(|p| hatch.add(p.clone()));

    sketch.add_group(&group, &Style::new("black", "0.5mm"));
    sketch.add_group(&hatch, &Style::new("black", "0.2mm"));
    sketch.render().save_default()?;
    Ok(())
}
