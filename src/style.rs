#[derive(Clone)]
pub struct Style {
    pub stroke: String,
    pub stroke_width: String,
}

impl Style {
    pub fn new(stroke: &str, stroke_width: &str) -> Self {
        Self {
            stroke: stroke.to_string(),
            stroke_width: stroke_width.to_string(),
        }
    }
}

impl Default for Style {
    fn default() -> Self {
        Self::new("black", "0.5mm")
    }
}
