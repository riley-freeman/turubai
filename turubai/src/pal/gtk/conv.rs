pub fn conv_weight(weight: crate::font::FontWeight) -> gtk4::pango::Weight {
    match weight {
        crate::font::FontWeight::Thin => gtk4::pango::Weight::Thin,
        crate::font::FontWeight::ExtraLight => gtk4::pango::Weight::Ultralight,
        crate::font::FontWeight::Light => gtk4::pango::Weight::Light,
        crate::font::FontWeight::SemiLight => gtk4::pango::Weight::Semilight,
        crate::font::FontWeight::Regular => gtk4::pango::Weight::Normal,
        crate::font::FontWeight::Medium => gtk4::pango::Weight::Medium,
        crate::font::FontWeight::SemiBold => gtk4::pango::Weight::Semibold,
        crate::font::FontWeight::Bold => gtk4::pango::Weight::Bold,
        crate::font::FontWeight::ExtraBold => gtk4::pango::Weight::Ultrabold,
        crate::font::FontWeight::Black => gtk4::pango::Weight::Heavy,
        crate::font::FontWeight::ExtraBlack => gtk4::pango::Weight::Ultraheavy,
    }
}

pub fn conv_underline_style(style: &crate::elements::TextLineStyle) -> gtk4::pango::Underline {
    match style {
        crate::elements::TextLineStyle::None => gtk4::pango::Underline::None,
        crate::elements::TextLineStyle::Single => gtk4::pango::Underline::Single,
        crate::elements::TextLineStyle::Double => gtk4::pango::Underline::Double,
        // Pango doesn't support Thick/Dotted/Dashed underlines via standard attributes easily
        // Mapping them to single for now as per plan
        crate::elements::TextLineStyle::Thick => gtk4::pango::Underline::Single,
        crate::elements::TextLineStyle::Dotted => gtk4::pango::Underline::Single,
        crate::elements::TextLineStyle::Dashed => gtk4::pango::Underline::Single,
    }
}
