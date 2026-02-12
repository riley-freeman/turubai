use crate::pal::apple::{stack, text, Context};
use crate::shadow::ShadowTree;
use crate::shadow::{NodeKind, ShadowNode};
use crate::Unit;

pub fn request_dimensions(
    node: &ShadowNode,
    context: Context,
    available_width: f64,
    available_height: f64,
) -> (Unit, Unit) {
    match node.kind.clone() {
        NodeKind::HStack {
            spacing,
            alignment: _,
        } => stack::h_stack_request_dimensions(
            node,
            context.clone(),
            available_width,
            available_height,
            spacing.to_pixels(None),
        ),

        NodeKind::VStack {
            spacing,
            alignment: _,
        } => stack::v_stack_request_dimensions(
            node,
            context.clone(),
            available_width,
            available_height,
            spacing.to_pixels(None),
        ),

        NodeKind::Text { .. } => {
            text::request_dimensions(node, context.clone(), available_width, available_height)
        }

        NodeKind::Spacer => (Unit::Percent(1.0), Unit::Percent(1.0)),

        NodeKind::Window { title: _ } | NodeKind::BackgroundColor { .. } => request_dimensions(
            node.children.get(0).unwrap(),
            context.clone(),
            available_width,
            available_height,
        ),

        NodeKind::Padding {
            top,
            left,
            bottom,
            right,
        } => {
            let (w, h) = request_dimensions(
                node.children.get(0).unwrap(),
                context.clone(),
                available_width,
                available_height,
            );
            let w = w.to_pixels(Some(available_width));
            let h = h.to_pixels(Some(available_height));
            (
                Unit::Pixels(w + left + right),
                Unit::Pixels(h + top + bottom),
            )
        }

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
        } => stack::h_stack_minimum_dimensions(
            node,
            context.clone(),
            available_width,
            spacing.to_pixels(None),
        ),

        NodeKind::VStack {
            spacing,
            alignment: _,
        } => stack::v_stack_minimum_dimensions(
            node,
            context.clone(),
            available_width,
            spacing.to_pixels(None),
        ),

        NodeKind::Text { .. } => {
            let (w, h) = text::request_dimensions(node, context.clone(), available_width, 0.0);
            (w.to_pixels(None), h.to_pixels(None))
        }

        // Spacers have 0 minimum size
        NodeKind::Spacer => (0.0, 0.0),

        NodeKind::Window { title: _ } | NodeKind::BackgroundColor { .. } => {
            request_minimum_dimensions(
                node.children.get(0).unwrap(),
                context.clone(),
                available_width,
            )
        }

        NodeKind::Padding {
            top,
            left,
            bottom,
            right,
        } => {
            let (w, h) = request_minimum_dimensions(
                node.children.get(0).unwrap(),
                context.clone(),
                available_width - left - right,
            );
            (w + left + right, h + top + bottom)
        }

        _ => (0.0, 0.0),
    }
}

pub fn update_node_sizes(
    node: &ShadowNode,
    tree: &ShadowTree,
    context: Context,
    available_width: f64,
    available_height: f64,
) -> (bool, bool) {
    // Only set explicit sizes on leaf nodes (Text) that need intrinsic sizing.
    // Stacks and Spacers use flex layout and shouldn't have explicit sizes.
    //
    // Returns (needs_full_width, needs_full_height)
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
            (false, false)
        }

        NodeKind::Spacer => {
            // Spacers use flex_grow, no explicit size needed
            // But they request full size
            (true, true)
        }

        NodeKind::VStack { .. } => {
            let mut needs_full_height = false;
            let mut needs_full_width = false;
            for child in &node.children {
                let (child_w, child_h) = update_node_sizes(
                    child,
                    tree,
                    context.clone(),
                    available_width,
                    available_height,
                );

                if child_w {
                    needs_full_width = true;
                }
                if child_h {
                    needs_full_height = true;
                }
            }

            let width_dim = if needs_full_width {
                taffy::Dimension::percent(1.0)
            } else {
                taffy::Dimension::auto()
            };
            let height_dim = if needs_full_height {
                taffy::Dimension::percent(1.0)
            } else {
                taffy::Dimension::auto()
            };

            if needs_full_width || needs_full_height {
                tree.set_size(node.taffy_id, width_dim, height_dim);
            }

            (needs_full_width, needs_full_height)
        }

        NodeKind::HStack { .. } => {
            let mut needs_full_width = false;
            let mut needs_full_height = false;
            for child in &node.children {
                let (child_w, child_h) = update_node_sizes(
                    child,
                    tree,
                    context.clone(),
                    available_width,
                    available_height,
                );

                if child_w {
                    needs_full_width = true;
                }
                if child_h {
                    needs_full_height = true;
                }
            }

            let width_dim = if needs_full_width {
                taffy::Dimension::percent(1.0)
            } else {
                taffy::Dimension::auto()
            };
            let height_dim = if needs_full_height {
                taffy::Dimension::percent(1.0)
            } else {
                taffy::Dimension::auto()
            };

            if needs_full_width || needs_full_height {
                tree.set_size(node.taffy_id, width_dim, height_dim);
            }

            (needs_full_width, needs_full_height)
        }

        NodeKind::Window { .. } => update_node_sizes(
            node.children.get(0).unwrap(),
            tree,
            context.clone(),
            available_width,
            available_height,
        ),

        NodeKind::BackgroundColor { .. } => {
            // Background color wrapper: propagate child's size requirements
            let child = node
                .children
                .get(0)
                .expect("BackgroundColor must have a child");
            let (needs_full_width, needs_full_height) = update_node_sizes(
                child,
                tree,
                context.clone(),
                available_width,
                available_height,
            );

            // Set size on the background color node to match child requirements
            let width_dim = if needs_full_width {
                taffy::Dimension::percent(1.0)
            } else {
                taffy::Dimension::auto()
            };
            let height_dim = if needs_full_height {
                taffy::Dimension::percent(1.0)
            } else {
                taffy::Dimension::auto()
            };

            if needs_full_width || needs_full_height {
                tree.set_size(node.taffy_id, width_dim, height_dim);
            }

            (needs_full_width, needs_full_height)
        }

        NodeKind::Padding { .. } => {
            // Background color wrapper: propagate child's size requirements
            let child = node
                .children
                .get(0)
                .expect("BackgroundColor must have a child");
            let (needs_full_width, needs_full_height) = update_node_sizes(
                child,
                tree,
                context.clone(),
                available_width,
                available_height,
            );

            // Set size on the background color node to match child requirements
            let width_dim = if needs_full_width {
                taffy::Dimension::percent(1.0)
            } else {
                taffy::Dimension::auto()
            };
            let height_dim = if needs_full_height {
                taffy::Dimension::percent(1.0)
            } else {
                taffy::Dimension::auto()
            };

            if needs_full_width || needs_full_height {
                tree.set_size(node.taffy_id, width_dim, height_dim);
            }

            (needs_full_width, needs_full_height)
        }

        _ => {
            // Other containers: recurse into children
            let mut needs_full_width = false;
            let mut needs_full_height = false;
            for child in &node.children {
                let (child_w, child_h) = update_node_sizes(
                    child,
                    tree,
                    context.clone(),
                    available_width,
                    available_height,
                );

                if child_w {
                    needs_full_width = true;
                }
                if child_h {
                    needs_full_height = true;
                }
            }
            (needs_full_width, needs_full_height)
        }
    }
}
