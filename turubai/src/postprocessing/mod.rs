use crate::elements::Element;

mod background_color;
pub use background_color::*;

pub trait PostProcess: Element {}
