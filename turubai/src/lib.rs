pub mod elements;
pub mod composition;
pub mod runtime;
pub mod font;
pub mod shadow;
pub mod pal;

#[cfg(test)]
mod tests;

use elements::Element;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum Backend {
    WinUI   = 0,
    Apple   = 1,
    Android = 2,
    Wayland = 3,
    X11     = 4,
}

pub trait Application: Send + Sync {
    fn markup(&self) -> Box<dyn Element>;
}

pub enum Unit {
    Pixels(f64),
    Em(f64),
    Percent(f64)
}

impl Unit {
    pub fn px(x: f64) -> Self {
        Self::Pixels(x)
    }

    pub fn em(x: f64) -> Self {
        Self::Em(x)
    }

    pub fn percent(x: f64) -> Self {
        Self::Percent(x)
    }
}





