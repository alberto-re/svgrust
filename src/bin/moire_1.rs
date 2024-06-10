use anyhow::Result;
use plt::prelude::*;

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::a4(Portrait), Uom::Px, Debug::On);

    let n = 45;
    let start_center = sketch.center() + Vec2::new(100., 100.);
    let end_center = sketch.center();
    let start_radius = 10.0;
    let end_radius = 330.0;

    for i in 0..n {
        let t = i as f64 / n as f64;
        let center = start_center.lerp(end_center, t);
        let radius = start_radius.lerp(end_radius, t);
        sketch.group(0).add(Circle::new(center, radius));
    }

    let start_center = sketch.center() + Vec2::new(100., 105.);
    let end_center = sketch.center();
    let start_radius = 11.0;
    let end_radius = 327.0;

    for i in 0..n {
        let t = i as f64 / n as f64;
        let center = start_center.lerp(end_center, t);
        let radius = start_radius.lerp(end_radius, t);
        sketch.group(1).add(Circle::new(center, radius));
    }

    let start_center = sketch.center() + Vec2::new(105., 100.);
    let end_center = sketch.center();
    let start_radius = 9.0;
    let end_radius = 333.0;

    for i in 0..n {
        let t = i as f64 / n as f64;
        let center = start_center.lerp(end_center, t);
        let radius = start_radius.lerp(end_radius, t);
        sketch.group(2).add(Circle::new(center, radius));
    }

    let start_center = sketch.center() + Vec2::new(94., 102.);
    let end_center = sketch.center();
    let start_radius = 8.0;
    let end_radius = 334.0;

    for i in 0..n {
        let t = i as f64 / n as f64;
        let center = start_center.lerp(end_center, t);
        let radius = start_radius.lerp(end_radius, t);
        sketch.group(3).add(Circle::new(center, radius));
    }

    let start_center = sketch.center() + Vec2::new(103., 95.);
    let end_center = sketch.center();
    let start_radius = 11.0;
    let end_radius = 332.0;

    for i in 0..n {
        let t = i as f64 / n as f64;
        let center = start_center.lerp(end_center, t);
        let radius = start_radius.lerp(end_radius, t);
        sketch.group(4).add(Circle::new(center, radius));
    }

    let start_center = sketch.center() + Vec2::new(104., 98.);
    let end_center = sketch.center();
    let start_radius = 5.0;
    let end_radius = 333.0;

    for i in 0..(n - 1) {
        let t = i as f64 / n as f64;
        let center = start_center.lerp(end_center, t);
        let radius = start_radius.lerp(end_radius, t);
        sketch.group(5).add(Circle::new(center, radius));
    }

    sketch.group(0).set_style(Style::new("yellow", "0.5mm"));
    sketch.group(1).set_style(Style::new("orange", "0.5mm"));
    sketch.group(2).set_style(Style::new("red", "0.5mm"));
    sketch.group(3).set_style(Style::new("green", "0.5mm"));
    sketch.group(4).set_style(Style::new("blue", "0.5mm"));
    sketch.group(5).set_style(Style::new("black", "0.5mm"));
    sketch.render().save_default()?;
    Ok(())
}
