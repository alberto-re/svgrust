use anyhow::Result;
use plt::prelude::*;

const SEED: u64 = 123;
const POISSON_RADIUS: f64 = 3.;
const RADIUS_TO_MINLEN_RATIO: f64 = 0.4;
const CIRCUMFERENCE_N_SAMPLE: usize = 100;

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::a4(Portrait), Uom::Mm, Debug::On);
    sketch.group(0).set_pen(&Pen::pigma_micron_05_black());
    sketch.group(1).set_pen(&Pen::pigma_micron_05_red());

    let circle = Circle::new(sketch.center(), sketch.min_len() * RADIUS_TO_MINLEN_RATIO);

    let mut points = sketch
        .as_rect()
        .sample_poisson2d(POISSON_RADIUS, SEED)
        .iter()
        .filter(|p| p.distance(circle.center) < circle.radius)
        .map(|p| *p)
        .collect::<Vec<Vec2>>();

    points.append(&mut circle.to_polygon(CIRCUMFERENCE_N_SAMPLE).points);

    let triangles = points.triangulate();

    let chosen = 100;
    let adj = find_adjacent(chosen, &triangles);
    if let Some(adja) = adj {
        println!("Found");
        sketch.group(1).add(triangles[adja].clone());
    } else {
        println!("Not found");
    }

    sketch.group(1).add(triangles[chosen].clone());

    sketch.group(0).add_many(triangles);

    sketch.render().save_default()?;
    Ok(())
}

fn merge_adjacent(left: &Polygon, right: &Polygon) -> Polygon {
    let mut points: Vec<Vec2> = vec![];
    points.append(&mut left.points.clone());
    points.append(&mut right.points.clone());
    
    let center_point = points.centroid();

    points = points.iter().map(|p|).collect();
    Polygon::new(vec![])
}

fn find_adjacent(index: usize, triangles: &Vec<Polygon>) -> Option<usize> {
    let left_edges = triangles[index].edges();
    for (otherindex, other) in triangles.iter().enumerate() {
        if otherindex == index {
            continue;
        }
        let right_edges = other.edges();
        for ledge in &left_edges {
            for redge in &right_edges {
                if ledge.0.distance(redge.0) + ledge.1.distance(redge.1) < 0.01
                    || ledge.1.distance(redge.0) + ledge.0.distance(redge.1) < 0.01
                {
                    return Some(otherindex);
                }
            }
        }
    }
    None
}
