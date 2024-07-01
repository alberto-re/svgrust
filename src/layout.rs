use crate::uom::Uom;

#[derive(Clone)]
pub enum Orientation {
    Landscape,
    Portrait,
}

#[derive(Clone)]
pub struct PageLayout {
    pub width: f64,
    pub height: f64,
    pub orientation: Orientation,
    pub style: Option<String>,
}

// https://www.adobe.com/uk/creativecloud/design/discover/a4-format.html
impl PageLayout {
    pub fn new(width: f64, height: f64, orientation: Orientation) -> Self {
        let (height, width) = match &orientation {
            Orientation::Portrait => (width, height),
            Orientation::Landscape => (height, width),
        };
        Self {
            height,
            width,
            orientation,
            style: None,
        }
    }

    pub fn axidraw_minikit(orientation: Orientation) -> Self {
        Self::new(
            Uom::convert_scalar(6.0, Uom::In, Uom::Px),
            Uom::convert_scalar(4.0, Uom::In, Uom::Px),
            orientation,
        )
    }

    pub fn a6(orientation: Orientation) -> Self {
        Self::new(
            Uom::convert_scalar(5.83, Uom::In, Uom::Px),
            Uom::convert_scalar(4.13, Uom::In, Uom::Px),
            orientation,
        )
    }

    pub fn a5(orientation: Orientation) -> Self {
        Self::new(
            Uom::convert_scalar(8.27, Uom::In, Uom::Px),
            Uom::convert_scalar(5.83, Uom::In, Uom::Px),
            orientation,
        )
    }

    pub fn a4(orientation: Orientation) -> Self {
        Self::new(
            Uom::convert_scalar(11.69, Uom::In, Uom::Px),
            Uom::convert_scalar(8.27, Uom::In, Uom::Px),
            orientation,
        )
    }

    pub fn a3(orientation: Orientation) -> Self {
        Self::new(
            Uom::convert_scalar(16.54, Uom::In, Uom::Px),
            Uom::convert_scalar(11.69, Uom::In, Uom::Px),
            orientation,
        )
    }

    pub fn set_style(&mut self, style: &str) -> &Self {
        self.style = Some(style.to_string());
        self
    }

    pub fn aspect_ratio(&self) -> f64 {
        self.width / self.height
    }
}
