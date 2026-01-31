mod vstack;
mod hstack;

pub use vstack::*;
pub use hstack::*;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct VStackModifiers {
    pub spacing: f32,
    pub alignment: HorizontalAlignment,
}


#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct HStackModifiers {
    pub spacing: f32,
    pub alignment: VerticalAlignment,
}

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

