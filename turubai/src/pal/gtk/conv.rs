use crate::color::Color;

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

pub fn conv_optional_color(color: &Color) -> Option<(f32, f32, f32, f32)> {
    match color {
        Color::Custom { r, g, b, a } => Some((*r, *g, *b, *a)),
        Color::SystemRed => Some((1.0, 0.0, 0.0, 1.0)),
        Color::SystemGreen => Some((0.0, 1.0, 0.0, 1.0)),
        Color::SystemBlue => Some((0.0, 0.0, 1.0, 1.0)),
        Color::SystemYellow => Some((1.0, 1.0, 0.0, 1.0)),
        Color::SystemOrange => Some((1.0, 0.5, 0.0, 1.0)),
        Color::SystemPurple => Some((0.5, 0.0, 0.5, 1.0)),
        Color::SystemPink => Some((0.921568627, 0.262745098, 0.337254902, 1.0)),
        Color::SystemIndigo => Some((0.337254902, 0.019607843, 0.568627451, 1.0)),
        Color::Text => None,
    }
}

pub fn create_pango_attr_list(
    font: &crate::font::Font,
    color: &Color,
    decoration: &crate::elements::TextDecoration,
) -> (gtk4::pango::AttrList, gtk4::pango::FontDescription) {
    // Font
    let mut font_desc = gtk4::pango::FontDescription::new();
    font_desc.set_family(&font.name());
    let size = font.size() * 1024.0; // Pango units are 1/1024
    font_desc.set_size(size as i32);

    font_desc.set_weight(conv_weight(font.weight()));

    if font.is_italic() {
        font_desc.set_style(gtk4::pango::Style::Italic);
    }

    // Attributes
    let attrs = gtk4::pango::AttrList::new();

    // Apply Font Description as Attribute
    let mut attr_font = gtk4::pango::AttrFontDesc::new(&font_desc);
    attr_font.set_start_index(0);
    attr_font.set_end_index(gtk4::pango::ATTR_INDEX_TO_TEXT_END);
    attrs.insert(attr_font);

    // Color
    if let Some((r, g, b, a)) = conv_optional_color(color) {
        let red = (r * 65535.0) as u16;
        let green = (g * 65535.0) as u16;
        let blue = (b * 65535.0) as u16;
        let alpha = (a * 65535.0) as u16;

        let mut attr_fg = gtk4::pango::AttrColor::new_foreground(red, green, blue);
        attr_fg.set_start_index(0);
        attr_fg.set_end_index(gtk4::pango::ATTR_INDEX_TO_TEXT_END);
        attrs.insert(attr_fg);

        let mut attr_alpha = gtk4::pango::AttrInt::new_foreground_alpha(alpha);
        attr_alpha.set_start_index(0);
        attr_alpha.set_end_index(gtk4::pango::ATTR_INDEX_TO_TEXT_END);
        attrs.insert(attr_alpha);
    }

    // Decoration - Underline
    let underline_style = conv_underline_style(&decoration.underline.style);

    if underline_style != gtk4::pango::Underline::None {
        let mut attr_u = gtk4::pango::AttrInt::new_underline(underline_style);
        attr_u.set_start_index(0);
        attr_u.set_end_index(gtk4::pango::ATTR_INDEX_TO_TEXT_END);
        attrs.insert(attr_u);

        if let Some((r, g, b, _)) = conv_optional_color(&decoration.underline.color) {
            let red = (r * 65535.0) as u16;
            let green = (g * 65535.0) as u16;
            let blue = (b * 65535.0) as u16;
            let mut attr_uc = gtk4::pango::AttrColor::new_underline_color(red, green, blue);
            attr_uc.set_start_index(0);
            attr_uc.set_end_index(gtk4::pango::ATTR_INDEX_TO_TEXT_END);
            attrs.insert(attr_uc);
        }
    }

    // Decoration - Strikethrough
    if decoration.strike_through.style.clone() != crate::elements::TextLineStyle::None {
        let mut attr_s = gtk4::pango::AttrInt::new_strikethrough(true);
        attr_s.set_start_index(0);
        attr_s.set_end_index(gtk4::pango::ATTR_INDEX_TO_TEXT_END);
        attrs.insert(attr_s);

        if let Color::Custom { r, g, b, a: _ } = decoration.strike_through.color {
            let red = (r * 65535.0) as u16;
            let green = (g * 65535.0) as u16;
            let blue = (b * 65535.0) as u16;
            let mut attr_sc = gtk4::pango::AttrColor::new_strikethrough_color(red, green, blue);
            attr_sc.set_start_index(0);
            attr_sc.set_end_index(gtk4::pango::ATTR_INDEX_TO_TEXT_END);
            attrs.insert(attr_sc);
        }
    }

    (attrs, font_desc)
}
