pub mod packing;

use std::f64::consts::TAU;

use crate::angle::Angle;
use crate::shapes::{Arc, Circle, LineString, MultiPolygon, Polygon, Rect};
use crate::vec2::Vec2;
use crate::Shape;
use geo::algorithm::Rotate as GeoRotate;
use geo::coord;
use geo::Contains as GeoContains;
use geo::Coord;
use geo::CoordsIter;
use rand::rngs::StdRng;
use rand::Rng;

pub trait Scale<T> {
    fn scale_perc(&self, perc: f64) -> T;
    fn scale_unit(&self, unit: f64) -> T;
}

pub trait Clip {
    fn clip(&self, bbox: &Polygon) -> Vec<LineString>;
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
    fn rotate(&self, angle: Angle) -> Self;
}

pub trait Centroid {
    fn centroid(&self) -> Vec2;
}

pub trait Contains {
    fn contains<T: Centroid>(&self, coord: &T) -> bool;
}

pub trait Translate {
    fn translate(&self, displacement: Vec2) -> Self;
}

pub trait ToGeoLineString {
    fn to_geo_linestring(&self) -> geo::LineString;
    fn to_geo_multilinestring(&self) -> geo::MultiLineString;
}

pub trait ToShape {
    fn to_shape(&self) -> Shape;
}

pub trait Triangulate {
    fn triangulate(&self) -> Vec<Polygon>;
}

impl Centroid for LineString {
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

impl Upsample for LineString {
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
        LineString::new(points)
    }
}

impl Chaikin for LineString {
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
        LineString::new(points)
    }
}

impl Rotate for LineString {
    // TODO: this should be a polygon method.
    // For simplicity here we assume the linestring is closed
    // and represents a polygon.
    // TODO: add direction (clockwise, anti-clockwise) of rotation
    // TODO: implement from scratch?
    fn rotate(&self, angle: Angle) -> Self {
        let poly: geo::Polygon = geo::Polygon::new(
            geo::LineString::new(
                self.points
                    .clone()
                    .iter()
                    .map(|v| coord! {x: v.x, y: v.y})
                    .collect::<Vec<Coord>>(),
            ),
            vec![],
        );
        let degrees = angle.to_radians() * 180.0 / TAU;
        let poly = poly.rotate_around_centroid(degrees);
        LineString::new(
            poly.exterior()
                .points()
                .map(|p| p.coords_iter().nth(0).unwrap())
                .map(|p| Vec2 { x: p.x, y: p.y })
                .collect::<Vec<Vec2>>(),
        )
    }
}

impl ToGeoLineString for LineString {
    fn to_geo_linestring(&self) -> geo::LineString {
        geo::LineString::new(
            self.points
                .clone()
                .iter()
                .map(|v| coord! {x: v.x, y:v.y})
                .collect::<Vec<Coord>>(),
        )
    }

    fn to_geo_multilinestring(&self) -> geo::MultiLineString {
        geo::MultiLineString::new(vec![self.to_geo_linestring()])
    }
}

impl ToShape for LineString {
    fn to_shape(&self) -> Shape {
        Shape::LineString(self.clone())
    }
}

impl ToShape for &LineString {
    fn to_shape(&self) -> Shape {
        Shape::LineString(LineString::new(self.points.clone()))
    }
}

impl Rotate for Polygon {
    // TODO: add direction (clockwise, anti-clockwise) of rotation
    // TODO: implement from scratch?
    fn rotate(&self, angle: Angle) -> Self {
        let poly: geo::Polygon = geo::Polygon::new(
            geo::LineString::new(
                self.points
                    .clone()
                    .iter()
                    .map(|v| coord! {x: v.x, y: v.y})
                    .collect::<Vec<Coord>>(),
            ),
            vec![],
        );
        let degrees = angle.to_radians() * 180.0 / TAU;
        let poly = poly.rotate_around_centroid(degrees);
        Polygon::new(
            poly.exterior()
                .points()
                .map(|p| p.coords_iter().nth(0).unwrap())
                .map(|p| Vec2 { x: p.x, y: p.y })
                .collect::<Vec<Vec2>>(),
        )
    }
}

impl ToGeoLineString for Polygon {
    fn to_geo_linestring(&self) -> geo::LineString {
        geo::LineString::new(
            self.points
                .clone()
                .iter()
                .map(|v| coord! {x: v.x, y:v.y})
                .collect::<Vec<Coord>>(),
        )
    }

    fn to_geo_multilinestring(&self) -> geo::MultiLineString {
        geo::MultiLineString::new(vec![self.to_geo_linestring()])
    }
}

impl Upsample for Polygon {
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
        Polygon::new(points)
    }
}

impl Chaikin for Polygon {
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
            }
            points = smoothed.clone();
        });
        Polygon::new(points)
    }
}

impl ToShape for Polygon {
    fn to_shape(&self) -> Shape {
        Shape::Polygon(self.clone())
    }
}

impl Centroid for Polygon {
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

impl Sample for Polygon {
    fn sample_uniform(&self, rng: &mut StdRng, n: u64) -> Vec<Vec2> {
        let poly = geo::Polygon::new(self.to_geo_linestring(), vec![]);
        let miny = self.points.iter().map(|p| p.y as usize).min().unwrap();
        let maxy = self.points.iter().map(|p| p.y as usize).max().unwrap();
        let minx = self.points.iter().map(|p| p.x as usize).min().unwrap();
        let maxx = self.points.iter().map(|p| p.x as usize).max().unwrap();
        let width = (maxx - minx) as f64;
        let height = (maxy - miny) as f64;
        let mut samples = vec![];
        while samples.len() < n as usize {
            let x = rng.gen::<f64>() * width + minx as f64;
            let y = rng.gen::<f64>() * height + miny as f64;
            if poly.contains(&coord! { x: x, y: y}) {
                samples.push(Vec2 { x, y });
            }
        }
        samples
    }
}

impl Contains for Polygon {
    fn contains<T: Centroid>(&self, shape: &T) -> bool {
        let poly = geo::Polygon::new(self.to_geo_linestring(), vec![]);
        let other_centroid = shape.centroid();
        poly.contains(&coord! { x: other_centroid.x, y: other_centroid.y })
    }
}

impl Clip for Vec<Polygon> {
    fn clip(&self, bbox: &Polygon) -> Vec<LineString> {
        let mut segments = vec![];
        for polygon in self {
            let mut clipped = polygon.clip(bbox, false);
            segments.append(&mut clipped);
        }
        segments
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

impl ToShape for Rect {
    fn to_shape(&self) -> Shape {
        Shape::Rectangle(self.clone())
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

impl ToShape for Circle {
    fn to_shape(&self) -> Shape {
        Shape::Circle(*self)
    }
}

impl ToShape for &Circle {
    fn to_shape(&self) -> Shape {
        Shape::Circle(**self)
    }
}

impl Centroid for Vec2 {
    fn centroid(&self) -> Vec2 {
        *self
    }
}
impl Triangulate for Vec<Vec2> {
    fn triangulate(&self) -> Vec<Polygon> {
        let points: Vec<delaunator::Point> = self.iter().map(|v| {
            delaunator::Point { x: v.x, y: v.y }
        }).collect();
        let triangulation = delaunator::triangulate(&points);
        let mut triangles: Vec<Polygon> = vec![];
        for i in (0..triangulation.triangles.len()).step_by(3) {
            let p1 = &points[triangulation.triangles[i]];
            let p2 = &points[triangulation.triangles[i + 1]];
            let p3 = &points[triangulation.triangles[i + 2]];
            triangles.push(Polygon::triangle(
                Vec2::new(p1.x, p1.y),
                Vec2::new(p2.x, p2.y),
                Vec2::new(p3.x, p3.y),
            ));
        }
        triangles
    }
}

impl Rotate for Vec<LineString> {
    fn rotate(&self, angle: Angle) -> Self {
        let mut newvec = vec![];
        let mut centroid_x: f64 = 0.;
        let mut centroid_y: f64 = 0.;
        let mut points = 0.;
        self.iter().for_each(|linestring| {
            linestring.points.iter().for_each(|point| {
                centroid_x += point.x;
                centroid_y += point.y;
                points += 1.;
            });
        });
        let centroid_x = centroid_x / points;
        let centroid_y = centroid_y / points;
        let centroid = Vec2::new(centroid_x, centroid_y);
        self.iter().for_each(|linestring| {
            let newlinestring = LineString::new(
                linestring
                    .points
                    .iter()
                    .map(|point| point.rotate(centroid, angle.to_radians()))
                    .collect::<Vec<Vec2>>(),
            );
            newvec.push(newlinestring);
        });
        newvec
    }
}

impl ToShape for Arc {
    fn to_shape(&self) -> Shape {
        Shape::Arc(self.clone())
    }
}

impl ToShape for &Arc {
    fn to_shape(&self) -> Shape {
        Shape::Arc(Arc::new(self.center, self.radius, self.start, self.end))
    }
}

impl ToShape for MultiPolygon {
    fn to_shape(&self) -> Shape {
        Shape::MultiPolygon(self.clone())
    }
}

impl ToShape for &MultiPolygon {
    fn to_shape(&self) -> Shape {
        Shape::MultiPolygon(MultiPolygon::new(self.polygons.clone()))
    }
}
