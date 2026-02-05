pub mod color;
pub mod composition;
pub mod elements;
pub mod font;
pub mod pal;
pub mod runtime;
pub mod shadow;

mod units;
pub use units::Unit;

#[cfg(test)]
mod tests;

use elements::Element;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum Backend {
    WinUI = 0,
    Apple = 1,
    Android = 2,
    Wayland = 3,
    X11 = 4,
}

pub trait Application: Send + Sync {
    fn markup(&self) -> Box<dyn Element>;
}
