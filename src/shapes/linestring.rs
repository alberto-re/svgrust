use crate::traits::ToGeoLineString;
use crate::vec2::Vec2;
use geo::algorithm::bool_ops::BooleanOps;
use geo::Coord;
use geo::MultiPolygon as GeoMultiPolygon;

/// A series of contiguous line segments represented by two or more points
#[derive(Clone, PartialEq)]
pub struct LineString {
    pub points: Vec<Vec2>,
}

impl LineString {
    pub fn new(points: Vec<Vec2>) -> Self {
        Self { points }
    }

    pub fn line(start: Vec2, end: Vec2) -> Self {
        Self {
            points: vec![start, end],
        }
    }

    pub fn add_vec(&mut self, vec: Vec2) -> Self {
        self.points.iter_mut().for_each(|p| {
            p.x += vec.x;
            p.y += vec.y;
        });
        self.clone()
    }

    pub fn clip<T: ToGeoLineString>(&self, other: &T, invert: bool) -> Vec<LineString> {
        let poly = geo::Polygon::new(other.to_geo_linestring(), vec![]);
        let mpoly = GeoMultiPolygon::new(vec![poly]);
        let clipped = mpoly.clip(&self.to_geo_multilinestring(), invert);
        let mut res = vec![];
        clipped.0.iter().for_each(|l| {
            let mut points: Vec<Coord> = vec![];
            l.clone().into_points().iter().for_each(|p| {
                points.push(p.0);
            });
            res.push(LineString::new(
                points
                    .iter()
                    .map(|c| Vec2 { x: c.x, y: c.y })
                    .collect::<Vec<Vec2>>(),
            ));
        });
        res
    }

    pub fn clip_many<T: ToGeoLineString>(&self, others: &[T], invert: bool) -> Vec<LineString> {
        let mut retval = vec![self.clone()];
        others.iter().for_each(|other| {
            retval = retval
                .iter()
                .flat_map(|l| l.clip(other, invert))
                .collect::<Vec<_>>();
        });
        retval
    }
}
