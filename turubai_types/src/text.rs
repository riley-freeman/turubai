use crate::font::FontWeight;


#[derive(Default, PartialEq, Eq, Clone, Copy)]
pub enum TextAlign {
    #[default]
    Leading,
    Center,
    Ending,
}

#[derive(Default, Clone, Copy, PartialEq)]
pub struct TextModifiers {
    pub size: f32,

    pub align: TextAlign,
    pub weight: FontWeight,
}

