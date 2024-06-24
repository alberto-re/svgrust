pub struct Pen<'a> {
    pub color: &'a str,
    pub thickness: f64,
}

impl<'a> Pen<'a> {
    pub fn new(color: &'a str, thickness: f64) -> Self {
        Self { color, thickness }
    }

    /// Sakura Pigma Micron 005 - Black
    pub fn pigma_micron_005_black() -> Self {
        Self::new("black", 0.20)
    }

    /// Sakura Pigma Micron 01 - Black
    pub fn pigma_micron_01_black() -> Self {
        Self::new("black", 0.25)
    }

    /// Sakura Pigma Micron 02 - Black
    pub fn pigma_micron_02_black() -> Self {
        Self::new("black", 0.30)
    }

    /// Sakura Pigma Micron 03 - Black
    pub fn pigma_micron_03_black() -> Self {
        Self::new("black", 0.35)
    }

    /// Sakura Pigma Micron 05 - Black
    pub fn pigma_micron_05_black() -> Self {
        Self::new("black", 0.45)
    }

    /// Sakura Pigma Micron 08 - Black
    pub fn pigma_micron_08_black() -> Self {
        Self::new("black", 0.5)
    }
}
