use anyhow::Result;
use noise::NoiseFn;
use noise::Perlin;
use plt::prelude::*;

struct Particle {
    x: f64,
    y: f64,
    trail: Vec<Vec2>,
    i: f64,
}

impl Particle {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            trail: vec![Vec2::new(x, y)],
            i: 0.
        }
    }

    pub fn update(&mut self, noise: Perlin) {
        let noise = noise.get([self.x * 0.01, self.y * 0.01, self.i * 0.5]);
        let theta = map_range(noise, 0., 1., 0., TAU);
        let v = Vec2::from_angle_length(Angle::from_radians(theta), 1.);
        self.x += v.x;
        self.y += v.y;
        self.i += 1.;
        self.trail.push(Vec2::new(self.x, self.y));
    }
}

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::axidraw_minikit(Portrait), true);
    let perlin = Perlin::new(705);
    let mut group1 = Group::new();

    let mut particles: Vec<Particle> = vec![];

    for i in (20..(sketch.height() as usize - 40)).step_by(2) {
        particles.push(Particle::new(sketch.center().x, i as f64));
    }

    particles.iter_mut().for_each(|p| {
        let iters = sketch.center().y - (f64::abs(p.y - sketch.center().y));
        let iters = iters * 5.;
        let iters = iters as usize;
        for _ in 0..iters {
            p.update(perlin);
        }
    });

    particles.iter_mut().for_each(|p| {
        group1.add_linestring(&LineString::new(p.trail.clone()));
    });

    sketch.add_group(&group1, &Style::new("rgba(255,0,0,0.4)", "0.4mm"));
    sketch.render().save_default()?;
    Ok(())
}
