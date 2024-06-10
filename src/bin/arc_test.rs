use anyhow::Result;
use plt::prelude::*;

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::axidraw_minikit(Landscape), Uom::Px, Debug::Off);

    let deg_0 = 0.;
    let deg_45 = 45. * TAU / 360.;
    let deg_90 = 90. * TAU / 360.;
    let deg_135 = 135. * TAU / 360.;
    let deg_180 = 180. * TAU / 360.;
    let deg_225 = 225. * TAU / 360.;
    let deg_270 = 270. * TAU / 360.;
    let deg_315 = 315. * TAU / 360.;
    let deg_360 = 360. * TAU / 360.;

    let radius: f64 = 20.;

    let center = Vec2 { x: 40., y: 40. };
    sketch.group(0).add(Circle::new(center, radius));
    sketch.group(1).add(Arc::new(center, radius, deg_0, deg_90));

    let center = Vec2 { x: 100., y: 40. };
    sketch.group(0).add(Circle::new(center, radius));
    sketch
        .group(1)
        .add(Arc::new(center, radius, deg_90, deg_180));

    let center = Vec2 { x: 160., y: 40. };
    sketch.group(0).add(Circle::new(center, radius));
    sketch
        .group(1)
        .add(Arc::new(center, radius, deg_180, deg_270));

    let center = Vec2 { x: 220., y: 40. };
    sketch.group(0).add(Circle::new(center, radius));
    sketch
        .group(1)
        .add(Arc::new(center, radius, deg_270, deg_360));

    let center = Vec2 { x: 280., y: 40. };
    sketch.group(0).add(Circle::new(center, radius));
    sketch
        .group(1)
        .add(&Arc::new(center, radius, deg_0, deg_180));

    let center = Vec2 { x: 340., y: 40. };
    sketch.group(0).add(Circle::new(center, radius));
    sketch
        .group(1)
        .add(Arc::new(center, radius, deg_180, deg_0));

    let center = Vec2 { x: 400., y: 40. };
    sketch.group(0).add(Circle::new(center, radius));
    sketch
        .group(1)
        .add(Arc::new(center, radius, deg_90, deg_270));

    let center = Vec2 { x: 460., y: 40. };
    sketch.group(0).add(Circle::new(center, radius));
    sketch
        .group(1)
        .add(Arc::new(center, radius, deg_270, deg_90));

    let center = Vec2 { x: 40., y: 100. };
    sketch.group(0).add(Circle::new(center, radius));
    sketch.group(1).add(Arc::new(center, radius, deg_0, deg_45));

    let center = Vec2 { x: 100., y: 100. };
    sketch.group(0).add(Circle::new(center, radius));
    sketch
        .group(1)
        .add(Arc::new(center, radius, deg_45, deg_90));

    let center = Vec2 { x: 160., y: 100. };
    sketch.group(0).add(Circle::new(center, radius));
    sketch
        .group(1)
        .add(Arc::new(center, radius, deg_90, deg_135));

    let center = Vec2 { x: 220., y: 100. };
    sketch.group(0).add(Circle::new(center, radius));
    sketch
        .group(1)
        .add(Arc::new(center, radius, deg_135, deg_180));

    let center = Vec2 { x: 280., y: 100. };
    sketch.group(0).add(Circle::new(center, radius));
    sketch
        .group(1)
        .add(Arc::new(center, radius, deg_180, deg_225));

    let center = Vec2 { x: 340., y: 100. };
    sketch.group(0).add(Circle::new(center, radius));
    sketch
        .group(1)
        .add(Arc::new(center, radius, deg_225, deg_270));

    let center = Vec2 { x: 400., y: 100. };
    sketch.group(0).add(Circle::new(center, radius));
    sketch
        .group(1)
        .add(Arc::new(center, radius, deg_270, deg_315));

    let center = Vec2 { x: 460., y: 100. };
    sketch.group(0).add(Circle::new(center, radius));
    sketch
        .group(1)
        .add(Arc::new(center, radius, deg_315, deg_360));

    let center = Vec2 { x: 40., y: 160. };
    sketch.group(0).add(Circle::new(center, radius));
    sketch
        .group(1)
        .add(Arc::new(center, radius, deg_90, deg_360));

    let center = Vec2 { x: 100., y: 160. };
    sketch.group(0).add(Circle::new(center, radius));
    sketch
        .group(1)
        .add(Arc::new(center, radius, deg_180, deg_90));

    let center = Vec2 { x: 160., y: 160. };
    sketch.group(0).add(Circle::new(center, radius));
    sketch
        .group(1)
        .add(Arc::new(center, radius, deg_180, deg_45));

    sketch.group(0).set_style(Style::new("black", "8.0px"));
    sketch.group(1).set_style(Style::new("orange", "8.0px"));

    sketch.render().save_default()?;
    Ok(())
}
