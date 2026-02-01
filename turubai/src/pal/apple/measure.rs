use cacao::text::Label;
use cacao::objc::{msg_send, sel, sel_impl};

use crate::pal::apple::Context;
use crate::shadow::ShadowTree;
use crate::shadow::{NodeKind, ShadowNode};

/// CGSize from Core Graphics - matches the Objective-C struct
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
struct CGSize {
    width: f64,
    height: f64,
}

pub fn get_estimated_size(node: &ShadowNode, context: Context) -> (f64, f64) {
    match node.kind.clone() {
        NodeKind::HStack { spacing, alignment: _ } => {
            let mut width = 0.0;
            let mut max_height = 0.0;
            for child in &node.children {
                let (child_width, child_height) = get_estimated_size(child, context.clone());
                if child_height > max_height {
                    max_height = child_height;
                }
                width += child_width;
            }
            width += spacing as f64 * (node.children.len() - 1) as f64;

            (width, max_height)
        }

        NodeKind::VStack { spacing, alignment: _ } => {
            let mut max_width = 0.0;
            let mut height = 0.0;

            for child in &node.children {
                let (child_width, child_height) = get_estimated_size(child, context.clone());
                if child_width > max_width {
                    max_width = child_width;
                }
                height += child_height;
            }

            height += spacing as f64 * (node.children.len() - 1) as f64;
            (max_width, height)
        }

        NodeKind::Text { content, font} => {
            let font = context.get_native_font(&font);
            let label = Label::new();
            label.set_text(content);
            label.set_font(font.os_font());

            label.objc.get(|handle|  unsafe {
                let size: CGSize = msg_send![handle, intrinsicContentSize];
                (size.width, size.height)
            })
        }
        
        NodeKind::Window { title: _ } => {
            get_estimated_size(node.children.get(0).unwrap(), context.clone())
        }

        _ => {
            unimplemented!()
        }
    }

}

pub fn update_node_sizes(node: &ShadowNode, tree: &ShadowTree, context: Context) {
    let(width, height) = get_estimated_size(node, context.clone());
    tree.set_size(
        node.taffy_id,
        taffy::Dimension::length(width as _),
        taffy::Dimension::length(height as _)
    );


    // Update the children.
    for child in &node.children {
        update_node_sizes( child, tree, context.clone());
    }
}

