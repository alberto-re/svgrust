use crate::prelude::Vec2;

// This is the default resolution both for web (CSS) and Inkscape:
// - https://developer.mozilla.org/en-US/docs/Web/CSS/resolution
// - https://inkscape.org/forums/beyond/resolution-change/
const DPI: f64 = 96.0;
// Same resolution but expressed in Dots per Millimeter
const DPM: f64 = 3.779527;

#[derive(Clone, Copy)]
pub enum Uom {
    In,
    Mm,
    Px,
}

impl Uom {
    pub fn convert_scalar(n: f64, from: Uom, to: Uom) -> f64 {
        match (from, to) {
            (Uom::Mm, Uom::Px) => n * DPM,
            (Uom::In, Uom::Px) => n * DPI,
            (Uom::Px, Uom::Px) => n,
            (Uom::Px, Uom::Mm) => n / DPM,
            (Uom::Px, Uom::In) => n / DPI,
            (_, _) => unimplemented!(),
        }
    }

    pub fn convert_vec2(v: Vec2, from: Uom, to: Uom) -> Vec2 {
        match (from, to) {
            (Uom::Mm, Uom::Px) => Vec2::new(v.x * DPM, v.y * DPM),
            (Uom::In, Uom::Px) => Vec2::new(v.x * DPI, v.y * DPI),
            (Uom::Px, Uom::Px) => v,
            (_, _) => unimplemented!(),
        }
    }
}
