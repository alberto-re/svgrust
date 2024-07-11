use crate::grid::SquareGrid;
use crate::shapes::linestring::LineString;
use crate::shapes::polygon::Polygon;
use crate::vec2::Vec2;

/// A rectangle represented by an upper-left corner plus width and height dimesions
#[derive(Clone, PartialEq, Debug)]
pub struct Rect {
    pub xy: Vec2,
    pub width: f64,
    pub height: f64,
}

impl Rect {
    pub fn new(xy: Vec2, width: f64, height: f64) -> Self {
        Self { xy, width, height }
    }

    pub fn with_center(xy: Vec2, h: f64, w: f64) -> Rect {
        let x = xy.x - w / 2.;
        let y = xy.y - h / 2.;
        Self::new(Vec2 { x, y }, h, w)
    }

    pub fn square_with_center(xy: Vec2, l: f64) -> Rect {
        Self::with_center(xy, l, l)
    }

    pub fn min_len(&self) -> f64 {
        f64::min(self.width, self.height)
    }

    pub fn grid(&self, rows: u64, cols: u64) -> Vec<Rect> {
        let w = self.width / cols as f64;
        let h = self.height / rows as f64;
        let mut cells = vec![];
        (0..rows).for_each(|r| {
            (0..cols).for_each(|c| {
                cells.push(Rect::new(
                    Vec2 {
                        x: c as f64 * w + self.xy.x,
                        y: r as f64 * h + self.xy.y,
                    },
                    w,
                    h,
                ));
            });
        });
        cells
    }

    pub fn to_linestr(&self, close: bool) -> LineString {
        let mut points = vec![
            self.xy,
            Vec2 {
                x: self.xy.x + self.width,
                y: self.xy.y,
            },
            Vec2 {
                x: self.xy.x + self.width,
                y: self.xy.y + self.height,
            },
            Vec2 {
                x: self.xy.x,
                y: self.xy.y + self.height,
            },
        ];
        if close {
            points.push(self.xy);
        }
        LineString { points }
    }

    pub fn to_polygon(&self) -> Polygon {
        let points = vec![
            self.xy,
            Vec2 {
                x: self.xy.x + self.width,
                y: self.xy.y,
            },
            Vec2 {
                x: self.xy.x + self.width,
                y: self.xy.y + self.height,
            },
            Vec2 {
                x: self.xy.x,
                y: self.xy.y + self.height,
            },
        ];
        Polygon { points }
    }

    pub fn into_square_grid(&self, square_side: f64) -> SquareGrid {
        SquareGrid::new(self.xy.x, self.xy.y, self.width, self.height, square_side)
    }

    pub fn sample_poisson2d(&self, radius: f64, seed: u64) -> Vec<Vec2> {
        let samples = fast_poisson::Poisson2D::new()
            .with_seed(seed)
            .with_dimensions([self.width, self.height], radius);

        samples.iter().map(|s| Vec2::from_slice(&s)).collect()
    }
}
