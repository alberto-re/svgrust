pub mod packing;

use std::f64::consts::TAU;

use crate::angle::Angle;
use crate::prelude::Pen;
use crate::shapes::{Arc, Circle, Hexagon, LineString, MultiPolygon, Polygon, Rect, Text};
use crate::vec2::Vec2;
use crate::Shape;
use geo::algorithm::Rotate as GeoRotate;
use geo::coord;
use geo::Contains as GeoContains;
use geo::Coord;
use geo::CoordsIter;
use rand::rngs::StdRng;
use rand::Rng;

pub trait ScalePerc {
    fn scale_perc(&self, percentage: f64) -> Self;
}

pub trait ScaleDist {
    fn scale_dist(&self, distance: f64) -> Self;
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

pub trait BoundingBox {
    fn bbox(&self) -> Rect;
    fn bbox_margin(&self, margin: f64) -> Rect;
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

pub trait Lerp {
    fn lerp(&self, other: Self, t: f64) -> Self;
}

#[derive(Clone)]
pub enum HatchFillStrategy {
    HorizontalLines,
}

pub trait HatchFill {
    fn hatch_fill(&self, pen: &Pen, strategy: HatchFillStrategy) -> Vec<LineString>;
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
            points.clone_from(&upsampled);
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
            points.clone_from(&smoothed);
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

impl BoundingBox for LineString {
    fn bbox(&self) -> Rect {
        let mut xmin = f64::MAX;
        let mut xmax = 0.;
        let mut ymin = f64::MAX;
        let mut ymax = 0.;
        self.points.iter().for_each(|p| {
            if p.x < xmin {
                xmin = p.x
            }
            if p.x > xmax {
                xmax = p.x
            }
            if p.y < ymin {
                ymin = p.y
            }
            if p.y > ymax {
                ymax = p.y
            }
        });
        Rect::new(Vec2::new(xmin, ymin), xmax - xmin, ymax - ymin)
    }

    fn bbox_margin(&self, margin: f64) -> Rect {
        let bbox = &self.bbox();
        let half_margin = margin / 2.0;
        Rect::new(
            bbox.xy + Vec2::new(-half_margin, -half_margin),
            bbox.width + half_margin,
            bbox.height + half_margin,
        )
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
            points.clone_from(&upsampled);
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
            points.clone_from(&smoothed);
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

impl ScaleDist for Polygon {
    fn scale_dist(&self, distance: f64) -> Polygon {
        // This is a very raw and fragile adaptation of this code:
        // https://codepen.io/HansMuller/pen/AgLWaz
        let edges: Vec<(Vec2, Vec2)> = self.edges();

        fn inward_normal(p1: Vec2, p2: Vec2) -> Vec2 {
            let dx = p2.x - p1.x;
            let dy = p2.y - p1.y;
            let edge_length = f64::sqrt(dx * dx + dy * dy);
            Vec2::new(-dy / edge_length, dx / edge_length)
        }

        let mut offset_edges: Vec<(Vec2, Vec2)> = vec![];
        for edge in &edges {
            let normal = inward_normal(edge.0, edge.1);
            let dx = normal.x * distance;
            let dy = normal.y * distance;
            let offset_edge = (
                Vec2::new(edge.0.x + dx, edge.0.y + dy),
                Vec2::new(edge.1.x + dx, edge.1.y + dy),
            );
            offset_edges.push(offset_edge)
        }

        fn edges_intersection(edge1: (Vec2, Vec2), edge2: (Vec2, Vec2)) -> Vec2 {
            let den = (edge2.1.y - edge2.0.y) * (edge1.1.x - edge1.0.x)
                - (edge2.1.x - edge2.0.x) * (edge1.1.y - edge1.0.y);
            if den == 0. {
                panic!(); // lines are parallel or conincident
            }

            let ua = ((edge2.1.x - edge2.0.x) * (edge1.0.y - edge2.0.y)
                - (edge2.1.y - edge2.0.y) * (edge1.0.x - edge2.0.x))
                / den;

            Vec2::new(
                edge1.0.x + ua * (edge1.1.x - edge1.0.x),
                edge1.0.y + ua * (edge1.1.y - edge1.0.y),
            )
        }

        let mut vertices: Vec<Vec2> = vec![];
        for i in 0..offset_edges.len() {
            let this_edge = offset_edges[i];
            let prev_edge = offset_edges[(i + offset_edges.len() - 1) % offset_edges.len()];
            let vertex = edges_intersection(prev_edge, this_edge);
            vertices.push(vertex);
        }
        Polygon::new(vertices)
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

impl ScalePerc for Rect {
    fn scale_perc(&self, percentage: f64) -> Rect {
        Rect::new(
            Vec2 {
                x: self.xy.x + self.width * ((1. - percentage) / 2.),
                y: self.xy.y + self.height * ((1. - percentage) / 2.),
            },
            self.width * percentage,
            self.height * percentage,
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
        self.center.distance(shape.centroid()) < self.radius
    }
}

impl ScalePerc for Circle {
    fn scale_perc(&self, percentage: f64) -> Circle {
        Circle::new(self.center, self.radius * percentage)
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
        let points: Vec<delaunator::Point> = self
            .iter()
            .map(|v| delaunator::Point { x: v.x, y: v.y })
            .collect();
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
                    .map(|point| point.rotate(centroid, angle))
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

impl ToShape for Text {
    fn to_shape(&self) -> Shape {
        Shape::Text(self.clone())
    }
}

impl ToShape for Hexagon {
    fn to_shape(&self) -> Shape {
        Shape::Hexagon(*self)
    }
}

impl ScalePerc for Hexagon {
    fn scale_perc(&self, percentage: f64) -> Hexagon {
        Hexagon::new(self.center, self.side * percentage, self.theta)
    }
}

impl ScaleDist for Hexagon {
    fn scale_dist(&self, distance: f64) -> Self {
        Hexagon::new(self.center, self.side + distance, self.theta)
    }
}

impl HatchFill for Hexagon {
    fn hatch_fill(&self, pen: &Pen, strategy: HatchFillStrategy) -> Vec<LineString> {
        let mut lines = vec![];
        match strategy {
            HatchFillStrategy::HorizontalLines => {
                let square = Rect::square_with_center(self.center, self.side * 2.);
                let mut y = self.center.y - self.side;
                let x = square.centroid().x - self.side + pen.thickness / 2.;
                while y < self.center.y + self.side - pen.thickness / 2. {
                    lines.push(LineString::line(
                        Vec2::new(x, y),
                        Vec2::new(x + self.side * 2. + pen.thickness / 2., y),
                    ));
                    y += pen.thickness;
                }
                lines = lines
                    .iter()
                    .flat_map(|l| l.clip(&self.to_polygon(), false))
                    .collect::<Vec<LineString>>();
            }
        }
        lines
    }
}
