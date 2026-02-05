use crate::pal::apple::{stack, text, Context};
use crate::shadow::ShadowTree;
use crate::shadow::{NodeKind, ShadowNode};
use crate::units::Pixels;
use crate::Unit;

pub fn request_dimensions(
    node: &ShadowNode,
    context: Context,
    available_width: f64,
    available_height: f64,
) -> (Box<dyn Unit>, Box<dyn Unit>) {
    match node.kind.clone() {
        NodeKind::HStack {
            spacing,
            alignment: _,
        } => stack::h_stack_request_dimensions(
            node,
            context.clone(),
            available_width,
            available_height,
            spacing,
        ),

        NodeKind::VStack {
            spacing,
            alignment: _,
        } => stack::v_stack_request_dimensions(
            node,
            context.clone(),
            available_width,
            available_height,
            spacing,
        ),

        NodeKind::Text { .. } => {
            text::request_dimensions(node, context.clone(), available_width, available_height)
        }

        NodeKind::Spacer => (
            Box::new(Pixels::new(available_width)),
            Box::new(Pixels::new(available_height)),
        ),

        NodeKind::Window { title: _ } => request_dimensions(
            node.children.get(0).unwrap(),
            context.clone(),
            available_width,
            available_height,
        ),

        _ => {
            unimplemented!()
        }
    }
}

/// Calculate minimum dimensions (spacers treated as 0 size)
pub fn request_minimum_dimensions(
    node: &ShadowNode,
    context: Context,
    available_width: f64,
) -> (f64, f64) {
    match node.kind.clone() {
        NodeKind::HStack {
            spacing,
            alignment: _,
        } => stack::h_stack_minimum_dimensions(node, context.clone(), available_width, spacing),

        NodeKind::VStack {
            spacing,
            alignment: _,
        } => stack::v_stack_minimum_dimensions(node, context.clone(), available_width, spacing),

        NodeKind::Text { .. } => {
            let (w, h) = text::request_dimensions(node, context.clone(), available_width, 0.0);
            (w.to_pixels(None), h.to_pixels(None))
        }

        // Spacers have 0 minimum size
        NodeKind::Spacer => (0.0, 0.0),

        NodeKind::Window { title: _ } => request_minimum_dimensions(
            node.children.get(0).unwrap(),
            context.clone(),
            available_width,
        ),

        _ => (0.0, 0.0),
    }
}

pub fn update_node_sizes(
    node: &ShadowNode,
    tree: &ShadowTree,
    context: Context,
    available_width: f64,
    available_height: f64,
) {
    // Only set explicit sizes on leaf nodes (Text) that need intrinsic sizing.
    // Stacks and Spacers use flex layout and shouldn't have explicit sizes.
    match &node.kind {
        NodeKind::Text { .. } => {
            let (width, height) =
                request_dimensions(node, context.clone(), available_width, available_height);
            let w = width.to_pixels(Some(available_width));
            let h = height.to_pixels(Some(available_height));
            tree.set_size(
                node.taffy_id,
                taffy::Dimension::length(w as f32),
                taffy::Dimension::length(h as f32),
            );
        }

        NodeKind::Spacer => {
            // Spacers use flex_grow, no explicit size needed
        }

        NodeKind::VStack { .. } => {
            let mut has_spacer = false;
            for child in &node.children {
                update_node_sizes(
                    child,
                    tree,
                    context.clone(),
                    available_width,
                    available_height,
                );
                if let NodeKind::Spacer = child.kind {
                    has_spacer = true;
                }
            }

            if has_spacer {
                tree.set_size(
                    node.taffy_id,
                    taffy::Dimension::auto(),
                    taffy::Dimension::percent(1.0),
                );
            }
        }

        _ => {
            // Other containers: recurse into children
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
    }
}
