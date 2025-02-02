use anyhow::Result;
use plt::field::PerlinField;
use plt::field::SpiralField;
use plt::field::Vector2to2;
use plt::field::Vector3to2;
use plt::prelude::*;
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::a4(Portrait), Uom::Px, Debug::Off);

    let seed = Seed::number(3650491894);
    let mut rng = StdRng::seed_from_u64(seed.clone().into());
    let perlin_field = PerlinField::new(seed.into());
    let spiral_field = SpiralField::new(sketch.center());

    let noise_scale = 0.002;
    let rows: usize = 150;
    let cols: usize = (rows as f64 * sketch.layout.aspect_ratio()) as usize;
    let balls: usize = 5;

    let mut cells = sketch
        .as_rect()
        .scale_perc(0.85)
        .grid(rows as u64, cols as u64);
    let mut cells2 = sketch
        .as_rect()
        .scale_perc(0.85)
        .grid(rows as u64, cols as u64);
    let mut cells3 = sketch
        .as_rect()
        .scale_perc(0.85)
        .grid(rows as u64, cols as u64);
    let mut cells4 = sketch
        .as_rect()
        .scale_perc(0.85)
        .grid(rows as u64, cols as u64);

    for _ in 0..balls {
        let x = rng.gen::<f64>() * sketch.width() * 0.8 + sketch.width() * 0.1;
        let y = rng.gen::<f64>() * sketch.height() * 0.8 + sketch.height() * 0.1;
        let point = Vec2::new(x, y);
        let radius = 30. + rng.gen::<f64>() * 230.;
        let iters = 100;
        for cell in cells.iter_mut() {
            for _ in 0..iters {
                let dist = cell.xy.distance(point);
                let dist = f64::abs(dist - radius);
                let force = map_range(dist, 0., 600., 1., 0.);
                let force = force.powi(18);
                cell.xy = cell.xy + spiral_field.vec2(cell.xy) * force;
            }
        }
        for cell in cells2.iter_mut() {
            for _ in 0..iters {
                let dist = cell.xy.distance(point);
                let dist = f64::abs(dist - radius);
                let force = map_range(dist, 0., 600., 1., 0.);
                let force = force.powi(18);
                cell.xy = cell.xy + spiral_field.vec2(cell.xy) * force;
            }
        }
        for cell in cells3.iter_mut() {
            for _ in 0..iters {
                let dist = cell.xy.distance(point);
                let dist = f64::abs(dist - radius);
                let force = map_range(dist, 0., 600., 1., 0.);
                let force = force.powi(18);
                cell.xy = cell.xy + spiral_field.vec2(cell.xy) * force;
            }
        }
        for cell in cells4.iter_mut() {
            for _ in 0..iters {
                let dist = cell.xy.distance(point);
                let dist = f64::abs(dist - radius);
                let force = map_range(dist, 0., 600., 1., 0.);
                let force = force.powi(18);
                cell.xy = cell.xy + spiral_field.vec2(cell.xy) * force;
            }
        }
    }

    for cell in cells.iter_mut() {
        for _ in 0..6 {
            let xyz = Vec3::new(cell.xy.x, cell.xy.y, 0.) * noise_scale;
            cell.xy += perlin_field.vec3(xyz);
        }
    }
    for cell in cells2.iter_mut() {
        for _ in 0..6 {
            let xyz = Vec3::new(cell.xy.x, cell.xy.y, 0.) * noise_scale;
            cell.xy += perlin_field.vec3(xyz + Vec3::new(0., 0., 0.3));
        }
    }
    for cell in cells3.iter_mut() {
        for _ in 0..6 {
            let xyz = Vec3::new(cell.xy.x, cell.xy.y, 0.) * noise_scale;
            cell.xy += perlin_field.vec3(xyz + Vec3::new(0., 0., 0.6));
        }
    }
    for cell in cells4.iter_mut() {
        for _ in 0..6 {
            let xyz = Vec3::new(cell.xy.x, cell.xy.y, 0.) * noise_scale;
            cell.xy += perlin_field.vec3(xyz + Vec3::new(0., 0., 1.0));
        }
    }

    for r in 0..rows {
        let mut vals = vec![];
        let mut vals2 = vec![];
        let mut vals3 = vec![];
        let mut vals4 = vec![];
        for c in 0..cols {
            let i = r * cols + c;
            if r % 2 == 0 {
                vals.push(cells[i].xy);
                vals2.push(cells2[i].xy);
                vals3.push(cells3[i].xy);
                vals4.push(cells4[i].xy);
            } else {
                vals.insert(0, cells[i].xy);
                vals2.insert(0, cells2[i].xy);
                vals3.insert(0, cells3[i].xy);
                vals4.insert(0, cells4[i].xy);
            }
        }
        sketch.group(0).add(LineString::new(vals));
        sketch.group(1).add(LineString::new(vals2));
        sketch.group(2).add(LineString::new(vals3));
        sketch.group(3).add(LineString::new(vals4));
    }

    for c in 0..cols {
        let mut vals = vec![];
        let mut vals2 = vec![];
        let mut vals3 = vec![];
        let mut vals4 = vec![];
        for r in 0..rows {
            let i = r * cols + c;
            if c % 2 == 0 {
                vals.push(cells[i].xy);
                vals2.push(cells2[i].xy);
                vals3.push(cells3[i].xy);
                vals4.push(cells4[i].xy);
            } else {
                vals.insert(0, cells[i].xy);
                vals2.insert(0, cells2[i].xy);
                vals3.insert(0, cells3[i].xy);
                vals4.insert(0, cells4[i].xy);
            }
        }
        sketch.group(0).add(LineString::new(vals));
        sketch.group(1).add(LineString::new(vals2));
        sketch.group(2).add(LineString::new(vals3));
        sketch.group(3).add(LineString::new(vals4));
    }

    sketch.group(3).set_style(Style::new("blue", "1.0px"));
    sketch.group(2).set_style(Style::new("azure", "1.0px"));
    sketch.group(1).set_style(Style::new("blue", "1.0px"));
    sketch.group(0).set_style(Style::new("blue", "1.0px"));
    sketch.render().save_default()?;
    Ok(())
}
