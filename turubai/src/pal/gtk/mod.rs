use std::cell::RefCell;
use std::rc::Rc;

use gtk4::{
    gio::prelude::{ApplicationExt, ApplicationExtManual},
    prelude::*,
    Fixed, Widget,
};

use crate::{
    pal::DynContext,
    shadow::{NodeKind, ShadowNode, ShadowTree},
    Application, Backend,
};

mod conv;
mod measure;

pub struct API;

impl crate::pal::API for API {
    const VARIANT: Backend = Backend::Apple;
    type Context = Context;
}

static DEFAULT_WINDOW_WIDTH: i32 = 800;
static DEFAULT_WINDOW_HEIGHT: i32 = 600;

/// Holds references to native GTK widgets so they can be repositioned
/// after layout recomputation (e.g. on window resize).
enum NativeWidget {
    /// A Fixed container with absolutely-positioned children
    Container {
        container: Fixed,
        children: Vec<NativeWidget>,
    },
    /// A text label
    Text { label: gtk4::Label },
    /// A spacer (flexible empty space)
    Spacer { widget: gtk4::Box },
}

impl NativeWidget {
    /// Get the underlying GTK widget
    fn widget(&self) -> gtk4::Widget {
        match self {
            NativeWidget::Container { container, .. } => container.clone().into(),
            NativeWidget::Text { label } => label.clone().into(),
            NativeWidget::Spacer { widget } => widget.clone().into(),
        }
    }

    /// Recursively update positions and sizes from the shadow tree layout.
    /// This is called after layout recomputation (e.g. on resize).
    fn update_frames(&self, node: &ShadowNode, tree: &ShadowTree) {
        match self {
            NativeWidget::Text { label } => {
                if let Some(layout) = tree.get_layout(node.taffy_id) {
                    label.set_size_request(layout.size.width as i32, layout.size.height as i32);
                }
            }
            NativeWidget::Container {
                container,
                children,
            } => {
                // Don't set_size_request on Fixed containers — they position
                // children absolutely and don't need a minimum size. Setting it
                // would cause GTK to resize the window, creating a feedback loop.

                for (child_widget, child_node) in children.iter().zip(node.children.iter()) {
                    // Reposition child within the Fixed container
                    if let Some(child_layout) = tree.get_layout(child_node.taffy_id) {
                        let transform =
                            gtk4::gsk::Transform::new().translate(&gtk4::graphene::Point::new(
                                child_layout.location.x,
                                child_layout.location.y,
                            ));
                        container.set_child_transform(&child_widget.widget(), Some(&transform));
                    }

                    // Recursively update the child's own frames
                    child_widget.update_frames(child_node, tree);
                }
            }
            NativeWidget::Spacer { widget } => {
                if let Some(layout) = tree.get_layout(node.taffy_id) {
                    widget.set_size_request(layout.size.width as i32, layout.size.height as i32);
                }
            }
        }
    }
}

pub struct Context {}

impl Context {
    /// Updates the layout of a node and its children in the shadow tree
    /// returns (full_width, full_height) if the node is a spacer and the parent should be 100% of the available space
    pub fn update_layout(
        &self,
        node: &ShadowNode,
        tree: &ShadowTree,
        available_width: f64,
        available_height: f64,
    ) -> (bool, bool) {
        match &node.kind {
            NodeKind::Text { .. } => {
                let (w, h) =
                    measure::request_dimensions(node, self, available_width, available_height);
                tree.set_size(
                    node.taffy_id,
                    taffy::Dimension::length(w.to_pixels(Some(available_width)) as f32),
                    taffy::Dimension::length(h.to_pixels(Some(available_height)) as f32),
                );
                (false, false)
            }
            NodeKind::Spacer => {
                // Spacers are handled by Taffy (flex: 1)
                (true, true)
            }

            NodeKind::HStack { .. } => {
                let mut needs_full_width = false;
                let mut needs_full_height = false;
                for child in &node.children {
                    let (child_w, child_h) =
                        self.update_layout(child, tree, available_width, available_height);

                    needs_full_width = child_w || needs_full_width;
                    needs_full_height = child_h || needs_full_height;
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
                tree.set_size(node.taffy_id, width_dim, height_dim);
                (needs_full_width, needs_full_height)
            }

            NodeKind::VStack { .. } => {
                let mut needs_full_height = false;
                let mut needs_full_width = false;
                for child in &node.children {
                    let (child_w, child_h) =
                        self.update_layout(child, tree, available_width, available_height);

                    needs_full_width = child_w || needs_full_width;
                    needs_full_height = child_h || needs_full_height;
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

                tree.set_size(node.taffy_id, width_dim, height_dim);
                (needs_full_width, needs_full_height)
            }

            NodeKind::BackgroundColor { .. } => {
                let child = node
                    .children
                    .get(0)
                    .expect("BackgroundColor must have a child");
                let (needs_full_width, needs_full_height) =
                    self.update_layout(child, tree, available_width, available_height);

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
                    let (child_w, child_h) =
                        self.update_layout(child, tree, available_width, available_height);

                    needs_full_width = child_w || needs_full_width;
                    needs_full_height = child_h || needs_full_height;
                }
                (needs_full_width, needs_full_height)
            }
        }
    }

    /// Create GTK widgets from the shadow tree WITHOUT applying positions.
    /// Returns a NativeWidget tree that can be positioned separately via update_frames().
    fn render_node(&self, node: &ShadowNode) -> NativeWidget {
        match &node.kind {
            NodeKind::Text {
                content,
                font,
                color,
                decoration,
            } => {
                let label = gtk4::Label::new(Some(content.as_str()));
                let class = conv::conv_create_text_class(font, color, decoration);
                label.set_css_classes(&[class.as_str()]);
                NativeWidget::Text { label }
            }
            NodeKind::VStack { .. } | NodeKind::HStack { .. } => {
                let container = gtk4::Fixed::new();
                let mut children = Vec::new();

                for child in &node.children {
                    let child_native = self.render_node(child);
                    container.put(&child_native.widget(), 0.0, 0.0);
                    children.push(child_native);
                }

                NativeWidget::Container {
                    container,
                    children,
                }
            }
            NodeKind::Spacer => NativeWidget::Spacer {
                widget: gtk4::Box::new(gtk4::Orientation::Horizontal, 0),
            },

            NodeKind::BackgroundColor { color } => {
                let widget = gtk4::Fixed::new();

                let child = node
                    .children
                    .first()
                    .expect("BackgroundColor requires at least one element!");
                let child_native = self.render_node(&child);
                widget.put(&child_native.widget(), 0.0, 0.0);

                let style = conv::conv_create_background_color_class(color);
                widget.style_context().add_class(&style);

                NativeWidget::Container {
                    container: widget,
                    children: vec![child_native],
                }
            }

            _ => NativeWidget::Text {
                label: gtk4::Label::new(Some("Unsupported Node")),
            },
        }
    }
}

impl DynContext for Context {
    type A = API;
    fn takeover(app: Box<dyn Application>) -> ! {
        let gtk_app = gtk4::Application::builder()
            .application_id(app.id())
            .build();

        let window_element = app.markup();

        let shadow_tree = Rc::new(RefCell::new(ShadowTree::new()));

        let mut window_node = shadow_tree
            .borrow()
            .create_node_from_element(window_element.as_ref());
        let root_node = Rc::new(window_node.children.pop().unwrap());

        gtk_app.connect_activate(move |app| {
            if let NodeKind::Window { title } = &window_node.kind {
                let window = gtk4::Window::builder()
                    .application(app)
                    .title(title.clone())
                    .default_width(DEFAULT_WINDOW_WIDTH)
                    .default_height(DEFAULT_WINDOW_HEIGHT)
                    .build();

                let context = Context {};

                // 1. Measure content
                context.update_layout(
                    &root_node,
                    &shadow_tree.borrow(),
                    DEFAULT_WINDOW_WIDTH as f64,
                    DEFAULT_WINDOW_HEIGHT as f64,
                );

                // 2. Compute layout
                shadow_tree.borrow_mut().compute_layout(
                    &root_node,
                    DEFAULT_WINDOW_WIDTH as f32,
                    DEFAULT_WINDOW_HEIGHT as f32,
                );

                // 3. Create widgets (without positions)
                let root_widget = context.render_node(&root_node);

                // 4. Apply positions from computed layout
                root_widget.update_frames(&root_node, &shadow_tree.borrow());

                // Use an Overlay with a DrawingArea as the base to get
                // reliable resize events without feedback loops.
                let overlay = gtk4::Overlay::new();
                let resize_sensor = gtk4::DrawingArea::new();
                resize_sensor.set_hexpand(true);
                resize_sensor.set_vexpand(true);
                overlay.set_child(Some(&resize_sensor));
                overlay.add_overlay(&root_widget.widget());

                window.set_child(Some(&overlay));

                // 5. Handle resize via DrawingArea::connect_resize
                let root_widget = Rc::new(root_widget);
                let shadow_tree = shadow_tree.clone();
                let root_node = root_node.clone();
                resize_sensor.connect_resize(move |_drawing_area, width, height| {
                    if width <= 0 || height <= 0 {
                        return;
                    }

                    let context = Context {};
                    context.update_layout(
                        &root_node,
                        &shadow_tree.borrow(),
                        width as f64,
                        height as f64,
                    );
                    shadow_tree.borrow_mut().compute_layout(
                        &root_node,
                        width as f32,
                        height as f32,
                    );
                    root_widget.update_frames(&root_node, &shadow_tree.borrow());
                });

                window.show();
            }
        });

        std::process::exit(gtk_app.run().into())
    }
}
