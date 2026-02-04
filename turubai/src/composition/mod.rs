mod hstack;
mod vstack;

pub use hstack::*;
pub use vstack::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum HorizontalAlignment {
    #[default]
    Leading,
    Center,
    Trailing,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum VerticalAlignment {
    #[default]
    Top,
    Center,
    Bottom,
}
