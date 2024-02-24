use crate::vec2::Vec2;

pub struct SquareGrid {
    vertexes: Vec<Vec2>,
    centers: Vec<Vec2>,
}

impl SquareGrid {
    pub fn new(x: f64, y: f64, width: f64, height: f64, square_side: f64) -> Self {
        let mut vertexes = vec![];
        let mut centers = vec![];
        let rows: usize = height as usize / square_side as usize;
        let cols: usize = width as usize / square_side as usize;
        let padding_width: f64 = (width - cols as f64 * square_side) / 2.;
        let padding_height: f64 = (height - rows as f64 * square_side) / 2.;
        for row_index in 0..rows {
            for col_index in 0..cols {
                let vx = x + padding_width + col_index as f64 * square_side;
                let vy = y + padding_height + row_index as f64 * square_side;
                let cx = vx + square_side / 2.;
                let cy = vy + square_side / 2.;
                vertexes.push(Vec2 { x: vx, y: vy });
                centers.push(Vec2 { x: cx, y: cy });
            }
        }
        Self { vertexes, centers }
    }

    pub fn iter_vertexes(&self) -> std::slice::Iter<'_, Vec2> {
        self.vertexes.iter()
    }

    pub fn iter_centers(&self) -> std::slice::Iter<'_, Vec2> {
        self.centers.iter()
    }
}
