use taffy::{FlexDirection, NodeId, Style};

use crate::{
    color::Color,
    composition::{HorizontalAlignment, VerticalAlignment},
    elements::TextDecoration,
    font::Font,
    shadow::conv::{conv_h_alignment, conv_v_alignment},
    Unit,
};

/// A node in the shadow tree - platform agnostic description of a UI element
#[derive(Debug)]
pub struct ShadowNode {
    /// Taffy node ID for layout computation
    pub taffy_id: NodeId,
    /// What kind of node this is
    pub kind: NodeKind,
    /// Layout style (flexbox properties)
    pub style: Style,
    /// Child nodes
    pub children: Vec<ShadowNode>,
}

/// The type of shadow node - describes what native view to create
#[derive(Debug, Clone)]
pub enum NodeKind {
    /// A window container
    Window {
        title: String,
    },
    /// A text label
    Text {
        content: String,
        font: Font,
        color: Color,
        decoration: TextDecoration,
    },
    /// A horizontal stack (HStack)
    HStack {
        spacing: Unit,
        alignment: VerticalAlignment,
    },
    /// A vertical stack (VStack)
    VStack {
        spacing: Unit,
        alignment: HorizontalAlignment,
    },
    Spacer,
    BackgroundColor {
        color: Color,
    },
    Padding {
        top: f64,
        left: f64,
        bottom: f64,
        right: f64,
    },
    Frame {
        max_width: Unit,
        max_height: Unit,
        min_width: Unit,
        min_height: Unit,
        width: Unit,
        height: Unit,
    },
    /// A generic container view
    View,
}

/// Descriptor returned by elements to build their shadow node
pub struct ShadowDescriptor {
    pub kind: NodeKind,
    pub style: Style,
}

impl ShadowDescriptor {
    pub fn text(
        content: impl Into<String>,
        font: Font,
        color: Color,
        decoration: TextDecoration,
    ) -> Self {
        Self {
            kind: NodeKind::Text {
                content: content.into(),
                font,
                color,
                decoration,
            },
            style: Style::default(),
        }
    }

    pub fn hstack(spacing: Unit, alignment: VerticalAlignment) -> Self {
        Self {
            kind: NodeKind::HStack { spacing, alignment },
            style: Style {
                flex_direction: FlexDirection::Row,
                align_items: Some(conv_v_alignment(alignment)),
                gap: taffy::Size {
                    width: spacing.into(),
                    height: taffy::LengthPercentage::length(0.0),
                },
                ..Default::default()
            },
        }
    }

    pub fn vstack(spacing: Unit, alignment: HorizontalAlignment) -> Self {
        Self {
            kind: NodeKind::VStack { spacing, alignment },
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: Some(conv_h_alignment(alignment)),
                gap: taffy::Size {
                    width: taffy::LengthPercentage::length(0.0),
                    height: spacing.into(),
                },
                ..Default::default()
            },
        }
    }

    pub fn spacer() -> Self {
        Self {
            kind: NodeKind::Spacer,
            style: Style {
                flex_grow: 1.0,
                ..Default::default()
            },
        }
    }

    pub fn background_color(color: Color) -> Self {
        Self {
            kind: NodeKind::BackgroundColor { color },
            style: Style {
                // Make the child fill the background color container
                flex_direction: FlexDirection::Column,
                // Stretch child horizontally (cross axis)
                align_items: Some(taffy::AlignItems::Stretch),
                ..Default::default()
            },
        }
    }

    pub fn padding(top: f64, left: f64, bottom: f64, right: f64) -> Self {
        Self {
            kind: NodeKind::Padding {
                top,
                left,
                bottom,
                right,
            },
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: Some(taffy::AlignItems::Stretch),
                padding: taffy::Rect {
                    top: taffy::LengthPercentage::length(top as f32),
                    left: taffy::LengthPercentage::length(left as f32),
                    bottom: taffy::LengthPercentage::length(bottom as f32),
                    right: taffy::LengthPercentage::length(right as f32),
                },
                ..Default::default()
            },
        }
    }

    pub fn window(title: String) -> Self {
        Self {
            kind: NodeKind::Window { title },
            style: Style {
                flex_direction: FlexDirection::Row,
                size: taffy::Size {
                    width: taffy::Dimension::percent(1.0),
                    height: taffy::Dimension::percent(1.0),
                },
                ..Default::default()
            },
        }
    }

    pub fn view() -> Self {
        Self {
            kind: NodeKind::View,
            style: Style::default(),
        }
    }

    /// Modify the style
    pub fn with_style(mut self, f: impl FnOnce(&mut Style)) -> Self {
        f(&mut self.style);
        self
    }
}
