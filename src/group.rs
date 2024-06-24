use crate::prelude::Pen;
use crate::shapes::{Circle, LineString};
use crate::style::Style;
use crate::traits::{Contains, ToShape};
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

    pub fn set_pen(&mut self, pen: Pen) {
        self.style.clone_from(&pen.into());
    }

    pub fn add<T: ToShape>(&mut self, element: T) {
        let shape = element.to_shape();
        match shape {
            Shape::Circle(s) => self.elements.push(Shape::Circle(s)),
            Shape::Arc(s) => self.elements.push(Shape::Arc(s)),
            Shape::Rectangle(s) => self.elements.push(Shape::Rectangle(s)),
            Shape::LineString(s) => self.elements.push(Shape::LineString(s)),
            Shape::Polygon(s) => self.elements.push(Shape::Polygon(s)),
            Shape::MultiPolygon(s) => self.elements.push(Shape::MultiPolygon(s)),
            Shape::Text(s) => self.elements.push(Shape::Text(s)),
        }
    }

    pub fn add_many<T: ToShape>(&mut self, elements: Vec<T>) {
        for element in elements {
            let shape = element.to_shape();
            match shape {
                Shape::Circle(s) => self.elements.push(Shape::Circle(s)),
                Shape::Arc(s) => self.elements.push(Shape::Arc(s)),
                Shape::Rectangle(s) => self.elements.push(Shape::Rectangle(s)),
                Shape::LineString(s) => self.elements.push(Shape::LineString(s)),
                Shape::Polygon(s) => self.elements.push(Shape::Polygon(s)),
                Shape::MultiPolygon(s) => self.elements.push(Shape::MultiPolygon(s)),
                Shape::Text(s) => self.elements.push(Shape::Text(s)),
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

    pub fn circles(&self) -> Vec<Circle> {
        let mut circles = vec![];
        self.elements.iter().for_each(|e| {
            if let Shape::Circle(s) = e {
                circles.push(*s)
            }
        });
        circles
    }

    pub fn split_shape<T: Contains>(&self, bbox: &T) -> (Group, Group) {
        let mut inside = Group::default();
        let mut outside = Group::default();
        self.elements.iter().for_each(|e| match e {
            Shape::Circle(_) => {
                unreachable!();
            }
            Shape::Arc(_) => {
                unreachable!();
            }
            Shape::Rectangle(s) => {
                if bbox.contains(s) {
                    inside.add(s.clone());
                } else {
                    outside.add(s.clone());
                }
            }
            Shape::LineString(s) => {
                if bbox.contains(s) {
                    inside.add(s.clone());
                } else {
                    outside.add(s.clone());
                }
            }
            Shape::Polygon(_) => {
                unreachable!();
            }
            Shape::MultiPolygon(_) => {
                unreachable!();
            }
            Shape::Text(_) => {
                unreachable!();
            }
        });
        (inside, outside)
    }
}

impl Default for Group {
    fn default() -> Self {
        Self::new()
    }
}
