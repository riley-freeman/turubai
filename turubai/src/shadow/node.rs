use taffy::{FlexDirection, NodeId, Style};

use crate::{
    color::Color,
    composition::{HorizontalAlignment, VerticalAlignment},
    elements::TextDecoration,
    font::Font,
    shadow::conv::{conv_h_alignment, conv_v_alignment},
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
        title: Option<String>,
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
        spacing: f64,
        alignment: VerticalAlignment,
    },
    /// A vertical stack (VStack)
    VStack {
        spacing: f64,
        alignment: HorizontalAlignment,
    },
    Spacer,
    BackgroundColor {
        color: Color,
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

    pub fn hstack(spacing: f64, alignment: VerticalAlignment) -> Self {
        Self {
            kind: NodeKind::HStack { spacing, alignment },
            style: Style {
                flex_direction: FlexDirection::Row,
                align_items: Some(conv_v_alignment(alignment)),
                gap: taffy::Size {
                    width: taffy::LengthPercentage::length(spacing as _),
                    height: taffy::LengthPercentage::length(0.0),
                },
                ..Default::default()
            },
        }
    }

    pub fn vstack(spacing: f64, alignment: HorizontalAlignment) -> Self {
        Self {
            kind: NodeKind::VStack { spacing, alignment },
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: Some(conv_h_alignment(alignment)),
                gap: taffy::Size {
                    width: taffy::LengthPercentage::length(0.0),
                    height: taffy::LengthPercentage::length(spacing as _),
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

    pub fn window(title: Option<String>) -> Self {
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
