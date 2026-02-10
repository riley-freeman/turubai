use cacao::color::Color;
use cacao::layout::Layout;
use cacao::text::Label;
use cacao::view::View;

use crate::pal::apple::{Context, NativeView};
use crate::shadow::{NodeKind, ShadowNode, ShadowTree};
use crate::units::{Percent, Pixels};
use crate::Unit;

pub fn h_stack_request_dimensions(
    node: &ShadowNode,
    context: Context,
    available_width: f64,
    available_height: f64,
    spacing: f64,
) -> (Box<dyn Unit>, Box<dyn Unit>) {
    let mut width = 0.0_f64;
    let mut max_height = 0.0_f64;
    let mut remaining_width = available_width;
    let mut spacer_count = 0_usize;

    // First pass: measure non-spacer children
    for child in &node.children {
        if let NodeKind::Spacer = child.kind {
            spacer_count += 1;
            continue;
        }

        let (child_width, child_height) = crate::pal::apple::measure::request_dimensions(
            child,
            context.clone(),
            remaining_width,
            available_height,
        );

        let cw = child_width.to_pixels(Some(remaining_width));
        let ch = child_height.to_pixels(Some(available_height));

        if ch > max_height {
            max_height = ch;
        }

        width += cw;
        remaining_width -= cw + spacing;
    }

    // Add spacing between children
    if node.children.len() > 1 {
        width += spacing * (node.children.len() - 1) as f64;
    }

    // If there are spacers, the stack takes full available width (percentage-based)
    if spacer_count > 0 {
        return (Percent::new(1.0), Pixels::new(max_height));
    }

    (Pixels::new(width), Pixels::new(max_height))
}

pub fn render_h_stack(node: &ShadowNode, tree: &ShadowTree, context: Context) -> NativeView {
    eprintln!("[DEBUG] rendering Horizontal Stack");

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
    available_width: f64,
    available_height: f64,
    spacing: f64,
) -> (Box<dyn Unit>, Box<dyn Unit>) {
    let mut max_width = 0.0_f64;
    let mut height = 0.0_f64;
    let mut remaining_height = available_height;
    let mut spacer_count = 0_usize;

    // First pass: measure non-spacer children
    for child in &node.children {
        if let NodeKind::Spacer = child.kind {
            spacer_count += 1;
            continue;
        }

        let (child_width, child_height) = crate::pal::apple::measure::request_dimensions(
            child,
            context.clone(),
            available_width,
            remaining_height,
        );

        let cw = child_width.to_pixels(Some(available_width));
        let ch = child_height.to_pixels(Some(remaining_height));

        if cw > max_width {
            max_width = cw;
        }

        height += ch;
        remaining_height -= ch + spacing;
    }

    // Add spacing between children
    if node.children.len() > 1 {
        height += spacing * (node.children.len() - 1) as f64;
    }

    // If there are spacers, the stack takes full available height (percentage-based)
    if spacer_count > 0 {
        return (Pixels::new(max_width), Percent::new(1.0));
    }

    (Pixels::new(max_width), Pixels::new(height))
}

/// Calculate minimum dimensions (spacers treated as 0 size)
pub fn v_stack_minimum_dimensions(
    node: &ShadowNode,
    context: Context,
    available_width: f64,
    spacing: f64,
) -> (f64, f64) {
    let mut max_width = 0.0_f64;
    let mut height = 0.0_f64;
    let mut child_count = 0_usize;

    for child in &node.children {
        // Skip spacers - they don't contribute to minimum size
        if let NodeKind::Spacer = child.kind {
            child_count += 1;
            continue;
        }

        let (child_width, child_height) = crate::pal::apple::measure::request_minimum_dimensions(
            child,
            context.clone(),
            available_width,
        );

        if child_width > max_width {
            max_width = child_width;
        }

        height += child_height;
        child_count += 1;
    }

    // Add spacing between children
    if child_count > 1 {
        height += spacing * (child_count - 1) as f64;
    }

    (max_width, height)
}

/// Calculate minimum dimensions for HStack (spacers treated as 0 size)
pub fn h_stack_minimum_dimensions(
    node: &ShadowNode,
    context: Context,
    available_width: f64,
    spacing: f64,
) -> (f64, f64) {
    let mut width = 0.0_f64;
    let mut max_height = 0.0_f64;
    let mut child_count = 0_usize;

    for child in &node.children {
        // Skip spacers - they don't contribute to minimum size
        if let NodeKind::Spacer = child.kind {
            child_count += 1;
            continue;
        }

        let (child_width, child_height) = crate::pal::apple::measure::request_minimum_dimensions(
            child,
            context.clone(),
            available_width,
        );

        if child_height > max_height {
            max_height = child_height;
        }

        width += child_width;
        child_count += 1;
    }

    // Add spacing between children
    if child_count > 1 {
        width += spacing * (child_count - 1) as f64;
    }

    (width, max_height)
}

pub fn render_v_stack(node: &ShadowNode, tree: &ShadowTree, context: Context) -> NativeView {
    eprintln!("[DEBUG] rendering Vertical Stack");

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

pub fn render_spacer() -> NativeView {
    let view = View::new();
    view.set_translates_autoresizing_mask_into_constraints(true);
    NativeView::Spacer { view }
}
