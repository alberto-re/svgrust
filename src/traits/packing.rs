use crate::shapes::{Circle, LineString};

pub trait CirclePacking {
    fn pack_with_circles(&self, r: f64, circles: &mut Vec<Circle>, min_dist: f64) -> Vec<Circle>;
}

impl CirclePacking for LineString {
    fn pack_with_circles(&self, r: f64, circles: &mut Vec<Circle>, min_dist: f64) -> Vec<Circle> {
        let mut candidates: Vec<Circle> = vec![];
        if self.points.len() < 3 {
            return candidates;
        }
        if self.points[0].distance(*self.points.last().unwrap()) < r + min_dist {
            return candidates;
        }
        for point in self.points.clone() {
            let candidate = Circle::new(point, r);
            if circles
                .iter()
                .any(|circle| circle.dist(&candidate) < min_dist)
                || candidates
                    .iter()
                    .any(|circle| circle.dist(&candidate) < min_dist)
            {
                continue;
            }
            candidates.push(candidate);
        }
        candidates
    }
}
