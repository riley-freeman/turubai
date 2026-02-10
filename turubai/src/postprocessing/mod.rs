use crate::elements::Element;

mod background_color;
mod padding;

pub use background_color::*;
pub use padding::*;

pub trait PostProcess: Element {}
