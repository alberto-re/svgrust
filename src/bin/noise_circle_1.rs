use anyhow::Result;
use noise::NoiseFn;
use noise::Perlin;
use plt::angle::Angle;
use plt::layout::Orientation::Landscape;
use plt::layout::PageLayout;
use plt::render::render_svg;
use plt::shapes::LineStr;
use plt::shapes::Rect;
use plt::traits::Rotate;
use plt::vec2::Vec2;
use plt::Group;
use plt::Sketch;
use plt::Style;

fn main() -> Result<()> {
    let mut sketch = Sketch::new(&PageLayout::axidraw_minikit(Landscape));
    let perlin = Perlin::new(37);
    let noise_ratio: f64 = 0.0002;
    let mut group = Group::new();
    let mut angle = 0.;
    let mut polygons: Vec<LineStr> = vec![];
    for x in (60..500).step_by(3) {
        let rect = &Rect::new(Vec2::new(x as f64, 60.), 50., 50.);
        let polygon = rect.to_linestr(true);
        let noise_value = perlin.get([x as f64 * noise_ratio, x as f64]);
        angle += noise_value;
        let polygon = polygon.rotate(Angle::from_radians(angle));
        polygons.push(polygon);
    }

    let mut clipped: Vec<LineStr> = vec![];

    for (i, polygon) in polygons.iter().enumerate() {
        let segments = polygon.clip_many(&polygons[i + 1..], true);
        segments.iter().for_each(|s| clipped.push(s.clone()));
    }

    clipped.iter().for_each(|p| group.add_lstr(&p.clone()));

    sketch.add_group(&group, &Style::new("black", "0.2mm"));
    render_svg(&sketch, "./samples/noise_circle_1.svg")?;
    Ok(())
}
