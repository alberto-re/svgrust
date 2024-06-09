use anyhow::Result;
use plt::prelude::*;

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::a4(Portrait), Uom::Px, Debug::On);

    let mut group1 = Group::new();
    let mut group2 = Group::new();
    let mut group3 = Group::new();
    let mut group4 = Group::new();
    let mut group5 = Group::new();
    let mut group6 = Group::new();

    let n = 45;
    let start_center = sketch.center() + Vec2::new(100., 100.);
    let end_center = sketch.center();
    let start_radius = 10.0;
    let end_radius = 330.0;

    for i in 0..n {
        let t = i as f64 / n as f64;
        let center = start_center.lerp(end_center, t);
        let radius = start_radius.lerp(end_radius, t);
        group1.add(Circle::new(center, radius));
    }

    let start_center = sketch.center() + Vec2::new(100., 105.);
    let end_center = sketch.center();
    let start_radius = 11.0;
    let end_radius = 327.0;

    for i in 0..n {
        let t = i as f64 / n as f64;
        let center = start_center.lerp(end_center, t);
        let radius = start_radius.lerp(end_radius, t);
        group2.add(Circle::new(center, radius));
    }

    let start_center = sketch.center() + Vec2::new(105., 100.);
    let end_center = sketch.center();
    let start_radius = 9.0;
    let end_radius = 333.0;

    for i in 0..n {
        let t = i as f64 / n as f64;
        let center = start_center.lerp(end_center, t);
        let radius = start_radius.lerp(end_radius, t);
        group3.add(Circle::new(center, radius));
    }

    let start_center = sketch.center() + Vec2::new(94., 102.);
    let end_center = sketch.center();
    let start_radius = 8.0;
    let end_radius = 334.0;

    for i in 0..n {
        let t = i as f64 / n as f64;
        let center = start_center.lerp(end_center, t);
        let radius = start_radius.lerp(end_radius, t);
        group4.add(Circle::new(center, radius));
    }

    let start_center = sketch.center() + Vec2::new(103., 95.);
    let end_center = sketch.center();
    let start_radius = 11.0;
    let end_radius = 332.0;

    for i in 0..n {
        let t = i as f64 / n as f64;
        let center = start_center.lerp(end_center, t);
        let radius = start_radius.lerp(end_radius, t);
        group5.add(Circle::new(center, radius));
    }

    let start_center = sketch.center() + Vec2::new(104., 98.);
    let end_center = sketch.center();
    let start_radius = 5.0;
    let end_radius = 333.0;

    for i in 0..(n - 1) {
        let t = i as f64 / n as f64;
        let center = start_center.lerp(end_center, t);
        let radius = start_radius.lerp(end_radius, t);
        group6.add(Circle::new(center, radius));
    }

    sketch.add_group(&group1, &Style::new("orange", "1.0px"));
    sketch.add_group(&group2, &Style::new("pink", "1.0px"));
    sketch.add_group(&group3, &Style::new("red", "1.0px"));
    sketch.add_group(&group4, &Style::new("green", "1.0px"));
    sketch.add_group(&group5, &Style::new("blue", "1.0px"));
    sketch.add_group(&group6, &Style::new("black", "1.0px"));
    sketch.render().save_default()?;
    Ok(())
}
