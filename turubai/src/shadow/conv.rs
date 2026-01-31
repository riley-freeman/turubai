use taffy::AlignItems;

use crate::composition::VerticalAlignment;
use crate::composition::HorizontalAlignment;

pub fn conv_v_alignment(a: VerticalAlignment) -> AlignItems {
    match a {
        VerticalAlignment::Top => AlignItems::Start,
        VerticalAlignment::Center => AlignItems::Center,
        VerticalAlignment::Bottom => AlignItems::End,
    }
}

pub fn conv_h_alignment(a: HorizontalAlignment) -> AlignItems {
    match a {
        HorizontalAlignment::Leading => AlignItems::Start,
        HorizontalAlignment::Center => AlignItems::Center,
        HorizontalAlignment::Trailing => AlignItems::End,
    }
}
