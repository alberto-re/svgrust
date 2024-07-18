use crate::prelude::Vec2;

#[derive(Copy, Clone, Debug)]
pub struct Edge {
    pub v1: Vec2,
    pub v2: Vec2,
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.v1 == other.v1 && self.v2 == other.v2 || self.v1 == other.v2 && self.v2 == other.v1
    }
}

impl Eq for Edge {}

#[cfg(test)]
mod tests {
    use crate::prelude::Vec2;
    use crate::shapes::edge::Edge;
    use rstest::rstest;

    #[rstest]
    #[case(20., 20., 30., 30., 20., 20., 30., 30., true)]
    #[case(30., 30., 20., 20., 20., 20., 30., 30., true)]
    #[case(10., 10., 30., 30., 20., 20., 30., 30., false)]
    fn eq(
        #[case] p1x: f64,
        #[case] p1y: f64,
        #[case] p2x: f64,
        #[case] p2y: f64,
        #[case] p3x: f64,
        #[case] p3y: f64,
        #[case] p4x: f64,
        #[case] p4y: f64,
        #[case] expected: bool,
    ) {
        let e1 = Edge {
            v1: Vec2::new(p1x, p1y),
            v2: Vec2::new(p2x, p2y),
        };
        let e2 = Edge {
            v1: Vec2::new(p3x, p3y),
            v2: Vec2::new(p4x, p4y),
        };
        assert_eq!(e1 == e2, expected);
    }
}
