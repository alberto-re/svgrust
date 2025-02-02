use crate::prelude::Pen;
use crate::shapes::linestring::LineString;
use crate::style::Style;
use crate::traits::ToShape;
use crate::Shape;

#[derive(Clone)]
pub struct Group {
    pub elements: Vec<Shape>,
    pub style: Style,
}

impl Group {
    pub fn new() -> Self {
        Self {
            elements: vec![],
            style: Style::default(),
        }
    }

    pub fn set_style(&mut self, style: Style) {
        self.style.clone_from(&style);
    }

    pub fn set_pen(&mut self, pen: &Pen) {
        self.style.clone_from(&pen.into());
    }

    pub fn add<T: ToShape>(&mut self, element: T) {
        let shape = element.to_shape();
        match shape {
            Shape::Circle(s) => self.elements.push(Shape::Circle(s)),
            Shape::Rectangle(s) => self.elements.push(Shape::Rectangle(s)),
            Shape::Hexagon(s) => self.elements.push(Shape::Hexagon(s)),
            Shape::LineString(s) => self.elements.push(Shape::LineString(s)),
            Shape::Polygon(s) => self.elements.push(Shape::Polygon(s)),
            Shape::Text(s) => self.elements.push(Shape::Text(s)),
            Shape::Triangle(s) => self.elements.push(Shape::Triangle(s)),
        }
    }

    pub fn add_many<T: ToShape>(&mut self, elements: Vec<T>) {
        for element in elements {
            let shape = element.to_shape();
            match shape {
                Shape::Circle(s) => self.elements.push(Shape::Circle(s)),
                Shape::Rectangle(s) => self.elements.push(Shape::Rectangle(s)),
                Shape::Hexagon(s) => self.elements.push(Shape::Hexagon(s)),
                Shape::LineString(s) => self.elements.push(Shape::LineString(s)),
                Shape::Polygon(s) => self.elements.push(Shape::Polygon(s)),
                Shape::Text(s) => self.elements.push(Shape::Text(s)),
                Shape::Triangle(s) => self.elements.push(Shape::Triangle(s)),
            }
        }
    }

    pub fn linestrings(&self) -> Vec<LineString> {
        let mut lstrs = vec![];
        self.elements.iter().for_each(|e| {
            if let Shape::LineString(s) = e {
                lstrs.push(s.clone())
            }
        });
        lstrs
    }
}

impl Default for Group {
    fn default() -> Self {
        Self::new()
    }
}
