#[derive(Clone, PartialEq)]
pub struct Pen<'a> {
    pub color: &'a str,
    ///  TODO: add nominal thicknews
    pub thickness: f64,
    pub diameter: f64,
}

impl<'a> Pen<'a> {
    pub fn new(color: &'a str, thickness: f64, diameter: f64) -> Self {
        Self {
            color,
            thickness,
            diameter,
        }
    }

    /// Sakura Pigma Micron 005 - Black
    pub fn pigma_micron_005_black() -> Self {
        Self::new("black", 0.20, 8.)
    }

    /// Sakura Pigma Micron 01 - Black
    pub fn pigma_micron_01_black() -> Self {
        Self::new("black", 0.25, 8.)
    }

    /// Sakura Pigma Micron 02 - Black
    pub fn pigma_micron_02_black() -> Self {
        Self::new("black", 0.30, 8.)
    }

    /// Sakura Pigma Micron 03 - Black
    pub fn pigma_micron_03_black() -> Self {
        Self::new("black", 0.35, 8.)
    }

    /// Sakura Pigma Micron 05 - Black
    pub fn pigma_micron_05_black() -> Self {
        Self::new("black", 0.45, 8.)
    }

    /// Sakura Pigma Micron 05 - Red
    pub fn pigma_micron_05_red() -> Self {
        Self::new("rgba(218, 28, 31, 1.0)", 0.45, 8.)
    }

    /// Sakura Pigma Micron 05 - Blue
    pub fn pigma_micron_05_blue() -> Self {
        Self::new("rgba(64, 96, 171, 1.0)", 0.45, 8.)
    }

    /// Sakura Pigma Micron 05 - Green
    pub fn pigma_micron_05_green() -> Self {
        Self::new("rgba(32, 179, 151, 1.0)", 0.45, 8.)
    }

    /// Sakura Pigma Micron 05 - Purple
    pub fn pigma_micron_05_purple() -> Self {
        Self::new("rgba(121, 81, 151, 1.0)", 0.45, 8.)
    }

    /// Sakura Pigma Micron 08 - Black
    pub fn pigma_micron_08_black() -> Self {
        Self::new("black", 0.5, 8.)
    }

    /// Uni-Ball Signo Broad - Gold
    /// The nominal thickness is 0.7mm but without applying any pressure
    /// it is half if not slightly less.
    pub fn uniball_signo_broad_gold() -> Self {
        Self::new("gold", 0.5, 8.)
    }

    /// Uni-Ball Signo Broad - White
    /// The nominal thickness is 0.7mm but without applying any pressure
    /// it is half if not slightly less.
    pub fn uniball_signo_broad_white() -> Self {
        Self::new("white", 0.5, 8.)
    }

    /// Uni Posca Metallic PC-1M - Gold
    /// The nominal thickness is 0.7mm but without applying any pressure
    /// it is half if not slightly less.
    pub fn uniposca_metallic_pc1m_gold() -> Self {
        Self::new("gold", 0.5, 1.4)
    }

    /// Stabilo Pen 68/46 - Black
    /// The nominal thickness is 1mm but without applying any pressure
    /// it is half if not slightly less.
    pub fn stabilo_pen_68_46_black() -> Self {
        Self::new("rgba(0, 0, 0, 0.9)", 0.5, 7.0)
    }

    /// Stabilo Pen 68/51 - Turquoise
    /// The nominal thickness is 1mm but without applying any pressure
    /// it is half if not slightly less.
    pub fn stabilo_pen_68_51_turquoise() -> Self {
        Self::new("rgba(42, 148, 146, 0.9)", 0.5, 7.0)
    }

    /// Molotow ONE4ALL 161 - Shock blue middle
    /// The nominal thickness is 2mm but without applying any pressure
    /// it is just about half.
    pub fn molotow_one4all_161_shock_blue_middle() -> Self {
        Self::new("rgba(0, 131, 187, 1.0)", 1.2, 8.0)
    }
}
