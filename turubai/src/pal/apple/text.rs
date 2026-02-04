use cacao::core_graphics::display::CGSize;
use cacao::layout::Layout;
use cacao::text::Label;
use cacao::view::View;

use crate::color::Color;
use crate::font::Font;
use crate::pal::apple::{Context, NativeView};
use crate::shadow::{NodeKind, ShadowNode};

use cacao::objc::{msg_send, sel, sel_impl};

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
    node: &ShadowNode,
    context: Context,
) -> NativeView {
    eprintln!("[DEBUG] rendering label: \"{}\"", content);

    let native_font = context.get_native_font(font);
    let native_color = context.get_native_color(color);

    let label = Label::new();
    label.set_text(content);
    label.set_font(native_font.os_font());
    label.set_text_color(native_color.os_color());

    let wrapper = View::new();
    wrapper.add_subview(&label);
    wrapper.set_translates_autoresizing_mask_into_constraints(true);
    NativeView::Text {
        wrapper,
        _font: native_font,
        _label: label,
    }
}
