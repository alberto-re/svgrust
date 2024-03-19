use anyhow::Result;
use plt::layout::Orientation::Landscape;
use plt::layout::PageLayout;
use plt::render::render_svg;
use plt::shapes::LineString;
use plt::traits::packing::CirclePacking;
use plt::vec2::Vec2;
use plt::Group;
use plt::Sketch;
use plt::Style;

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::axidraw_minikit(Landscape));

    let mut lines1 = Group::new();
    let mut lines2 = Group::new();

    let mut circles = vec![];
    let circles = sketch
        .as_rect()
        .to_linestr(true)
        .pack_with_circles(5., &mut circles, 5.);

    for circle in &circles {
        println!("{:?}", circle.center);
    }

    let bboxes = circles
        .iter()
        .map(|c| c.to_linestr(200))
        .collect::<Vec<LineString>>();

    for step in (0..1000).step_by(5) {
        let start = Vec2 {
            x: step as f64,
            y: 0.,
        };
        let end = Vec2 {
            x: (step - 300) as f64,
            y: sketch.as_rect().height,
        };
        let _ = &LineString::new(vec![start, end])
            .clip_many(&bboxes, false)
            .iter()
            .for_each(|l| lines1.add_lstr(&l.clone()));
        let start = Vec2 {
            x: step as f64 + 3.,
            y: 0.,
        };
        let end = Vec2 {
            x: (step - 303) as f64,
            y: sketch.as_rect().height,
        };
        let _ = &LineString::new(vec![start, end])
            .clip_many(&bboxes, false)
            .iter()
            .for_each(|l| lines2.add_lstr(&l.clone()));
    }

    sketch.add_group(&lines1, &Style::new("#093c80", "0.45mm"));
    sketch.add_group(&lines2, &Style::new("#a32784", "0.45mm"));
    render_svg(&sketch, "./samples/004.svg")?;
    Ok(())
}
