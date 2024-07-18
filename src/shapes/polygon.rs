use crate::shapes::edge::Edge;
use crate::shapes::LineString;
use crate::traits::ToGeoLineString;
use crate::vec2::Vec2;
use geo::algorithm::bool_ops::BooleanOps;
use geo::Coord;
use geo::MultiPolygon as GeoMultiPolygon;

/// A bounded area represented by a LineString exterior ring
#[derive(Clone, PartialEq, Debug)]
pub struct Polygon {
    pub points: Vec<Vec2>,
}

impl Polygon {
    pub fn new(points: Vec<Vec2>) -> Self {
        Self { points }
    }

    pub fn triangle(p1: Vec2, p2: Vec2, p3: Vec2) -> Self {
        Self {
            points: vec![p1, p2, p3],
        }
    }

    pub fn add_vec(&mut self, vec: Vec2) -> Self {
        self.points.iter_mut().for_each(|p| {
            p.x += vec.x;
            p.y += vec.y;
        });
        self.clone()
    }

    pub fn to_linestring(&self) -> LineString {
        let mut points = self.points.clone();
        if points[0] != points[points.len() - 1] {
            points.push(points[0]);
        }
        LineString::new(points)
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
        let mut retval = vec![self.to_linestring()];
        others.iter().for_each(|other| {
            retval = retval
                .iter()
                .flat_map(|l| l.clip(other, invert))
                .collect::<Vec<LineString>>();
        });
        retval
    }

    pub fn edges(&self) -> Vec<Edge> {
        let mut edges = vec![];
        for i in 0..self.points.len() {
            let curpoint = self.points[i];
            let nextpoint = self.points[(i + 1) % self.points.len()];
            edges.push(Edge {
                v1: curpoint,
                v2: nextpoint,
            });
        }
        edges
    }

    pub fn edges_in_common(&self, rhs: &Polygon) -> Vec<Edge> {
        let left = self.edges();
        let right = rhs.edges();
        let mut common = vec![];
        for l in &left {
            for r in &right {
                if l == r {
                    common.push(*l);
                }
            }
        }
        common
    }

    // Merge two adjacent (they shares at least one edge),
    // non-overlapping polygons.
    // The method doesn't check the preconditions: is up
    // to the caller to ensure preconditions are met.
    pub fn merge_adjacent(&self, adj: &Self) -> Self {
        let mut union_points = vec![];
        union_points.append(&mut self.points.clone());
        union_points.append(&mut adj.points.clone());

        let psum: Vec2 = union_points.iter().copied().sum();
        let center = psum / union_points.len() as f64;

        union_points.sort_by_key(|p| (*p - center).to_polar().0);
        union_points.dedup_by_key(|p| (*p - center).to_polar().0);

        Polygon::new(union_points)
    }
}

#[cfg(test)]
mod tests {
    use crate::shapes::edge::Edge;
    use crate::shapes::polygon::Polygon;
    use crate::vec2::Vec2;

    #[test]
    fn edges_in_common() {
        let p1 = Polygon::new(vec![
            Vec2::new(10., 10.),
            Vec2::new(20., 10.),
            Vec2::new(20., 20.),
            Vec2::new(10., 20.),
        ]);
        let p2 = Polygon::new(vec![
            Vec2::new(20., 10.),
            Vec2::new(30., 10.),
            Vec2::new(30., 20.),
            Vec2::new(20., 20.),
        ]);
        let edges = p1.edges_in_common(&p2);
        assert_eq!(edges.len(), 1);
        assert_eq!(
            edges[0],
            Edge {
                v1: Vec2::new(20., 10.),
                v2: Vec2::new(20., 20.)
            }
        );
    }
}
