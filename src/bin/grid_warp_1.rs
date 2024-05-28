use anyhow::Result;
use plt::prelude::*;
use plt::vectorfield::Spiral2dVectorField;
use plt::vectorfield::PerlinNoise2dVectorField;
use plt::vectorfield::VectorAt;
use rand::rngs::StdRng;
use rand::SeedableRng;
use rand::Rng;

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::a4(Portrait), Uom::Px, false);
    let mut group = Group::new();
    let mut group2 = Group::new();
    let mut group3 = Group::new();

    let seed = Seed::from_number(2476206517);
    let mut rng = StdRng::seed_from_u64(seed.clone().into());
    let perlin_field = PerlinNoise2dVectorField::new(0.0015, 0.0025, seed.into());
    let vector_field = Spiral2dVectorField::new(sketch.center());

    let rows: usize = 120;
    let cols: usize = 90;
    let balls: usize = 5;

    let mut cells = sketch.as_rect().scale_perc(0.95).grid(rows as u64, cols as u64);
    let mut cells2 = sketch.as_rect().scale_perc(0.95).grid(rows as u64, cols as u64);
    let mut cells3 = sketch.as_rect().scale_perc(0.95).grid(rows as u64, cols as u64);

    for cell in cells.iter_mut() {
        for _ in 0..6 {
            cell.xy = cell.xy + perlin_field.vector_at(cell.xy) * Vec2::new(2., 2.);
        }
    }
    for cell in cells2.iter_mut() {
        for _ in 0..6 {
            cell.xy = cell.xy + perlin_field.vector_at(cell.xy) * Vec2::new(3., 3.);
        }
    }
    for cell in cells3.iter_mut() {
        for _ in 0..6 {
            cell.xy = cell.xy + perlin_field.vector_at(cell.xy) * Vec2::new(4., 4.);
        }
    }

    for _ in 0..balls {
        let x = rng.gen::<f64>() * sketch.width() * 0.8 + sketch.width() * 0.1;
        let y = rng.gen::<f64>() * sketch.height() * 0.8 + sketch.height() * 0.1;
        let point = Vec2::new(x, y);
        let radius = 30. + rng.gen::<f64>() * 200.;
        let iters = 100;
        for cell in cells.iter_mut() {
            for _ in 0..iters {
                let dist = cell.xy.distance(&point);
                let dist = f64::abs(dist - radius);
                let force = map_range(dist, 0., 600., 1., 0.);
                let force = force.powi(18);
                cell.xy = cell.xy + vector_field.vector_at(cell.xy) * force;
            }
        }
        for cell in cells2.iter_mut() {
            for _ in 0..iters {
                let dist = cell.xy.distance(&point);
                let dist = f64::abs(dist - radius);
                let force = map_range(dist, 0., 600., 1., 0.);
                let force = force.powi(18);
                cell.xy = cell.xy + vector_field.vector_at(cell.xy) * force;
            }
        }
        for cell in cells3.iter_mut() {
            for _ in 0..iters {
                let dist = cell.xy.distance(&point);
                let dist = f64::abs(dist - radius);
                let force = map_range(dist, 0., 600., 1., 0.);
                let force = force.powi(18);
                cell.xy = cell.xy + vector_field.vector_at(cell.xy) * force;
            }
        }
    }

    for r in 0..rows {
        let mut vals = vec![];
        let mut vals2 = vec![];
        let mut vals3 = vec![];
        for c in 0..cols {
            let i = r * cols + c;
            vals.push(cells[i].xy);
            vals2.push(cells2[i].xy);
            vals3.push(cells3[i].xy);
        }
        group.add(LineString::new(vals));
        group2.add(LineString::new(vals2));
        group3.add(LineString::new(vals3));
    }

    for c in 0..cols {
        let mut vals = vec![];
        let mut vals2 = vec![];
        let mut vals3 = vec![];
        for r in 0..rows {
            let i = r * cols + c;
            vals.push(cells[i].xy);
            vals2.push(cells2[i].xy);
            vals3.push(cells2[i].xy);
        }
        group.add(LineString::new(vals));
        group2.add(LineString::new(vals2));
        group3.add(LineString::new(vals3));
    }

    sketch.add_group(&group3, &Style::new("lime", "1.0px"));
    sketch.add_group(&group2, &Style::new("green", "1.0px"));
    sketch.add_group(&group, &Style::new("black", "1.0px"));
    sketch.render().save_default()?;
    Ok(())
}
