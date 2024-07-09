use anyhow::Result;
use noise::NoiseFn;
use noise::Perlin;
use plt::prelude::*;
use plt::traits::HatchFillStrategy;
use rand::{rngs::StdRng, Rng, SeedableRng};

// https://www.gorillasun.de/blog/a-guide-to-hexagonal-grids-in-p5js/
// Maybe take inspiration from this? https://tyrer.io/crafted-by-code - James Webb telescope?

const SEED: u32 = 54;
const HEX_SIDE: f64 = 11.0;
const NOISE_SCALE: f64 = 0.3;
const NO_FILL_RATE: f64 = 0.05;

fn hatch_fill(hex: &Hexagon, sketch: &mut Sketch, perlin: &Perlin, rng: &mut StdRng, pen: &Pen) {
    if rng.gen::<f64>() < NO_FILL_RATE {
        // Do not fill
    } else {
        let noise_val = perlin.get([hex.center.x * NOISE_SCALE, hex.center.y * NOISE_SCALE]);
        let noise_val = (noise_val + 1.0) / 2.0;
        let mut group_n = 0;
        if noise_val > 0.666 {
            group_n = 2;
        } else if noise_val > 0.333 {
            group_n = 1;
        }
        sketch
            .group(group_n)
            .add_many(hex.hatch_fill(pen, HatchFillStrategy::HorizontalLines));
    }
}

fn decoration_1(hex: &Hexagon) -> Vec<LineString> {
    let mut decoration: Vec<LineString> = vec![];
    let vertexes = hex.vertexes();
    let start_a = hex.center.lerp(vertexes[0], 0.8);
    let start_b = hex.center.lerp(vertexes[1], 0.8);
    let end_a = hex.center;
    let end_b = hex.center.lerp(vertexes[2], 0.8);
    let mut t = 0.1;
    while t < 1. {
        let line = LineString::line(start_a.lerp(end_a, t), start_b.lerp(end_b, t));
        decoration.push(line.clone());
        t += 0.15;
    }
    let start_a = hex.center.lerp(vertexes[2], 0.8);
    let start_b = hex.center.lerp(vertexes[3], 0.8);
    let end_a = hex.center;
    let end_b = hex.center.lerp(vertexes[4], 0.8);
    let mut t = 0.1;
    while t < 1. {
        let line = LineString::line(start_a.lerp(end_a, t), start_b.lerp(end_b, t));
        decoration.push(line.clone());
        t += 0.15;
    }
    let start_a = hex.center.lerp(vertexes[4], 0.8);
    let start_b = hex.center.lerp(vertexes[5], 0.8);
    let end_a = hex.center;
    let end_b = hex.center.lerp(vertexes[0], 0.8);
    let mut t = 0.1;
    while t < 1. {
        let line = LineString::line(start_a.lerp(end_a, t), start_b.lerp(end_b, t));
        decoration.push(line.clone());
        t += 0.15;
    }
    decoration
}

fn decoration_2(hex: &Hexagon) -> Vec<LineString> {
    vec![
        LineString::line(
            hex.center,
            hex.center + Vec2::from_polar(Angle::from_degrees(120.), hex.apothem * 0.9),
        ),
        LineString::line(
            hex.center,
            hex.center + Vec2::from_polar(Angle::from_degrees(240.), hex.apothem * 0.9),
        ),
        LineString::line(
            hex.center,
            hex.center + Vec2::from_polar(Angle::from_degrees(360.), hex.apothem * 0.9),
        ),
    ]
}

fn decoration_3(hex: &Hexagon) -> Vec<LineString> {
    let mut decoration: Vec<LineString> = vec![];
    let vertexes = hex.vertexes();
    let start_a = hex.center.lerp(vertexes[0], 0.8);
    let start_b = hex.center.lerp(vertexes[1], 0.8);
    let end_b = hex.center;
    let end_a = hex.center.lerp(vertexes[2], 0.8);
    let mut t = 0.1;
    while t < 1. {
        let line = LineString::line(start_a.lerp(end_a, t), start_b.lerp(end_b, t));
        decoration.push(line.clone());
        t += 0.15;
    }
    let start_a = hex.center.lerp(vertexes[2], 0.8);
    let start_b = hex.center.lerp(vertexes[3], 0.8);
    let end_b = hex.center;
    let end_a = hex.center.lerp(vertexes[4], 0.8);
    let mut t = 0.1;
    while t < 1. {
        let line = LineString::line(start_a.lerp(end_a, t), start_b.lerp(end_b, t));
        decoration.push(line.clone());
        t += 0.15;
    }
    let start_a = hex.center.lerp(vertexes[4], 0.8);
    let start_b = hex.center.lerp(vertexes[5], 0.8);
    let end_b = hex.center;
    let end_a = hex.center.lerp(vertexes[0], 0.8);
    let mut t = 0.1;
    while t < 1. {
        let line = LineString::line(start_a.lerp(end_a, t), start_b.lerp(end_b, t));
        decoration.push(line.clone());
        t += 0.15;
    }
    decoration
}

fn decoration_4(hex: &Hexagon, rng: &mut StdRng) -> Vec<LineString> {
    let mut decoration: Vec<LineString> = vec![];
    if rng.gen::<f64>() < 0.5 {
        for side in hex.scale_perc(0.7).sides() {
            decoration.push(LineString::line(side.0, side.1));
        }
    }
    if rng.gen::<f64>() < 0.5 {
        for side in hex.scale_perc(0.575).sides() {
            decoration.push(LineString::line(side.0, side.1));
        }
    }
    if rng.gen::<f64>() < 0.5 {
        for side in hex.scale_perc(0.45).sides() {
            decoration.push(LineString::line(side.0, side.1));
        }
    }
    if rng.gen::<f64>() < 0.5 {
        for side in hex.scale_perc(0.325).sides() {
            decoration.push(LineString::line(side.0, side.1));
        }
    }
    if rng.gen::<f64>() < 0.5 {
        for side in hex.scale_perc(0.2).sides() {
            decoration.push(LineString::line(side.0, side.1));
        }
    }
    decoration
}

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::a4(Portrait), Uom::Mm, Debug::On);
    let seed = Seed::number(SEED);
    let mut rng = StdRng::seed_from_u64(seed.clone().into());
    let perlin = Perlin::new(seed.into());

    // a6 = 19
    // a5 = 37
    // a4 = 91
    // a3 = 130
    let mut hexagons: Vec<Hexagon> =
        Hexagon::spiral(sketch.center(), HEX_SIDE, Angle::from_degrees(0.), 91);

    let pen_fill_1 = Pen::pigma_micron_05_blue();
    let pen_fill_2 = Pen::pigma_micron_05_purple();
    let pen_fill_3 = Pen::pigma_micron_05_green();
    let pen_deco = Pen::uniball_signo_broad_gold();
    let pen_contour = Pen::uniball_signo_broad_gold();

    sketch.group(0).set_pen(&pen_fill_1);
    sketch.group(1).set_pen(&pen_fill_2);
    sketch.group(2).set_pen(&pen_fill_3);
    sketch.group(3).set_pen(&pen_deco);
    sketch.group(4).set_pen(&pen_contour);

    hexagons = hexagons.iter().map(|x| x.scale_perc(0.9)).collect();
    let mut decorations: Vec<LineString> = vec![];

    hexagons
        .iter()
        .for_each(|hex| hatch_fill(hex, &mut sketch, &perlin, &mut rng, &pen_fill_1));

    hexagons.iter().for_each(|h| {
        if rng.gen::<f64>() < 0.05 {
            decorations.append(&mut decoration_1(h));
        } else if rng.gen::<f64>() < 0.05 {
            decorations.append(&mut decoration_3(h));
        } else {
            if rng.gen::<f64>() < 0.1 {
                decorations.append(&mut decoration_2(h));
            }
            if rng.gen::<f64>() < 0.5 {
                decorations.append(&mut decoration_4(h, &mut rng));
            }
        }
    });

    let mut scaled: Vec<Hexagon> = vec![];
    scaled.append(
        &mut hexagons
            .iter()
            .map(|h| h.scale_dist(pen_contour.thickness))
            .collect::<Vec<Hexagon>>(),
    );
    scaled.append(
        &mut hexagons
            .iter()
            .map(|h| h.scale_dist(-pen_contour.thickness))
            .collect::<Vec<Hexagon>>(),
    );
    hexagons.append(&mut scaled);

    // decorations = decorations
    //     .iter()
    //     .map(|h| {
    //         let diff = pen_fill_1.diameter - pen_contour.diameter;
    //         let center_translated = h.center + Vec2::new(diff / 2., 0.);
    //         Hexagon::new(center_translated, h.side, h.theta)
    //     })
    //     .collect();

    sketch.group(3).add_many(decorations);

    // hexagons = hexagons
    //     .iter()
    //     .map(|h| {
    //         let diff = pen_fill_1.diameter - pen_contour.diameter;
    //         let center_translated = h.center + Vec2::new(diff / 2., 0.);
    //         Hexagon::new(center_translated, h.side, h.theta)
    //     })
    //     .collect();

    sketch.group(4).add_many(hexagons);

    sketch.render().save_default()?;
    Ok(())
}
