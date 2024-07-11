use std::f64::consts::TAU;

use crate::angle::Angle;
use crate::shapes::polygon::Polygon;
use crate::vec2::Vec2;

/// A regular hexagon represented by a center and a side length
#[derive(Clone, PartialEq, Copy)]
pub struct Hexagon {
    /// The center point
    pub center: Vec2,
    /// The side length
    pub side: f64,
    /// The apothem i.e. the distance between the midpoint of any side and the center of the hexagon
    pub apothem: f64,
    /// The rotation angle
    pub theta: Angle,
}

impl Hexagon {
    pub fn new(center: Vec2, side: f64, theta: Angle) -> Self {
        let apothem = 0.5 * f64::sqrt(3.0) * side;
        Self {
            center,
            side,
            apothem,
            theta,
        }
    }

    pub fn spiral(center: Vec2, side: f64, theta: Angle, n: usize) -> Vec<Self> {
        let mut hexagons = vec![];
        if n == 0 {
            return hexagons;
        }
        let last = Hexagon::new(center, side, theta);
        hexagons.push(last);
        if n == 1 {
            return hexagons;
        }
        // Pick an arbitrary adjacent hexagon. TODO: parametrize
        let mut dir_angle = Angle::from_degrees(30.);
        let adj_center = last.center + Vec2::from_polar(last.theta + dir_angle, last.apothem * 2.);
        let last = Hexagon::new(adj_center, side, theta);
        hexagons.push(last);
        if n == 2 {
            return hexagons;
        }

        dir_angle += Angle::from_degrees(120.);
        let mut adj_center =
            last.center + Vec2::from_polar(last.theta + dir_angle, last.apothem * 2.);
        let mut last = Hexagon::new(adj_center, side, theta);
        hexagons.push(last);
        if n == 3 {
            return hexagons;
        }

        for _ in 3..n {
            adj_center = last.center + Vec2::from_polar(last.theta + dir_angle, last.apothem * 2.);
            let n_adj_hex: usize = hexagons
                .iter()
                .map(|x| {
                    if x.center.distance(adj_center) <= x.apothem * 2.05 {
                        1
                    } else {
                        0
                    }
                })
                .sum();
            if n_adj_hex < 2 {
                dir_angle += Angle::from_degrees(60.);
                adj_center =
                    last.center + Vec2::from_polar(last.theta + dir_angle, last.apothem * 2.);
            }
            last = Hexagon::new(adj_center, side, theta);
            hexagons.push(last);
        }
        hexagons
    }

    pub fn vertexes(&self) -> Vec<Vec2> {
        let step = TAU / 6.;
        (0..6)
            .map(|i| {
                let theta = Angle::from_radians(i as f64 * step) + self.theta;
                self.center + Vec2::from_polar(theta, self.side)
            })
            .collect::<Vec<Vec2>>()
    }

    pub fn sides(&self) -> Vec<(Vec2, Vec2)> {
        let mut sides = vec![];
        let vertexes = self.vertexes();
        for i in 0..6 {
            sides.push((vertexes[i], vertexes[(i + 1) % 6]))
        }
        sides
    }

    pub fn to_polygon(&self) -> Polygon {
        Polygon::new(self.vertexes())
    }
}
