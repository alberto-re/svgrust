use anyhow::Result;
use plt::render_svg;
use plt::shapes::Circle;
use plt::shapes::Rectangle;
use plt::Orientation::Landscape;
use plt::PageLayout;
use plt::Sketch;
use rand::Rng;

const MIN_R: f64 = 1.;
const MIN_DIST: f64 = 1.;
const CIRCLES_N: usize = 200;

fn place_circles(enclosing_circle: &Circle) -> Vec<Circle> {
    let mut circles: Vec<Circle> = vec![];

    while circles.len() < CIRCLES_N {
        let candidate = Circle::new(enclosing_circle.rnd_uniform(), MIN_R);
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
    let enclosing_circle = Circle::new(sketch.layout.center(), sketch.layout.shortest_side() / 2.5);
    let rect = Rectangle::new(sketch.layout.center(), 100., 100.);
    let mut circles = place_circles(&enclosing_circle);
    grow_circles(&mut circles);
    for c in circles.iter() {
        sketch.add_circle(c.clone());
    }
    sketch.add_rect(rect);
    render_svg(&sketch, "/Users/are/Desktop/shapes.svg")?;
    Ok(())
}
