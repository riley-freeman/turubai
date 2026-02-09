use gtk4::{
    gdk::Display,
    prelude::{StyleContextExt, WidgetExt},
    CssProvider,
};

use crate::{color::Color, elements::TextLineStyle};

pub fn conv_underline_style(style: &crate::elements::TextLineStyle) -> &'static str {
    match style {
        TextLineStyle::None => "none",
        TextLineStyle::Single => "underline",
        TextLineStyle::Double => "underline double",
        TextLineStyle::Thick => "underline",
        TextLineStyle::Dotted => "underline dotted",
        TextLineStyle::Dashed => "underline dashed",
    }
}

pub fn conv_strike_through_style(style: &crate::elements::TextLineStyle) -> &'static str {
    match style {
        TextLineStyle::None => "none",
        TextLineStyle::Single => "line-through",
        TextLineStyle::Double => "line-through double",
        TextLineStyle::Thick => "line-through",
        TextLineStyle::Dotted => "line-through dotted",
        TextLineStyle::Dashed => "line-through dashed",
    }
}

pub fn conv_color_to_css(color: &Color) -> String {
    match color {
        Color::Custom { r, g, b, a } => {
            format! {"rgba({},{},{},{})", r * 255.0, g * 255.0, b * 255.0, a}
        }
        Color::SystemRed => "red".to_string(),
        Color::SystemGreen => "green".to_string(),
        Color::SystemBlue => "blue".to_string(),
        Color::SystemYellow => "yellow".to_string(),
        Color::SystemOrange => "orange".to_string(),
        Color::SystemPurple => "purple".to_string(),
        Color::SystemPink => "pink".to_string(),
        Color::SystemIndigo => "indigo".to_string(),
        Color::Text => "inherit".to_string(),
    }
}

pub fn conv_create_text_class(
    font: &crate::font::Font,
    color: &Color,
    decoration: &crate::elements::TextDecoration,
) -> String {
    let mut properties = String::new();
    let family = format! {"font-family: \"{}\";", font.name()};
    let size = format! {"font-size: {}pt;", font.size()};
    let weight = format! {"font-weight: {};", font.weight() as u32};
    let style = format! {"font-style: {};", if font.is_italic() { "italic" } else { "normal" }};
    let color = format!("color: {};", conv_color_to_css(color));

    let mut decorations = String::new();
    let underline = conv_underline_style(&decoration.underline.style);
    let underline_color = conv_color_to_css(&decoration.underline.color);

    let strikethrough = conv_strike_through_style(&decoration.strike_through.style);
    let strikethrough_color = conv_color_to_css(&decoration.strike_through.color);

    let underline = format!("{}", underline);
    decorations.push_str(&underline);
    if decoration.strike_through.style != crate::elements::TextLineStyle::None {
        let strikethrough = format!(" {} {}", strikethrough, strikethrough_color);
        decorations.push_str(&strikethrough);
    } else {
        decorations.push(' ');
        decorations.push_str(&underline_color);
    }

    properties.push_str(&format!("text-decoration: {};", decorations));
    properties.push_str(&color);
    properties.push_str(&family);
    properties.push_str(&size);
    properties.push_str(&weight);
    properties.push_str(&style);

    let class = randomizer::Randomizer::ALPHABETICAL(8).string().unwrap();
    let css = format!(".{} {{ {} }}", class, properties);

    let css_provider = CssProvider::new();
    css_provider.load_from_data(css.as_str());
    gtk4::style_context_add_provider_for_display(
        &Display::default().unwrap(),
        &css_provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
    class
}

// pub fn create_pango_attr_list(
//     font: &crate::font::Font,
//     color: &Color,
//     decoration: &crate::elements::TextDecoration,
// ) -> (gtk4::pango::AttrList, gtk4::pango::FontDescription) {
//     // Font
//     let mut font_desc = gtk4::pango::FontDescription::new();
//     font_desc.set_family(&font.name());
//
//     let size = font.size() * 1024.0; // Pango units are 1/1024
//     font_desc.set_size(size as i32);
//     font_desc.set_weight(conv_weight(font.weight()));
//     if font.is_italic() {
//         font_desc.set_style(gtk4::pango::Style::Italic);
//     }
//
//     // Attributes
//     let attrs = gtk4::pango::AttrList::new();
//
//     // Apply Font Description as Attribute
//     let mut attr_font = gtk4::pango::AttrFontDesc::new(&font_desc);
//     attr_font.set_start_index(0);
//     attr_font.set_end_index(gtk4::pango::ATTR_INDEX_TO_TEXT_END);
//     attrs.insert(attr_font);
//
//     // Color
//     if let Some((r, g, b, a)) = conv_optional_color(color) {
//         let red = (r * 65535.0) as u16;
//         let green = (g * 65535.0) as u16;
//         let blue = (b * 65535.0) as u16;
//         let alpha = (a * 65535.0) as u16;
//
//         let mut attr_fg = gtk4::pango::AttrColor::new_foreground(red, green, blue);
//         attr_fg.set_start_index(0);
//         attr_fg.set_end_index(gtk4::pango::ATTR_INDEX_TO_TEXT_END);
//         attrs.insert(attr_fg);
//
//         let mut attr_alpha = gtk4::pango::AttrInt::new_foreground_alpha(alpha);
//         attr_alpha.set_start_index(0);
//         attr_alpha.set_end_index(gtk4::pango::ATTR_INDEX_TO_TEXT_END);
//         attrs.insert(attr_alpha);
//     }
//
//     // Decoration - Underline
//     let underline_style = conv_underline_style(&decoration.underline.style);
//     if underline_style != gtk4::pango::Underline::None {
//         let mut attr_u = gtk4::pango::AttrInt::new_underline(underline_style);
//         attr_u.set_start_index(0);
//         attr_u.set_end_index(gtk4::pango::ATTR_INDEX_TO_TEXT_END);
//         attrs.insert(attr_u);
//
//         if let Some((r, g, b, _)) = conv_optional_color(&decoration.underline.color) {
//             let red = (r * 65535.0) as u16;
//             let green = (g * 65535.0) as u16;
//             let blue = (b * 65535.0) as u16;
//             let mut attr_uc = gtk4::pango::AttrColor::new_underline_color(red, green, blue);
//             attr_uc.set_start_index(0);
//             attr_uc.set_end_index(gtk4::pango::ATTR_INDEX_TO_TEXT_END);
//             attrs.insert(attr_uc);
//         }
//     }
//
//     // Decoration - Strikethrough
//     if decoration.strike_through.style.clone() != crate::elements::TextLineStyle::None {
//         let mut attr_s = gtk4::pango::AttrInt::new_strikethrough(true);
//         attr_s.set_start_index(0);
//         attr_s.set_end_index(gtk4::pango::ATTR_INDEX_TO_TEXT_END);
//         attrs.insert(attr_s);
//
//         if let Color::Custom { r, g, b, a: _ } = decoration.strike_through.color {
//             let red = (r * 65535.0) as u16;
//             let green = (g * 65535.0) as u16;
//             let blue = (b * 65535.0) as u16;
//             let mut attr_sc = gtk4::pango::AttrColor::new_strikethrough_color(red, green, blue);
//             attr_sc.set_start_index(0);
//             attr_sc.set_end_index(gtk4::pango::ATTR_INDEX_TO_TEXT_END);
//             attrs.insert(attr_sc);
//         }
//     }
//
//     (attrs, font_desc)
// }

pub fn conv_create_background_color_class(color: &Color) -> String {
    let value = match color {
        Color::Custom { r, g, b, a } => {
            format!(
                "rgba({}, {}, {}, {})",
                r * 255.0,
                g * 255.0,
                b * 255.0,
                a * 255.0
            )
        }
        Color::SystemRed => "red".to_string(),
        Color::SystemGreen => "green".to_string(),
        Color::SystemBlue => "blue".to_string(),
        Color::SystemYellow => "yellow".to_string(),
        Color::SystemOrange => "orange".to_string(),
        Color::SystemPurple => "purple".to_string(),
        Color::SystemPink => "pink".to_string(),
        Color::SystemIndigo => "indigo".to_string(),
        Color::Text => "black".to_string(),
    };

    let class = randomizer::Randomizer::ALPHABETICAL(8).string().unwrap();
    let css = format!(".{} {{ background-color: {} }}", class, value);

    let provider = CssProvider::new();
    provider.load_from_data(css.as_str());

    gtk4::style_context_add_provider_for_display(
        &Display::default().unwrap(),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
    class
}
