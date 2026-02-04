use crate::pal::apple::{stack, text, Context};
use crate::shadow::ShadowTree;
use crate::shadow::{NodeKind, ShadowNode};
use core::f64;

/// CGSize from Core Graphics - matches the Objective-C struct
// #[repr(C)]
// #[derive(Debug, Copy, Clone, Default)]
// pub struct CGSize {
//     width: f64,
//     height: f64,
// }

pub fn request_dimensions(
    node: &ShadowNode,
    context: Context,
    preferred_width: f64,
    preferred_height: f64,
) -> (f64, f64) {
    match node.kind.clone() {
        NodeKind::HStack {
            spacing,
            alignment: _,
        } => stack::h_stack_request_dimensions(
            node,
            context.clone(),
            preferred_width,
            preferred_height,
            spacing,
        ),

        NodeKind::VStack {
            spacing,
            alignment: _,
        } => stack::v_stack_request_dimensions(
            node,
            context.clone(),
            preferred_width,
            preferred_height,
            spacing,
        ),

        NodeKind::Text { content, font, .. } => {
            text::request_dimensions(node, context.clone(), preferred_width, preferred_height)
        }

        NodeKind::Window { title: _ } => request_dimensions(
            node.children.get(0).unwrap(),
            context.clone(),
            preferred_width,
            preferred_height,
        ),

        _ => {
            unimplemented!()
        }
    }
}

pub fn update_node_sizes(
    node: &ShadowNode,
    tree: &ShadowTree,
    context: Context,
    available_width: f64,
    available_height: f64,
) {
    let (width, height) =
        request_dimensions(node, context.clone(), available_width, available_height);
    tree.set_size(
        node.taffy_id,
        taffy::Dimension::length(width as _),
        taffy::Dimension::length(height as _),
    );

    // Update the children.
    for child in &node.children {
        update_node_sizes(
            child,
            tree,
            context.clone(),
            available_width,
            available_height,
        );
    }
}
