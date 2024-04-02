use anyhow::Result;
use noise::NoiseFn;
use noise::Perlin;
use plt::prelude::*;

struct Particle {
    x: f64,
    y: f64,
    startx: f64,
    starty: f64,
    trail: Vec<Vec2>,
    i: f64,
}

impl Particle {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            startx: x,
            starty: y,
            trail: vec![],
            i: 0.
        }
    }

    pub fn update(&mut self, noise: Perlin, itersl: usize, itersr: usize) {
        let mut t1: Vec<Vec2> = vec![];
        let mut t2: Vec<Vec2> = vec![];
        for _ in 0..itersl {
            let noise = noise.get([self.x * 0.005, self.y * 0.01, self.i * 0.7]);
            let theta = map_range(noise, -1., 1., 0., TAU);
            let v = Vec2::from_angle_length(Angle::from_radians(theta), 1.);
            self.x += v.x;
            self.y += v.y;
            self.i += 1.;
            t1.push(Vec2::new(self.x, self.y));
        }
        self.x = self.startx;
        self.y = self.starty;
        self.i = 0.;
        for _ in 0..itersr {
            let noise = noise.get([self.x * 0.005, self.y * 0.01, -self.i * 0.7]);
            let theta = map_range(noise, -1., 1., 0., TAU);
            let theta = theta + PI;
            let v = Vec2::from_angle_length(Angle::from_radians(theta), 1.);
            self.x += v.x;
            self.y += v.y;
            self.i += 1.;
            t2.push(Vec2::new(self.x, self.y));
        }
        t1.reverse();
        for p in &t1 {
            self.trail.push(p.clone());
        }
        for p in &t2 {
            self.trail.push(p.clone());
        }
    }
}

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::axidraw_minikit(Portrait), true);
    let perlin = Perlin::new(51);
    let mut group1 = Group::new();

    let mut particles_left: Vec<Particle> = vec![];

    for i in (0..sketch.height() as usize).step_by(2) {
        let noise = perlin.get([i as f64 * 0.004, i  as f64 * 0.004]);
        particles_left.push(Particle::new(sketch.center().x + noise * 50., i as f64));
    }

    particles_left.iter_mut().for_each(|p| {
        // let iters = sketch.center().y - (f64::abs(p.y - sketch.center().y));
        // let iters = iters * 1.;
        let noisel = perlin.get([p.y * 0.002, p.y * 0.002, 10.]);
        let maxwl = map_range(sketch.center().y - (f64::abs(p.y - sketch.center().y)), 0., 150., 0., sketch.width() / 2.);
        let noiser = perlin.get([p.y * 0.002, p.y * 0.002, 40.]);
        let maxwr = map_range(sketch.center().y - (f64::abs(p.y - sketch.center().y)), 0., 150., 0., sketch.width() / 2.);
        let itersl = map_range(noisel, -1., 1., 5., 300.);
        let itersl = itersl as usize;
        let itersr = map_range(noiser, -1., 1., 5., 300.);
        let itersr = itersr as usize;
        p.update(perlin, itersl, itersr);
    });

    particles_left.iter_mut().for_each(|p| {
        group1.add_linestring(&LineString::new(p.trail.clone()));
    });

    // let mut uberlinestring: Vec<Vec2> = vec![];

    // particles_left.iter_mut().enumerate().for_each(|(i, p)| {
    //     if i % 2 == 0 {
    //         p.trail.reverse();
    //         uberlinestring.append(&mut p.trail);
    //     } else {
    //         uberlinestring.append(&mut p.trail);
    //     }
    // });

    // group1.add_linestring(&LineString::new(uberlinestring));

    // particles_left.iter_mut().for_each(|p| {
    //     group1.add_linestring(&LineString::new(p.trail.clone()));
    // });
   
    sketch.add_group(&group1, &Style::new("rgba(255,0,0,0.4)", "0.4mm"));
    sketch.render().save_default()?;
    Ok(())
}
