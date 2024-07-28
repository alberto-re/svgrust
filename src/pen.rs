/// A pen, marker, pencil.. or just anything that a pen-plotter can use to draw.
#[derive(Clone, PartialEq)]
pub struct Pen<'a> {
    /// The color of the stroke. Can be anything CSS understands.
    pub color: &'a str,
    /// The thickness of the stroke expressed in the `Uom` of the `Sketch`.
    pub thickness: f64,
    /// The diameter of the physical pen.
    pub diameter: f64,
}

impl<'a> Pen<'a> {
    /// Create a new `Pen` with given `color, `thickness` and `diameter`.
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
    pub fn uniball_signo_broad_gold() -> Self {
        Self::new("gold", 0.5, 8.)
    }

    /// Uni-Ball Signo Broad - White
    pub fn uniball_signo_broad_white() -> Self {
        Self::new("white", 0.5, 8.)
    }

    /// Uni Posca Metallic PC-1M - Gold
    pub fn uniposca_metallic_pc1m_gold() -> Self {
        Self::new("gold", 0.5, 1.4)
    }

    /// https://www.stabilo.com/it/prodotti/per-colorare/pennarelli/stabilo-pen-68/
    ///
    /// Stabilo Pen 68/46 - Black
    pub fn stabilo_pen_68_46_black() -> Self {
        Self::new("rgba(0, 0, 0, 0.9)", 0.9, 7.0)
    }

    /// Stabilo Pen 68/51 - Turquoise
    pub fn stabilo_pen_68_51_turquoise() -> Self {
        Self::new("rgba(42, 148, 146, 0.9)", 0.9, 7.0)
    }

    /// Stabilo Pen 68/86 - Beige
    pub fn stabilo_pen_68_86_beige() -> Self {
        Self::new("rgba(195, 189, 182, 0.9)", 0.9, 7.0)
    }

    /// Stabilo Pen 68/32 - Ultramarine
    pub fn stabilo_pen_68_32_ultramarine() -> Self {
        Self::new("rgba(19, 66, 149, 0.9)", 0.9, 7.0)
    }

    /// Stabilo Pen 68/12 - Eucalyptus
    pub fn stabilo_pen_68_12_eucalyptus() -> Self {
        Self::new("rgba(131, 192, 181, 0.9)", 0.9, 7.0)
    }

    /// Molotow ONE4ALL 161 - Shock blue middle
    pub fn molotow_one4all_161_shock_blue_middle() -> Self {
        Self::new("rgba(0, 131, 187, 1.0)", 1.2, 8.0)
    }
}
