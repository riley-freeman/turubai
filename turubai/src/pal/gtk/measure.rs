use gtk4::prelude::NativeDialogExtManual;

use crate::units::{Percent, Pixels};

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
            let (attrs, _) = Self::create_pango_attr_list(font, color, decoration);
            label.set_attributes(Some(&attrs));

            let (_, natural_width, _, _) = label.measure(gtk4::Orientation::Horizontal, -1);
            let (_, natural_height, _, _) =
                label.measure(gtk4::Orientation::Vertical, natural_width);

            let width = Box::new(Pixels::new(natural_width));
            let height = Box::new(Pixels::new(natural_height));
            (width, height)
        }
        NodeKind::Spacer => (Box::new(Percent::new(1.0)), Box::new(Percent::new(1.0))),
        NodeKind::HStack => {
            let mut width = 0.0_f64;
            let mut max_height = 0.0_f64;

            for child in &node.children {
                let (child_width, child_height) = crate::pal::apple::measure::request_dimensions(
                    child,
                    context.clone(),
                    available_width,
                    available_height,
                );

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

            (
                Box::new(Pixels::new(width)),
                Box::new(Pixels::new(max_height)),
            )
        }
        NodeKind::VStack => {
            let max_width = 0.0_f64;
            let height = 0.0_f64;

            for child in &node.children {
                let (child_width, child_height) = crate::pal::apple::measure::request_dimensions(
                    child,
                    context.clone(),
                    available_width,
                    available_height,
                );

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

            (
                Box::new(Pixels::new(max_width)),
                Box::new(Pixels::new(height)),
            )
        }

        _ => (Box::new(Pixels::new(0.0)), Box::new(Pixels::new(0.0))),
    }
}
