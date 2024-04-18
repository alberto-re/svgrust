use anyhow::Result;
use noise::NoiseFn;
use noise::Perlin;
use plt::prelude::*;
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

// https://openprocessing.org/sketch/2008342

// Main ideas explored here:
// - plots with a lot of empty spaces, in general, looks better on dark paper
// - repetition is good, but a bit of unexpected makes the plot more interesting to the eye

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::axidraw_minikit(Portrait), Uom::Px, false);
    let mut layer = Group::new();

    let seed = Seed::from_number(90);
    let mut rng = StdRng::seed_from_u64(seed.clone().into());
    let perlin = Perlin::new(seed.into());

    let enclosing = sketch.as_rect();
    let cells = enclosing.grid(60, 40);

    cells.iter().for_each(|c| {
        let mut points: Vec<Vec2> = vec![];
        if rng.gen::<f64>() < 0.5 {
            points.push(Vec2 {
                x: c.xy.x,
                y: c.xy.y,
            });
            points.push(Vec2 {
                x: c.xy.x + c.width,
                y: c.xy.y + c.height,
            });
        } else {
            points.push(Vec2 {
                x: c.xy.x + c.width,
                y: c.xy.y,
            });
            points.push(Vec2 {
                x: c.xy.x,
                y: c.xy.y + c.height,
            });
        }
        layer.add(&LineString::new(points).add_vec(enclosing.xy));
    });

    let circle = Circle::new(
        sketch.as_rect().centroid(),
        enclosing.scale_perc(0.98).min_len() / 2.,
    );

    let (inner, _) = layer.split_shape(&circle.scale_perc(0.9));
    let mut inner1 = Group::new();

    inner.linestrings().iter().for_each(|e| {
        let val = perlin.get([e.centroid().x * 0.015, e.centroid().y * 0.02]);
        if val < 0.5 && rng.gen::<f64>() < 0.85 {
            inner1.add(e)
        }
    });

    inner1.add(&circle.scale_perc(0.990));
    inner1.add(&circle.scale_perc(0.985));
    inner1.add(&circle.scale_perc(0.980));
    inner1.add(&circle.scale_perc(0.975));
    inner1.add(&circle.scale_perc(0.970));
    inner1.add(&circle.scale_perc(0.965));
    inner1.add(&circle.scale_perc(0.960));

    sketch.add_group(&inner1, &Style::new("black", "2.0px"));

    sketch.render().save_default()?;
    Ok(())
}
