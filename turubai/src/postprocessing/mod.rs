use crate::elements::Element;

mod background_color;
mod padding;
mod frame;

pub use background_color::*;
pub use padding::*;
pub use frame::*;

pub trait PostProcess: Element {}
