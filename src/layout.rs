use geo::coord;
use geo::Coord;

// This is the default resolution both for web (CSS) and Inkscape:
// - https://developer.mozilla.org/en-US/docs/Web/CSS/resolution
// - https://inkscape.org/forums/beyond/resolution-change/
const DPI: f64 = 96.0;

pub enum Uom {
    In,
    Mm,
    Px,
}

pub enum Orientation {
    Landscape,
    Portrait,
}

pub struct PageLayout {
    pub width: f64,
    pub height: f64,
    pub uom: Uom,
    pub orientation: Orientation,
    pub style: Option<String>,
}

impl PageLayout {
    pub fn new(width: f64, height: f64, uom: Uom, orientation: Orientation) -> Self {
        Self {
            width,
            height,
            uom,
            orientation,
            style: None,
        }
    }

    pub fn axidraw_minikit(orientation: Orientation) -> Self {
        Self::new(6.0 * DPI, 4.0 * DPI, Uom::In, orientation)
    }

    pub fn set_style(&mut self, style: String) -> &Self {
        self.style = Some(style);
        self
    }

    pub fn center(&self) -> Coord {
        coord! { x: self.width / 2., y: self.height / 2. }
    }

    pub fn shortest_side(&self) -> f64 {
        f64::min(self.width, self.height)
    }
}
