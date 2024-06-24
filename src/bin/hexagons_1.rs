use anyhow::Result;
use plt::prelude::*;
use rand::{rngs::StdRng, Rng, SeedableRng};
use rayon::prelude::*;

// https://www.gorillasun.de/blog/a-guide-to-hexagonal-grids-in-p5js/
// Maybe take inspiration from this? https://tyrer.io/crafted-by-code - James Webb telescope?

const SEED: u32 = 120;
const HEX_SIDE: f64 = 8.;

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::a4(Portrait), Uom::Mm, Debug::Off);

    let seed = Seed::number(SEED);
    let mut rng = StdRng::seed_from_u64(seed.clone().into());

    let mut count = 0;
    let mut y = 0.;
    let mut hexagons: Vec<Polygon> = vec![];

    while y < 300. {
        let mut x = 0.;
        while x < 1000. {
            let mut curx = x;
            if count % 2 == 0 {
                curx = x + HEX_SIDE * 0.75;
            }
            let center = Vec2::new(curx, y);
            let hx = Polygon::hexagon(center, HEX_SIDE / 2.);
            hexagons.push(hx);
            x += HEX_SIDE * 1.50;
        }
        y += HEX_SIDE / 2.3;
        count += 1;
    }

    hexagons = hexagons
        .iter()
        .map(|h| {
            let x = h.centroid().x;
            let y = h.centroid().y;
            let t = 0.;
            let scalar =
                f64::sin(2.31 * x + 1.11 * t + 5.95 + 2.57 * f64::sin(1.73 * y - 1.65 * t + 1.87))
                    + f64::sin(
                        7.09 * y - 1.28 * t + 4.15 + 2.31 * f64::sin(2.53 * x + 1.66 * t + 4.45),
                    );
            let scalar = (scalar + 2.) / 4. * 0.85 + rng.gen::<f64>() * 0.15;
            h.scale_dist(scalar * scalar * 3.5)
        })
        .collect();

    hexagons = hexagons
        .iter()
        .map(|p| p.rotate(Angle::degrees(25.)))
        .collect();

    let hexagons_clipped: Vec<LineString> = hexagons
        .par_iter()
        .flat_map(|p| p.clip(&sketch.as_rect().scale_perc(0.9).to_polygon(), false))
        .collect();

    sketch.group(0).add_many(hexagons_clipped);
    sketch.group(0).set_pen(Pen::pigma_micron_05_black());
    sketch.render().save_default()?;
    Ok(())
}
