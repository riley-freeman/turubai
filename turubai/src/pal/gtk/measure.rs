use gtk4::prelude::WidgetExt;

use crate::{
    pal::gtk::{conv, Context},
    shadow::{NodeKind, ShadowNode},
    units::{Percent, Pixels},
    Unit,
};

pub fn request_dimensions(
    node: &ShadowNode,
    context: &Context,
    available_width: f64,
    available_height: f64,
) -> (Box<dyn Unit>, Box<dyn Unit>) {
    match &node.kind {
        NodeKind::Text {
            content,
            font,
            color,
            decoration,
        } => {
            let label = gtk4::Label::new(Some(content.as_str()));
            let class = conv::conv_create_text_class(&font, &color, &decoration);
            label.set_css_classes(&[&class]);

            let (_, natural_width, _, _) = label.measure(gtk4::Orientation::Horizontal, -1);
            let (_, natural_height, _, _) =
                label.measure(gtk4::Orientation::Vertical, natural_width);

            let width = Pixels::new(natural_width as f64);
            let height = Pixels::new(natural_height as f64);
            (width, height)
        }
        NodeKind::Spacer => (Percent::new(1.0), Percent::new(1.0)),
        NodeKind::HStack { spacing, .. } => {
            let mut width = 0.0_f64;
            let mut max_height = 0.0_f64;

            for child in &node.children {
                let (child_width, child_height) =
                    request_dimensions(child, context.clone(), available_width, available_height);

                let cw = child_width.to_pixels(Some(available_width));
                let ch = child_height.to_pixels(Some(available_height));

                if ch > max_height {
                    max_height = ch;
                }
                width += cw;
            }

            // Add spacing between children
            if node.children.len() > 1 {
                width += spacing * (node.children.len() - 1) as f64;
            }

            (Pixels::new(width), Pixels::new(max_height))
        }
        NodeKind::VStack { spacing, .. } => {
            let mut max_width = 0.0_f64;
            let mut height = 0.0_f64;

            for child in &node.children {
                let (child_width, child_height) =
                    request_dimensions(child, context, available_width, available_height);

                let cw = child_width.to_pixels(Some(available_width));
                let ch = child_height.to_pixels(Some(available_height));

                if cw > max_width {
                    max_width = cw;
                }
                height += ch;
            }

            // Add spacing between children
            if node.children.len() > 1 {
                height += spacing * (node.children.len() - 1) as f64;
            }

            (Pixels::new(max_width), Pixels::new(height))
        }
        _ => (Pixels::new(0.0), Pixels::new(0.0)),
    }
}
