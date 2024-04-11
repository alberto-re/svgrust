use anyhow::Result;
use noise::NoiseFn;
use noise::Perlin;
use plt::prelude::*;
use rand::{rngs::StdRng, Rng, SeedableRng};

fn pursuit_polygons(polygon: &Polygon, t: f64, t_step: f64, times: usize) -> Vec<Polygon> {

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

    let mut ret = vec![
        pursuit_once(polygon, t)
    ];

    for n in 1..times {
        ret.push(pursuit_once(ret.last().unwrap(), t + n as f64 * t_step));
    }
    ret
}

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::a4(Portrait), true);

    let seed = Seed::from_number(91);
    let mut rng = StdRng::seed_from_u64(seed.clone().into());
    let perlin = Perlin::new(seed.into());

    let rows = 8;
    let cols = 6;
    let t_min = 0.04;
    let t_max = 0.04;
    let t_step = 0.01;
    let times = 12;

    let mut shapes: Vec<Polygon> = vec![];

    sketch.as_rect().grid(rows, cols).iter().enumerate().for_each(|(i, rect)| {
        let row = i / cols as usize;
        let col = i % cols as usize;
        let polygon = rect.to_polygon();
        shapes.push(polygon.clone());
        let clockwise = if rng.gen::<f64>() < 0.5 { true } else { false };
        let t = map_range(perlin.get([row as f64 * 0.1, col as f64 * 0.1]), -1., 1., t_min, t_max);
        let t = if clockwise { t } else { 1. - t };
        let t_step = if clockwise { t_step } else { t_step * -1. };
        pursuit_polygons(&polygon, t, t_step, times).iter().for_each(|p| shapes.push(p.clone()));
    });

    let mut group = Group::new();
    group.add_many(shapes);
    sketch.add_group(&group, &Style::new("black", "0.4mm"));
    sketch.render().save_default()?;
    Ok(())
}
