use anyhow::Result;
use plt::prelude::*;

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::a4(Portrait), Uom::Mm, true);
    let mut group = Group::new();

    for i in 0..=10 {
        let x = 1.;
        let y = 0. + 10. * i as f64;
        let p1 = Vec2::new(x, y);
        let p2 = p1 + Vec2::new(y, 0.);
        group.add(LineString::line(p1, p2));
        group.add(Text::new(Vec2::new(x, y - 1.), &format!("{y}mm")));
    }

    for i in 0..=10 {
        group.add(Rect::square_with_center(
            Vec2::new(160., 55.),
            10. * i as f64,
        ));
    }

    sketch.add_group(&group, &Style::new("black", "0.2mm"));
    sketch.render().save_default()?;
    Ok(())
}
