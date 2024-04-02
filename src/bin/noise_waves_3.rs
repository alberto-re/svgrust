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
            i: 0.,
        }
    }

    pub fn update(&mut self, noise: Perlin) {
        let noise = noise.get([self.x * 0.02, self.y * 0.02, self.i * 0.1]);
        let theta = map_range(
            noise,
            -1.,
            1.,
            Angle::from_degrees(90.).as_radians(),
            Angle::from_degrees(270.).as_radians(),
        );
        let v = Vec2::from_angle_length(Angle::from_radians(theta), 2.);
        self.x += v.x;
        self.y += v.y;
        self.i += 1.;
        self.trail.push(Vec2::new(self.x, self.y));
    }
}

// Try to imitate this
// https://openprocessing.org/sketch/756526

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::axidraw_minikit(Landscape), true);
    let perlin = Perlin::new(18);
    let mut group1 = Group::new();

    let n_particles: usize = 2000;

    let mut particles: Vec<Particle> = vec![];

    for i in 0..n_particles {
        let theta = map_range(i as f64, 0., n_particles as f64, 0., TAU);
        let pos = sketch.center()
            + Vec2::new(110., 0.)
            + Vec2::from_angle_length(Angle::from_radians(theta), 130.);
        particles.push(Particle::new(pos.x, pos.y));
    }

    (0..125).for_each(|_| {
        for i in 0..n_particles {
            particles[i].update(perlin);
        }
    });

    for i in 0..n_particles {
        group1.add_linestring(&LineString::new(particles[i].trail.clone()));
    }

    sketch.add_group(&group1, &Style::new("black", "0.1mm"));
    sketch.render().save_default()?;
    Ok(())
}
