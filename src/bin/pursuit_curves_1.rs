use anyhow::Result;
use noise::NoiseFn;
use noise::Perlin;
use plt::prelude::*;
use rand::{rngs::StdRng, Rng, SeedableRng};

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
    let mut sketch = Sketch::new(&PageLayout::a4(Portrait), Uom::Px, true);

    let seed = Seed::from_number(19);
    let mut rng = StdRng::seed_from_u64(seed.clone().into());
    let perlin = Perlin::new(seed.into());

    let rows = 8;
    let cols = 6;
    let t_min = 0.01;
    let t_max = 0.08;
    let t_step = 0.01;
    let times = 18;

    let mut shapes: Vec<Polygon> = vec![];
    let mut shapes2: Vec<Polygon> = vec![];

    Rect::new(
        Vec2::new(50., 50.),
        sketch.width() - 100.,
        sketch.height() - 100.,
    )
    .grid(rows, cols)
    .iter()
    .enumerate()
    .for_each(|(i, rect)| {
        let row = i / cols as usize;
        let col = i % cols as usize;
        let polygon = rect.to_polygon();
        shapes.push(polygon.clone());
        shapes2.push(polygon.clone());
        let clockwise = rng.gen::<f64>() < 0.5;
        let t = map_range(
            perlin.get([row as f64 * 0.05, col as f64 * 0.05]),
            -1.,
            1.,
            t_min,
            t_max,
        );
        let t = if clockwise { t } else { 1. - t };
        let t_step = if clockwise { t_step } else { t_step * -1. };
        pursuit_polygons_times(&polygon, t, t_step, times)
            .iter()
            .for_each(|p| shapes.push(p.clone()));
        pursuit_polygons_times(&polygon, t, t_step, times)
            .iter()
            .for_each(|p| shapes2.push(p.clone()));
    });

    let mut group = Group::new();
    group.add_many(shapes);
    let mut group2 = Group::new();
    group2.add_many(shapes2);
    sketch.add_group(&group, &Style::new("blue", "0.4mm"));
    sketch.add_group(&group2, &Style::new("green", "0.4mm"));
    sketch.render().save_default()?;
    Ok(())
}
