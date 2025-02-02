use anyhow::Result;
use plt::prelude::*;
use plt::traits::Triangulate;

fn pursuit_polygons_times(polygon: &Polygon, t: f64, t_step: f64, times: usize) -> Vec<Polygon> {
    if times == 0 {
        return vec![];
    }

    fn pursuit_once(polygon: &Polygon, t: f64) -> Polygon {
        let mut vecs = vec![];
        for edge in polygon.edges() {
            let newvec = edge.v1.lerp(edge.v2, t);
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
    let mut sketch = Sketch::new(&PageLayout::a4(Portrait), Uom::Px, Debug::Off);

    let mut group = Group::new();

    let mut pset: Vec<Vec2> = vec![];

    let steps: Vec<(f64, usize)> = vec![(0.48, 19), (0.40, 16), (0.30, 13), (0.20, 8), (0.10, 5)];

    for (step_radius, step_n_poly) in steps {
        Circle::new(sketch.center(), sketch.min_len() * 0.9 * step_radius)
            .to_polygon(step_n_poly)
            .points
            .iter()
            .for_each(|p| pset.push(*p));
    }

    pset.triangulate()
        .iter()
        .map(|t| t.to_polygon())
        .for_each(|triangle| {
            group.add(triangle.clone());
            let times = map_range(
                triangle.centroid().distance(Vec2::new(100., 1000.)),
                0.,
                300.,
                10.,
                15.,
            );
            sketch
                .group(0)
                .add_many(pursuit_polygons_times(&triangle, 0.06, 0.0, times as usize));
        });

    sketch.group(0).set_style(Style::new("black", "0.5mm"));
    sketch.render().save_default()?;
    Ok(())
}
