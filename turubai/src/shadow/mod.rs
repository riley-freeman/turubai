mod conv;
mod node;

pub use node::*;

use std::cell::RefCell;
use std::collections::HashMap;
use taffy::{AvailableSpace, Dimension, Layout, LengthPercentage, NodeId, Size, Style, TaffyTree};

/// The shadow tree holds the platform-agnostic representation of the UI.
/// It owns the layout tree (Taffy) and maps layout nodes to shadow nodes.
pub struct ShadowTree {
    /// Taffy layout tree for flexbox computations
    taffy: RefCell<TaffyTree<()>>,
    /// Root node of the shadow tree
    root: Option<ShadowNode>,
    /// Maps Taffy NodeIds to their computed layouts (after layout pass)
    layouts: HashMap<NodeId, Layout>,
}

impl ShadowTree {
    pub fn new() -> Self {
        Self {
            taffy: RefCell::new(TaffyTree::new()),
            root: None,
            layouts: HashMap::new(),
        }
    }

    /// Build the shadow tree from an element tree
    pub fn build_from_element(&mut self, element: &dyn crate::elements::Element) -> &ShadowNode {
        let node = self.create_node_from_element(element);
        self.root = Some(node);
        self.root.as_ref().unwrap()
    }

    /// Create a shadow node from an element
    pub fn create_node_from_element(&self, element: &dyn crate::elements::Element) -> ShadowNode {
        // Get the shadow descriptor from the element
        let descriptor = element.shadow_descriptor();

        // Create child shadow nodes recursively
        let mut children: Vec<ShadowNode> = Vec::with_capacity(element.child_count());
        element.for_each_child(&mut |child| {
            children.push(self.create_node_from_element(child));
        });

        // Create Taffy node for layout
        let child_taffy_ids: Vec<NodeId> = children.iter().map(|c| c.taffy_id).collect();
        let taffy_id = self
            .taffy
            .borrow_mut()
            .new_with_children(descriptor.style.clone(), &child_taffy_ids)
            .expect("Failed to create taffy node");

        ShadowNode {
            taffy_id,
            kind: descriptor.kind,
            style: descriptor.style,
            children,
        }
    }

    /// Compute layout for the entire tree
    pub fn compute_layout(
        &mut self,
        root: &ShadowNode,
        available_width: f32,
        available_height: f32,
    ) {
        let available = taffy::Size {
            width: taffy::AvailableSpace::Definite(available_width),
            height: taffy::AvailableSpace::Definite(available_height),
        };

        self.taffy
            .borrow_mut()
            .compute_layout(root.taffy_id, available)
            .expect("Failed to compute layout");

        // Cache computed layouts
        self.cache_layouts_recursive(&root);
    }

    fn cache_layouts_recursive(&mut self, node: &ShadowNode) {
        if let Ok(layout) = self.taffy.borrow().layout(node.taffy_id) {
            self.layouts.insert(node.taffy_id, *layout);
        }
        for child in &node.children {
            self.cache_layouts_recursive(child);
        }
    }

    pub fn set_size(&self, id: NodeId, width: Dimension, height: Dimension) {
        let mut style = {
            let tree = self.taffy.borrow_mut();
            tree.style(id).unwrap().clone()
        };
        style.size = Size { width, height };
        let _ = self.taffy.borrow_mut().set_style(id, style);
    }

    /// Get the computed layout for a node
    pub fn get_layout(&self, taffy_id: NodeId) -> Option<&Layout> {
        self.layouts.get(&taffy_id)
    }

    /// Get the root node
    pub fn root(&self) -> Option<&ShadowNode> {
        self.root.as_ref()
    }
}

impl Default for ShadowTree {
    fn default() -> Self {
        Self::new()
    }
}

#[test]
fn test_centering() {
    let mut tree = taffy::TaffyTree::<()>::new();

    let child = tree.new_with_children(Style::default(), &[]).unwrap();
    let h_stack = tree
        .new_with_children(
            Style {
                justify_content: Some(taffy::JustifyContent::Center),
                ..Default::default()
            },
            &[child],
        )
        .unwrap();

    let v_stack = tree
        .new_with_children(
            Style {
                justify_content: Some(taffy::JustifyContent::Center),
                ..Default::default()
            },
            &[h_stack],
        )
        .unwrap();

    tree.compute_layout(
        v_stack,
        Size {
            width: AvailableSpace::Definite(800.0),
            height: AvailableSpace::Definite(600.0),
        },
    )
    .unwrap();

    let v = tree.layout(v_stack).unwrap();
    assert_eq!(v.size.width, 800.0);
    assert_eq!(v.size.height, 600.0);
}

#[test]
fn test_background_layout() {
    use crate::color::Color;
    use crate::elements::Element;
    use crate::shadow::ShadowDescriptor;

    // Define a simple mock element
    struct MockElement;
    impl Element for MockElement {
        fn name(&self) -> &'static str {
            "mock"
        }
        fn display_name(&self) -> &'static str {
            "Mock"
        }
        fn shadow_descriptor(&self) -> ShadowDescriptor {
            ShadowDescriptor::view()
        }
    }

    let child = Box::new(MockElement);
    let bg = crate::postprocessing::BackgroundColor::new(&Color::SystemRed, child);

    let tree = ShadowTree::new();
    let root_node = tree.create_node_from_element(&bg);

    // Verify it is a BackgroundColor node
    if let NodeKind::BackgroundColor { color } = &root_node.kind {
        // Verify the color is correct
        assert!(matches!(color, Color::SystemRed));
    } else {
        panic!("Root node is not BackgroundColor");
    }

    // Verify BackgroundColor has exactly one child
    assert_eq!(
        root_node.children.len(),
        1,
        "BackgroundColor should have 1 child"
    );

    // Verify the style uses Column flex direction for proper child layout
    let taffy = tree.taffy.borrow();
    let style = taffy.style(root_node.taffy_id).unwrap();
    assert_eq!(
        style.flex_direction,
        taffy::FlexDirection::Column,
        "BackgroundColor should use Column flex direction"
    );
    assert_eq!(
        style.align_items,
        Some(taffy::AlignItems::Stretch),
        "BackgroundColor should stretch children horizontally"
    );
}
