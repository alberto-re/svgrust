// This is the default resolution both for web (CSS) and Inkscape:
// - https://developer.mozilla.org/en-US/docs/Web/CSS/resolution
// - https://inkscape.org/forums/beyond/resolution-change/
const DPI: f64 = 96.0;

#[derive(Clone)]
pub enum Uom {
    In,
    Mm,
    Px,
}

#[derive(Clone)]
pub enum Orientation {
    Landscape,
    Portrait,
}

#[derive(Clone)]
pub struct PageLayout {
    pub width: f64,
    pub height: f64,
    pub uom: Uom,
    pub orientation: Orientation,
    pub style: Option<String>,
}

impl PageLayout {
    pub fn new(width: f64, height: f64, uom: Uom, orientation: Orientation) -> Self {
        let (height, width) = match &orientation {
            Orientation::Portrait => (width, height),
            Orientation::Landscape => (height, width),
        };
        Self {
            height,
            width,
            uom,
            orientation,
            style: None,
        }
    }

    pub fn axidraw_minikit(orientation: Orientation) -> Self {
        Self::new(6.0 * DPI, 4.0 * DPI, Uom::In, orientation)
    }

    pub fn set_style(&mut self, style: &str) -> &Self {
        self.style = Some(style.to_string());
        self
    }
}
