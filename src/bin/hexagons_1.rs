use anyhow::Result;
use plt::prelude::*;

// https://www.gorillasun.de/blog/a-guide-to-hexagonal-grids-in-p5js/
// Maybe take inspiration from this? https://tyrer.io/crafted-by-code - James Webb telescope?

const HEX_SIDE: f64 = 7.;

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::axidraw_minikit(Portrait), Uom::Mm, Debug::On);
    let hexagons: Vec<Hexagon> =
        Hexagon::spiral(sketch.center(), HEX_SIDE, Angle::degrees(120.), 37);
    sketch.group(0).add_many(hexagons);
    sketch.group(0).set_pen(Pen::pigma_micron_05_black());
    sketch.render().save_default()?;
    Ok(())
}
