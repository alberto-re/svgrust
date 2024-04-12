use anyhow::Result;
use plt::prelude::*;

fn pursuit_polygons_times(polygon: &Polygon, t: f64, t_step: f64, times: usize) -> Vec<Polygon> {
    if times == 0 {
        return vec![];
    }

    fn pursuit_once(polygon: &Polygon, t: f64) -> Polygon {
        let mut vecs = vec![];
        for edge in polygon.edges() {
            let newvec = edge.0.lerp(&edge.1, t);
            vecs.push(newvec);
        }
        Polygon::new(vecs)
    }

    let mut ret = vec![pursuit_once(polygon, t)];

    for n in 1..times {
        ret.push(pursuit_once(ret.last().unwrap(), t + n as f64 * t_step));
    }
    ret
}

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::a4(Portrait), true);
    
    let mut group = Group::new();

    let mut pset: Vec<Vec2> = vec![];

    let steps: Vec<(f64, usize)> = vec![(0.45, 17), (0.30, 9), (0.15, 5)];

    for (step_radius, step_n_poly) in steps {
        Circle::new(sketch.center(), sketch.min_len() * step_radius)
            .to_polygon(step_n_poly)
            .points
            .iter()
            .for_each(|p| pset.push(*p));
    }

    pset.triangulate().iter().for_each(|triangle| {
        group.add(triangle.clone());
        group.add_many(pursuit_polygons_times(&triangle, 0.08, 0.0, 20));
    });

    sketch.add_group(&group, &Style::new("black", "0.3mm"));
    sketch.render().save_default()?;
    Ok(())
}
