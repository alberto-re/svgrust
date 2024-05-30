use std::time::Instant;

use crate::render::render_svg;
use crate::shapes::Rect;
use crate::traits::Centroid;
use crate::uom::Uom;
use crate::vec2::Vec2;
use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use svg::Document;

use crate::layout::PageLayout;
use crate::Group;
use crate::Style;

pub enum Debug {
    Off,
    On,
}

/// A high-level representation of a plotter drawing
pub struct Sketch {
    pub layout: PageLayout,
    pub groups: Vec<(Group, Style)>,
    pub uom: Uom,
    doc: Document,
    debug: Debug,
    created: Instant,
}

impl Sketch {
    /// Construct a new sketch
    pub fn new(layout: &PageLayout, uom: Uom, debug: Debug) -> Self {
        Self {
            layout: layout.clone(),
            groups: vec![],
            doc: Document::new(),
            uom,
            debug,
            created: Instant::now(),
        }
    }

    /// Add a group to the sketch
    pub fn add_group(&mut self, layer: &Group, style: &Style) {
        self.groups.push((layer.clone(), style.clone()));
    }

    /// Return a Rect representing the sketch area
    pub fn as_rect(&self) -> Rect {
        Rect::new(
            Vec2 { x: 0., y: 0. },
            Uom::convert_scalar(self.layout.width, Uom::Px, self.uom),
            Uom::convert_scalar(self.layout.height, Uom::Px, self.uom),
        )
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
        let new_to_render = self.created.elapsed();
        println!(
            "Time elapsed from new(): {} milliseconds",
            new_to_render.as_millis()
        );
        if matches!(self.debug, Debug::On) {
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
