pub mod packing;

use std::f64::consts::TAU;

use crate::shapes::{Circle, LineStr, Rect};
use crate::vec2::Vec2;
use geo::algorithm::Rotate as GeoRotate;
use geo::coord;
use geo::Coord;
use geo::CoordsIter;
use geo::Polygon;
use rand::rngs::StdRng;
use rand::Rng;

pub trait Scale<T> {
    fn scale_perc(&self, perc: f64) -> T;
    fn scale_unit(&self, unit: f64) -> T;
}

pub trait Sample {
    fn sample_uniform(&self, rng: &mut StdRng, n: u64) -> Vec<Vec2>;
}

pub trait Upsample {
    fn upsample(&self, factor: u64) -> Self;
}

pub trait Chaikin {
    fn chaikin(&self, iterations: u64, wrap: bool) -> Self;
}

pub trait Rotate {
    fn rotate(&self, radians: f64) -> Self;
}

pub trait Centroid {
    fn centroid(&self) -> Vec2;
}

pub trait Contains {
    fn contains<T: Centroid>(&self, coord: &T) -> bool;
}

impl Centroid for LineStr {
    fn centroid(&self) -> Vec2 {
        // TODO: we must prevent division by zero
        let mut xsum: f64 = 0.;
        let mut ysum: f64 = 0.;
        self.points.iter().for_each(|p| {
            xsum += p.x;
            ysum += p.y;
        });
        Vec2 {
            x: xsum / self.points.len() as f64,
            y: ysum / self.points.len() as f64,
        }
    }
}

impl Upsample for LineStr {
    fn upsample(&self, factor: u64) -> Self {
        // TODO: add wrap bool argument like Chaikin
        let mut points = self.points.clone();
        (0..factor).for_each(|_| {
            let mut upsampled = vec![];
            for i in 1..points.len() {
                upsampled.push(points[i - 1]);
                let middle_point = Vec2 {
                    x: (points[i - 1].x + points[i].x) * 0.5,
                    y: (points[i - 1].y + points[i].y) * 0.5,
                };
                upsampled.push(middle_point);
            }
            upsampled.push(Vec2 {
                x: (points[0].x + points[points.len() - 1].x) * 0.5,
                y: (points[0].y + points[points.len() - 1].y) * 0.5,
            });
            points = upsampled.clone();
        });
        LineStr::new(points)
    }
}

impl Chaikin for LineStr {
    fn chaikin(&self, iterations: u64, closed: bool) -> Self {
        let mut points = self.points.clone();
        (0..iterations).for_each(|_| {
            let mut smoothed = vec![];
            for i in 1..points.len() {
                smoothed.push(Vec2 {
                    x: points[i - 1].x * 0.75 + points[i].x * 0.25,
                    y: points[i - 1].y * 0.75 + points[i].y * 0.25,
                });
                smoothed.push(Vec2 {
                    x: points[i - 1].x * 0.25 + points[i].x * 0.75,
                    y: points[i - 1].y * 0.25 + points[i].y * 0.75,
                });
            }
            if closed {
                smoothed.push(Vec2 {
                    x: points[points.len() - 1].x * 0.75 + points[1].x * 0.25,
                    y: points[points.len() - 1].y * 0.75 + points[1].y * 0.25,
                });
                smoothed.push(Vec2 {
                    x: points[points.len() - 1].x * 0.25 + points[1].x * 0.75,
                    y: points[points.len() - 1].y * 0.25 + points[1].y * 0.75,
                });
            }
            points = smoothed.clone();
        });
        LineStr::new(points)
    }
}

impl Rotate for LineStr {
    // TODO: this should be a polygon method.
    // For simplicity here we assume the linestring is closed
    // and represents a polygon.
    // TODO: add direction (clockwise, anti-clockwise) of rotation
    // TODO: implement from scratch?
    fn rotate(&self, radians: f64) -> Self {
        let poly: Polygon = geo::Polygon::new(
            geo::LineString::new(
                self.points
                    .clone()
                    .iter()
                    .map(|v| coord! {x: v.x, y: v.y})
                    .collect::<Vec<Coord>>(),
            ),
            vec![],
        );
        let degrees = radians * 180.0 / TAU;
        let poly = poly.rotate_around_centroid(degrees);
        LineStr::new(
            poly.exterior()
                .points()
                .map(|p| p.coords_iter().nth(0).unwrap())
                .map(|p| Vec2 { x: p.x, y: p.y })
                .collect::<Vec<Vec2>>(),
        )
    }
}

impl Scale<Rect> for Rect {
    fn scale_perc(&self, perc: f64) -> Rect {
        Rect::new(
            Vec2 {
                x: self.xy.x + self.width * ((1. - perc) / 2.),
                y: self.xy.y + self.height * ((1. - perc) / 2.),
            },
            self.width * perc,
            self.height * perc,
        )
    }

    fn scale_unit(&self, unit: f64) -> Rect {
        Rect::new(
            Vec2 {
                x: self.xy.x + unit / 2.,
                y: self.xy.y + unit / 2.,
            },
            self.width - unit,
            self.height - unit,
        )
    }
}

impl Contains for Rect {
    fn contains<T: Centroid>(&self, shape: &T) -> bool {
        (shape.centroid().x > self.xy.x && shape.centroid().x < self.xy.x + self.width)
            && (shape.centroid().y > self.xy.y && shape.centroid().y < self.xy.y + self.height)
    }
}

impl Sample for Rect {
    fn sample_uniform(&self, rng: &mut StdRng, n: u64) -> Vec<Vec2> {
        let mut samples = vec![];
        (0..n).for_each(|_| {
            let x = rng.gen::<f64>() * self.width + self.xy.x;
            let y = rng.gen::<f64>() * self.height + self.xy.y;
            samples.push(Vec2 { x, y });
        });
        samples
    }
}

impl Centroid for Rect {
    fn centroid(&self) -> Vec2 {
        Vec2 {
            x: self.xy.x + self.width * 0.5,
            y: self.xy.y + self.height * 0.5,
        }
    }
}

impl Sample for Circle {
    fn sample_uniform(&self, rng: &mut StdRng, n: u64) -> Vec<Vec2> {
        let mut samples = vec![];
        (0..n).for_each(|_| {
            let r_sqrt = (rng.gen::<f64>() * self.radius * self.radius).sqrt();
            let angle = rng.gen::<f64>() * TAU;
            let x = r_sqrt * angle.cos() + self.center.x;
            let y = r_sqrt * angle.sin() + self.center.y;
            samples.push(Vec2 { x, y });
        });
        samples
    }
}

impl Centroid for Circle {
    fn centroid(&self) -> Vec2 {
        self.center
    }
}

impl Contains for Circle {
    fn contains<T: Centroid>(&self, shape: &T) -> bool {
        self.center.euclidean_distance(&shape.centroid()) < self.radius
    }
}

impl Scale<Circle> for Circle {
    fn scale_perc(&self, perc: f64) -> Circle {
        Circle::new(self.center, self.radius * perc)
    }

    fn scale_unit(&self, unit: f64) -> Circle {
        Circle::new(self.center, self.radius - unit)
    }
}

impl Centroid for Vec2 {
    fn centroid(&self) -> Vec2 {
        *self
    }
}
