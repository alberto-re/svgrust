use crate::render::render_svg;
use crate::shapes::Rect;
use crate::traits::Centroid;
use crate::vec2::Vec2;
use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use svg::Document;

use crate::layout::PageLayout;
use crate::Group;
use crate::Style;

/// A high-level representation of a plotter drawing
pub struct Sketch {
    pub layout: PageLayout,
    pub groups: Vec<(Group, Style)>,
    doc: Document,
    debug: bool,
}

impl Sketch {
    /// Construct a new sketch
    pub fn new(layout: &PageLayout, debug: bool) -> Self {
        Self {
            layout: layout.clone(),
            groups: vec![],
            doc: Document::new(),
            debug,
        }
    }

    /// Add a group to the sketch
    pub fn add_group(&mut self, layer: &Group, style: &Style) {
        self.groups.push((layer.clone(), style.clone()));
    }

    /// Return a Rect representing the sketch area
    pub fn as_rect(&self) -> Rect {
        Rect::new(Vec2 { x: 0., y: 0. }, self.layout.width, self.layout.height)
    }

    /// Return a 2d vector representing the center point of the sketch
    pub fn center(&self) -> Vec2 {
        self.as_rect().centroid()
    }

    /// Return the width of the sketch
    pub fn width(&self) -> f64 {
        self.as_rect().width
    }

    /// Return the height of the sketch
    pub fn height(&self) -> f64 {
        self.as_rect().height
    }

    pub fn top_middle(&self, margin: f64) -> Vec2 {
        Vec2 {
            x: self.as_rect().width / 2.,
            y: margin,
        }
    }

    pub fn bottom_middle(&self, margin: f64) -> Vec2 {
        Vec2 {
            x: self.as_rect().width / 2.,
            y: self.as_rect().height - margin,
        }
    }

    pub fn bottom_left(&self, margin: f64) -> Vec2 {
        Vec2 {
            x: margin,
            y: self.as_rect().height - margin,
        }
    }

    pub fn left_middle(&self, margin: f64) -> Vec2 {
        Vec2 {
            x: margin,
            y: self.as_rect().height / 2.,
        }
    }

    pub fn right_middle(&self, margin: f64) -> Vec2 {
        Vec2 {
            x: self.as_rect().width - margin,
            y: self.as_rect().height / 2.,
        }
    }

    pub fn min_len(&self) -> f64 {
        f64::min(self.width(), self.height())
    }

    pub fn render(&mut self) -> &Self {
        if self.debug {
            let mut debug = Group::new();
            debug.add(self.as_rect());
            self.add_group(&debug, &Style::new("black", "0.2mm"))
        }
        self.doc = render_svg(self);
        self
    }

    pub fn save_to(&self, path: &str) -> Result<()> {
        svg::save(path, &self.doc).context("Cannot save SVG file")?;
        println!("Output written in '{path}'");
        Ok(())
    }

    pub fn save_default(&self) -> Result<()> {
        let bin_name = std::env::current_exe()?
            .file_name()
            .ok_or_else(|| anyhow!("Cannot determine binary filename"))?
            .to_owned()
            .into_string()
            // TODO: get rid of this unwrap()
            .unwrap();
        let path = format!("samples/{bin_name}.svg");
        self.save_to(&path)?;
        Ok(())
    }
}
