use anyhow::Result;
use geo::coord;
use geo::Coord;
use noise::NoiseFn;
use noise::Perlin;
use plt::layout::Orientation::Portrait;
use plt::layout::PageLayout;
use plt::render::render_svg;
use plt::shapes::Centroid;
use plt::shapes::Circle;
use plt::shapes::LineString;
use plt::shapes::Scale;
use plt::Group;
use plt::Shape;
use plt::Sketch;
use plt::Style;
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

// https://openprocessing.org/sketch/2008342

// Main ideas explored here:
// - plots with a lot of empty spaces, in general, looks better on dark paper
// - repetition is good, but a bit of unexpected makes the plot more interesting to the eye

fn main() -> Result<()> {
    let mut sketch = Sketch::new(PageLayout::axidraw_minikit(Portrait));
    let mut layer = Group::new();
    let enclosing = sketch.as_rect();

    let cells = enclosing.grid(60, 40);
    let mut rng = StdRng::seed_from_u64(42);
    cells.iter().for_each(|c| {
        let mut points: Vec<Coord> = vec![];
        if rng.gen::<f64>() < 0.5 {
            points.push(coord! { x: c.xy.x, y: c.xy.y });
            points.push(coord! { x: c.xy.x + c.width, y: c.xy.y + c.height });
        } else {
            points.push(coord! { x: c.xy.x + c.width, y: c.xy.y });
            points.push(coord! { x: c.xy.x, y: c.xy.y + c.height });
        }
        layer.add_lstr(&LineString::new(points).add_vec(enclosing.xy));
    });

    let circle = Circle::new(sketch.centroid(), enclosing.scale(0.98).min_len() / 2.);

    let (inner, _) = layer.split_shape(&circle.scale(0.9));
    let mut inner1 = Group::new();
    let mut inner2 = Group::new();

    let perlin = Perlin::new(38);

    inner.elements.iter().for_each(|e| {
        if let Shape::LineString(s) = e {
            let val = perlin.get([s.centroid().x * 0.015, s.centroid().y * 0.02]);
            if val < 0.5 && rng.gen::<f64>() < 0.85 {
                inner1.elements.push(e.clone());
            } else {
                inner2.elements.push(e.clone());
            }
        }
    });

    // inner1.add_circle(&circle);
    // inner1.add_circle(&circle.scale(0.995));
    inner1.add_circle(&circle.scale(0.990));
    inner1.add_circle(&circle.scale(0.985));
    inner1.add_circle(&circle.scale(0.980));
    inner1.add_circle(&circle.scale(0.975));
    inner1.add_circle(&circle.scale(0.970));
    inner1.add_circle(&circle.scale(0.965));
    inner1.add_circle(&circle.scale(0.960));

    let mut outer = Group::new();
    inner1.elements.iter().for_each(|e| {
        if let Shape::LineString(s) = e {
            outer.add_lstr(&s.clone().add_vec(coord! {x: 1., y: 1.}));
        }
    });
    // sketch.add_group(&outer, &Style::new("red", "1.0px"));
    sketch.add_group(&inner1, &Style::new("black", "1.0px"));

    render_svg(&sketch, "/Users/are/Desktop/prova.svg")?;
    Ok(())
}
