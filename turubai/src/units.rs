use core::f64;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum Unit {
    #[default]
    Auto,
    Pixels(f64),
    Percent(f64),
    Em(f64),
}

impl Eq for Unit {}

impl Unit {
    pub fn to_pixels(&self, available: Option<f64>) -> f64 {
        match self {
            Unit::Auto => f64::NAN,
            Unit::Pixels(v) => *v,
            Unit::Percent(v) => available
                .map(|available| available * v)
                .unwrap_or(f64::INFINITY),
            Unit::Em(v) => v * 16.0,
        }
    }

    pub fn is_percent(&self) -> bool {
        matches!(self, Unit::Percent(_))
    }
}

impl Into<taffy::Dimension> for Unit {
    fn into(self) -> taffy::Dimension {
        match self {
            Unit::Pixels(v) => taffy::Dimension::length(v as f32),
            Unit::Percent(v) => taffy::Dimension::percent(v as f32),
            Unit::Em(v) => taffy::Dimension::length(v as f32 * 16.0),
            Unit::Auto => taffy::Dimension::auto(),
        }
    }
}

impl Into<taffy::LengthPercentage> for Unit {
    fn into(self) -> taffy::LengthPercentage {
        match self {
            Unit::Pixels(v) => taffy::LengthPercentage::length(v as f32),
            Unit::Percent(v) => taffy::LengthPercentage::percent(v as f32),
            Unit::Em(v) => taffy::LengthPercentage::length(v as f32 * 16.0),
            Unit::Auto => taffy::LengthPercentage::length(0.0),
        }
    }
}
