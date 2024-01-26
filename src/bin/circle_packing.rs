use anyhow::Result;
use plt::layout::Orientation::Landscape;
use plt::layout::PageLayout;
use plt::render::render_svg;
use plt::shapes::Circle;
use plt::shapes::Rect;
use plt::shapes::Sample;
use plt::Group;
use plt::Sketch;
use plt::Style;
use rand::Rng;

const MIN_R: f64 = 1.;
const MIN_DIST: f64 = 1.;
const CIRCLES_N: usize = 200;

fn place_circles(enclosing_circle: &Circle) -> Vec<Circle> {
    let mut circles: Vec<Circle> = vec![];

    while circles.len() < CIRCLES_N {
        let candidate = Circle::new(*enclosing_circle.sample_uniform(1).first().unwrap(), MIN_R);
        if circles
            .iter()
            .filter(|c| c.dist(&candidate) < MIN_DIST)
            .count()
            == 0
        {
            circles.push(candidate.clone());
        }
    }
    circles
}

fn grow_circles(circles: &mut [Circle]) {
    let mut rng = rand::thread_rng();
    for _ in 0..2000 {
        let chosen = rng.gen_range(0..circles.len());
        if circles
            .iter()
            .filter(|c| *c != &circles[chosen] && c.dist(&circles[chosen]) < MIN_DIST + 1.)
            .count()
            == 0
        {
            circles[chosen].radius += 1.;
        }
    }
}

fn main() -> Result<()> {
    let mut sketch = Sketch::new(PageLayout::axidraw_minikit(Landscape));

    let enclosing_circle = Circle::new(sketch.centroid(), sketch.as_rect().min_len() / 2.5);
    let mut circles = place_circles(&enclosing_circle);
    grow_circles(&mut circles);

    let rect = Rect::new(sketch.centroid(), 100., 100.);

    let mut layer1 = Group::new().set_style(Style::new("blue", "2px"));
    for c in circles.iter() {
        layer1.add_circle(c);
    }

    let mut layer2 = Group::new().set_style(Style::new("red", "3px"));
    layer2.add_rect(&rect);

    sketch.add_group(&layer1);
    sketch.add_group(&layer2);

    render_svg(&sketch, "/Users/are/Desktop/shapes.svg")?;
    Ok(())
}
