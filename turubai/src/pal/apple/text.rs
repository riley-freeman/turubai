use std::ffi::c_void;
use std::sync::Arc;

use cacao::core_foundation::base::{CFType, TCFType};
use cacao::core_foundation::boolean::CFBoolean;
use cacao::core_foundation::number::CFNumber;
use cacao::core_foundation::string::CFString;
use cacao::core_graphics::display::{CFDictionary, CGSize};
use cacao::foundation::NSNumber;
use cacao::layout::Layout;
use cacao::objc::runtime::Object;
use cacao::text::{AttributedString, Label};
use cacao::view::View;
use objc_id::Id;

use crate::color::Color;
use crate::elements::{TextDecoration, TextDecorationLine, TextLineStyle};
use crate::font::Font;
use crate::pal::apple::color::NativeColor;
use crate::pal::apple::{Context, NativeView};
use crate::shadow::{NodeKind, ShadowNode};

use cacao::objc::{class, msg_send, sel, sel_impl};

pub fn request_dimensions(
    node: &ShadowNode,
    context: Context,
    preferred_width: f64,
    preferred_height: f64,
) -> (f64, f64) {
    if let NodeKind::Text { content, font, .. } = &node.kind {
        let font = context.get_native_font(font);
        let label = Label::new();
        label.set_text(content);
        label.set_font(font.os_font());

        label.objc.get(|handle| unsafe {
            let size: CGSize = msg_send![handle, intrinsicContentSize];
            (size.width, size.height)
        })
    } else {
        unimplemented!()
    }
}

pub fn render_text(
    content: &str,
    font: &Font,
    color: &Color,
    decoration: &TextDecoration,
    node: &ShadowNode,
    context: Context,
) -> NativeView {
    eprintln!("[DEBUG] rendering label: \"{}\"", content);

    let native_font = context.get_native_font(font);
    let native_color = context.get_native_color(color);

    let cf_content = CFString::new(content);

    let single_line = CFNumber::from(1);
    let thick_line = CFNumber::from(2);
    let double_line = CFNumber::from(9);
    let dotted_line = CFNumber::from(257);
    let dashed_line = CFNumber::from(513);

    let underline_attr = CFString::new("NSUnderline");
    let underline_color_attr = CFString::new("NSUnderlineColor");
    let strike_through_attr = CFString::new("NSStrikethrough");
    let strike_through_color_attr = CFString::new("NSStrikethroughColor");

    let mut attributes: Vec<(CFType, CFType)> = Vec::new();
    let (_flags, underline_color) = if decoration.underline.style != TextLineStyle::None {
        let (flags, uc) = handle_text_decoration(
            &context,
            &decoration.underline,
            underline_attr.as_CFType(),
            underline_color_attr.as_CFType(),
            &mut attributes,
        );
        (flags, Some(uc))
    } else {
        (Box::new(CFNumber::from(0)), None)
    };

    let (_flags, strike_through_color) = if decoration.strike_through.style != TextLineStyle::None {
        let (flags, stc) = handle_text_decoration(
            &context,
            &decoration.strike_through,
            strike_through_attr.as_CFType(),
            strike_through_color_attr.as_CFType(),
            &mut attributes,
        );
        (flags, Some(stc))
    } else {
        (Box::new(CFNumber::from(0)), None)
    };

    let attr_dict = CFDictionary::from_CFType_pairs(&attributes);
    let attrbuted_text: AttributedString = unsafe {
        let attributed_string_class = class!(NSAttributedString);
        let instance: AttributedString = msg_send![attributed_string_class, alloc];
        let _: c_void = msg_send![instance, initWithString: cf_content attributes: attr_dict];
        instance
    };

    let label = Label::new();
    label.set_attributed_text(attrbuted_text);
    label.set_font(native_font.os_font());
    label.set_text_color(native_color.os_color());

    let wrapper = View::new();
    wrapper.add_subview(&label);
    wrapper.set_translates_autoresizing_mask_into_constraints(true);

    NativeView::Text {
        wrapper,
        _font: native_font,
        _label: label,
        underline_color,
        strike_through_color,
    }
}

fn handle_text_decoration(
    context: &Context,
    decoration: &TextDecorationLine,
    style_target: CFType,
    color_target: CFType,
    attributes: &mut Vec<(CFType, CFType)>,
) -> (Box<CFNumber>, Arc<NativeColor>) {
    let flags: u32 = match decoration.style {
        TextLineStyle::None => 0,
        TextLineStyle::Single => 1,
        TextLineStyle::Thick => 2,
        TextLineStyle::Double => 9,
        TextLineStyle::Dotted => 256,
        TextLineStyle::Dashed => 512,
    };

    let native_color = context.get_native_color(&decoration.color);
    attributes.push((color_target, native_color.os_color().cg_color().as_CFType()));

    let cf_flags = CFNumber::from(flags as i64);
    let boxed_cf_flags = Box::new(cf_flags);
    attributes.push((style_target, boxed_cf_flags.as_CFType()));

    (boxed_cf_flags, native_color)
}
