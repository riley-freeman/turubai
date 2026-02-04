use cacao::layout::Layout;
use cacao::view::View;

use crate::pal::apple::{Context, NativeView};
use crate::shadow::{ShadowNode, ShadowTree};

pub fn h_stack_request_dimensions(
    node: &ShadowNode,
    context: Context,
    preferred_width: f64,
    preferred_height: f64,
    spacing: f64,
) -> (f64, f64) {
    let mut width = 0.0;
    let mut max_height = 0.0;
    for child in &node.children {
        let (child_width, child_height) = crate::pal::apple::measure::request_dimensions(
            child,
            context.clone(),
            preferred_width,
            preferred_height,
        );
        if child_height > max_height {
            max_height = child_height;
        }
        width += child_width;
    }
    width += spacing as f64 * (node.children.len() - 1) as f64;

    (width, max_height)
}

pub fn render_h_stack(node: &ShadowNode, tree: &ShadowTree, context: Context) -> NativeView {
    eprintln!("[DEBUG] rendering Horizontal Stack (Column)");

    let rendered: Vec<NativeView> = node
        .children
        .iter()
        .map(|c| Context::render_node(c, tree, context.clone()))
        .collect();

    let view = View::new();
    view.set_translates_autoresizing_mask_into_constraints(true);
    for cv in &rendered {
        view.add_subview(cv.view());
    }

    NativeView::Container {
        view,
        _children: rendered,
    }
}

pub fn v_stack_request_dimensions(
    node: &ShadowNode,
    context: Context,
    preferred_width: f64,
    preferred_height: f64,
    spacing: f64,
) -> (f64, f64) {
    let mut max_width = 0.0;
    let mut height = 0.0;

    for child in &node.children {
        let (child_width, child_height) = crate::pal::apple::measure::request_dimensions(
            child,
            context.clone(),
            preferred_width,
            preferred_height,
        );
        if child_width > max_width {
            max_width = child_width;
        }
        height += child_height;
    }

    height += spacing as f64 * (node.children.len() - 1) as f64;
    (max_width, height)
}

pub fn render_v_stack(node: &ShadowNode, tree: &ShadowTree, context: Context) -> NativeView {
    eprintln!("[DEBUG] rendering Vertical Stack (Row)");

    let rendered: Vec<NativeView> = node
        .children
        .iter()
        .map(|c| Context::render_node(c, tree, context.clone()))
        .collect();

    let view = View::new();
    view.set_translates_autoresizing_mask_into_constraints(true);
    for cv in &rendered {
        view.add_subview(cv.view());
    }

    NativeView::Container {
        view,
        _children: rendered,
    }
}
