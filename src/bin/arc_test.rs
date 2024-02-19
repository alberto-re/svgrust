use std::f64::consts::TAU;

use anyhow::Result;
use geo::coord;
use plt::layout::Orientation::Landscape;
use plt::layout::PageLayout;
use plt::render::render_svg;
use plt::shapes::Arc;
use plt::shapes::Circle;
use plt::Group;
use plt::Sketch;
use plt::Style;

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::axidraw_minikit(Landscape));
    let mut arcs = Group::new();
    let mut circles = Group::new();

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

    let center = coord! { x: 40., y: 40. };
    circles.add_circle(&Circle::new(center, radius));
    arcs.add_arc(&Arc::new(center, radius, deg_0, deg_90));

    let center = coord! { x: 100., y: 40. };
    circles.add_circle(&Circle::new(center, radius));
    arcs.add_arc(&Arc::new(center, radius, deg_90, deg_180));

    let center = coord! { x: 160., y: 40. };
    circles.add_circle(&Circle::new(center, radius));
    arcs.add_arc(&Arc::new(center, radius, deg_180, deg_270));

    let center = coord! { x: 220., y: 40. };
    circles.add_circle(&Circle::new(center, radius));
    arcs.add_arc(&Arc::new(center, radius, deg_270, deg_360));

    let center = coord! { x: 280., y: 40. };
    circles.add_circle(&Circle::new(center, radius));
    arcs.add_arc(&Arc::new(center, radius, deg_0, deg_180));

    let center = coord! { x: 340., y: 40. };
    circles.add_circle(&Circle::new(center, radius));
    arcs.add_arc(&Arc::new(center, radius, deg_180, deg_0));

    let center = coord! { x: 400., y: 40. };
    circles.add_circle(&Circle::new(center, radius));
    arcs.add_arc(&Arc::new(center, radius, deg_90, deg_270));

    let center = coord! { x: 460., y: 40. };
    circles.add_circle(&Circle::new(center, radius));
    arcs.add_arc(&Arc::new(center, radius, deg_270, deg_90));

    let center = coord! { x: 40., y: 100. };
    circles.add_circle(&Circle::new(center, radius));
    arcs.add_arc(&Arc::new(center, radius, deg_0, deg_45));

    let center = coord! { x: 100., y: 100. };
    circles.add_circle(&Circle::new(center, radius));
    arcs.add_arc(&Arc::new(center, radius, deg_45, deg_90));

    let center = coord! { x: 160., y: 100. };
    circles.add_circle(&Circle::new(center, radius));
    arcs.add_arc(&Arc::new(center, radius, deg_90, deg_135));

    let center = coord! { x: 220., y: 100. };
    circles.add_circle(&Circle::new(center, radius));
    arcs.add_arc(&Arc::new(center, radius, deg_135, deg_180));

    let center = coord! { x: 280., y: 100. };
    circles.add_circle(&Circle::new(center, radius));
    arcs.add_arc(&Arc::new(center, radius, deg_180, deg_225));

    let center = coord! { x: 340., y: 100. };
    circles.add_circle(&Circle::new(center, radius));
    arcs.add_arc(&Arc::new(center, radius, deg_225, deg_270));

    let center = coord! { x: 400., y: 100. };
    circles.add_circle(&Circle::new(center, radius));
    arcs.add_arc(&Arc::new(center, radius, deg_270, deg_315));

    let center = coord! { x: 460., y: 100. };
    circles.add_circle(&Circle::new(center, radius));
    arcs.add_arc(&Arc::new(center, radius, deg_315, deg_360));

    let center = coord! { x: 40., y: 160. };
    circles.add_circle(&Circle::new(center, radius));
    arcs.add_arc(&Arc::new(center, radius, deg_90, deg_360));

    let center = coord! { x: 100., y: 160. };
    circles.add_circle(&Circle::new(center, radius));
    arcs.add_arc(&Arc::new(center, radius, deg_180, deg_90));

    let center = coord! { x: 160., y: 160. };
    circles.add_circle(&Circle::new(center, radius));
    arcs.add_arc(&Arc::new(center, radius, deg_180, deg_45));

    sketch.add_group(&circles, &Style::new("black", "5.0px"));
    sketch.add_group(&arcs, &Style::new("red", "6.0px"));

    render_svg(&sketch, "./samples/arc_test.svg")?;
    Ok(())
}
