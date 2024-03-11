use anyhow::Result;
use plt::layout::Orientation::Landscape;
use plt::layout::PageLayout;
use plt::render::render_svg;
use plt::shapes::Circle;
use plt::shapes::LineStr;
use plt::traits::Scale;
use plt::vec2::Vec2;
use plt::Group;
use plt::Sketch;
use plt::Style;

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::axidraw_minikit(Landscape));

    let mut lines1 = Group::new();
    let mut lines2 = Group::new();
    let mut lines3 = Group::new();

    let circle = Circle::new(
        sketch.center(),
        sketch.as_rect().scale_perc(0.98).min_len() / 2.,
    );
    let bbox = circle.to_linestr(200);

    for step in (0..1000).step_by(5) {
        let start = Vec2 {
            x: step as f64,
            y: 0.,
        };
        let end = Vec2 {
            x: (step - 300) as f64,
            y: sketch.as_rect().height,
        };
        let _ = &LineStr::new(vec![start, end])
            .clip(&bbox, false)
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
        let _ = &LineStr::new(vec![start, end])
            .clip(&bbox, false)
            .iter()
            .for_each(|l| lines2.add_lstr(&l.clone()));
    }

    let bbox = circle.scale_perc(1.10).to_linestr(200);
    for step in (0..1000).step_by(5) {
        LineStr::new(vec![
            Vec2 {
                x: 0.,
                y: step as f64,
            },
            Vec2 {
                x: sketch.as_rect().width,
                y: step as f64,
            },
        ])
        .clip(&bbox, true)
        .iter()
        .for_each(|l| {
            lines3.add_lstr(&l.clone());
        });
    }

    sketch.add_group(&lines1, &Style::new("#093c80", "0.45mm"));
    sketch.add_group(&lines2, &Style::new("#a32784", "0.45mm"));
    //sketch.add_group(&lines3, &Style::new("black", "0.45mm"));
    render_svg(&sketch, "./samples/003.svg")?;
    Ok(())
}
