use std::process::exit;
use std::ptr::slice_from_raw_parts;

use cacao::appkit::{App, AppDelegate};
use cacao::appkit::window::{Window, WindowConfig, WindowDelegate, WindowStyle};
use cacao::color::Color;
use cacao::core_foundation::bundle::{CFBundleGetIdentifier, CFBundleGetMainBundle};
use cacao::layout::{Layout, LayoutConstraint};
use cacao::objc::{msg_send, sel, sel_impl};
use cacao::text::Label;
use cacao::view::View;

use libc::strlen;

/// CGSize from Core Graphics - matches the Objective-C struct
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
struct CGSize {
    width: f64,
    height: f64,
}

/// Get the intrinsic content size of a Label using Objective-C runtime
fn get_label_intrinsic_size(label: &Label) -> (f64, f64) {
    // Use ObjcProperty's get() to access the underlying NSTextField
    label.objc.get(|obj| unsafe {
        let size: CGSize = msg_send![obj, intrinsicContentSize];
        (size.width, size.height)
    })
}

use crate::{Application, Backend};
use crate::pal::{apple, DynContext};
use crate::shadow::{NodeKind, ShadowNode, ShadowTree};

pub struct API;

impl crate::pal::API for API {
    const VARIANT: Backend = Backend::Apple;
    type Context = Context;
}

/// Main application context - holds the app and manages windows
pub struct Context {
    user_app: Box<dyn Application>,
    window: Window<TurubaiWindowDelegate>,
}

impl Context {
    fn new(app: Box<dyn Application>) -> Self {
        // Build the shadow tree from the app's markup
        let root_element = app.markup();
        let mut shadow_tree = ShadowTree::new();
        let root_shadow = shadow_tree.create_node_from_element(root_element.as_ref());

        // Create the native window from the shadow tree
        let window = Self::create_window(&root_shadow, &mut shadow_tree);

        Self {
            user_app: app,
            window,
        }
    }

    fn create_window(root: &ShadowNode, tree: &mut ShadowTree) -> Window<TurubaiWindowDelegate> {
        let title = match &root.kind {
            NodeKind::Window { title } => title.clone().unwrap_or_else(|| "Turubai App".to_string()),
            _ => "Turubai App".to_string(),
        };

        let mut config = WindowConfig::default();
        config.set_styles(&[
            WindowStyle::Titled,
            WindowStyle::Closable,
            WindowStyle::Miniaturizable,
            WindowStyle::Resizable,
        ]);

        let delegate = TurubaiWindowDelegate::new(root, tree);
        let window = Window::with(config, delegate);
        window.set_title(&title);
        window.set_minimum_content_size(100.0, 10.0);

        window
    }
}

impl AppDelegate for Context {
    fn did_finish_launching(&self) {
        App::activate();
        self.window.show();
    }

    fn should_terminate_after_last_window_closed(&self) -> bool {
        true
    }
}

impl DynContext for Context {
    type A = apple::API;

    fn takeover(turubai_app: Box<dyn Application>) -> ! {
        let bundle_id = unsafe {
            let bundle = CFBundleGetMainBundle();
            if bundle.is_null() {
                "com.itsjustbox.turubai.unnamed_app"
            } else {
                let os_bundle_id = CFBundleGetIdentifier(bundle);
                if !os_bundle_id.is_null() {
                    let len = strlen(os_bundle_id as _);
                    let slice = &*slice_from_raw_parts(os_bundle_id as *const u8, len);
                    str::from_utf8_unchecked(slice)
                } else {
                    "com.itsjustbox.turubai.unnamed_app"
                }
            }
        };

        let context = Context::new(turubai_app);
        App::new(bundle_id, context).run();
        exit(0)
    }
}

/// Holds a native view and any inner content that needs to stay alive
/// Cacao requires Rust wrappers to be retained for native views to persist
enum NativeView {
    /// A plain view container with children
    Container {
        view: View,
        _children: Vec<NativeView>,
    },
    /// A text label wrapped in a view
    Text {
        wrapper: View,
        _label: Label,
    },
}

impl NativeView {
    /// Get the view to add as subview
    fn view(&self) -> &View {
        match self {
            NativeView::Container { view, .. } => view,
            NativeView::Text { wrapper, .. } => wrapper,
        }
    }
}

/// Window delegate - manages the content of a single window
pub struct TurubaiWindowDelegate {
    content: View,
    /// Keep all native views alive - cacao needs these retained
    _root_views: Vec<NativeView>,
    /// Content size for window fitting
    content_width: f64,
    content_height: f64,
}

impl TurubaiWindowDelegate {
    fn new(root: &ShadowNode, _tree: &mut ShadowTree) -> Self {
        eprintln!("[DEBUG] TurubaiWindowDelegate::new called");
        eprintln!("[DEBUG] Root node kind: {:?}", root.kind);
        eprintln!("[DEBUG] Root has {} children", root.children.len());

        let content = View::new();

        // Track content size for window fitting
        let mut max_width: f64 = 0.0;
        let mut max_height: f64 = 0.0;
        // let mut total_height: f64 = 0.0;

        // Render the tree
        let mut root_views: Vec<NativeView> = Vec::new();

        for (i, child) in root.children.iter().enumerate() {
            eprintln!("[DEBUG] Rendering child {}: {:?}", i, child.kind);
            let (native_view, width, height) = Self::render_node(child);

            content.add_subview(native_view.view());

            // Track size
            if width > max_width {
                max_width = width;
            }
            if height > max_height {
                max_height = height;
            }
            // total_height += height;

            // Pin to edges with padding
            LayoutConstraint::activate(&[
                native_view.view().top.constraint_equal_to(&content.top),
                native_view.view().leading.constraint_equal_to(&content.leading),
                content.trailing.constraint_equal_to(&native_view.view().trailing),
                content.bottom.constraint_equal_to(&native_view.view().bottom),
            ]);

            root_views.push(native_view);
        }

        // Add padding to content size
        let content_width = max_width;
        let content_height = max_height;

        eprintln!("[DEBUG] Computed content size: {}x{}", content_width, content_height);

        Self {
            content,
            _root_views: root_views,
            content_width,
            content_height,
        }
    }

    /// Recursively render a shadow node to native views
    /// Returns (NativeView, estimated_width, estimated_height)
    fn render_node(node: &ShadowNode) -> (NativeView, f64, f64) {
        match &node.kind {
            NodeKind::Text { content, .. } => {
                eprintln!("[DEBUG] Creating Text label with content: '{}'", content);

                let label = Label::new();
                label.set_text(content);
                label.set_text_color(Color::Label);

                // Get actual intrinsic size from the label
                let (width, height) = get_label_intrinsic_size(&label);
                eprintln!("[DEBUG] Label intrinsic size: {}x{}", width, height);

                let wrapper = View::new();
                wrapper.add_subview(&label);

                LayoutConstraint::activate(&[
                    label.top.constraint_equal_to(&wrapper.top),
                    label.leading.constraint_equal_to(&wrapper.leading),
                    label.trailing.constraint_equal_to(&wrapper.trailing),
                    label.bottom.constraint_equal_to(&wrapper.bottom),
                ]);

                let native = NativeView::Text { wrapper, _label: label };
                (native, width, height)
            }

            NodeKind::HStack { spacing} => {
                let stack = View::new();
                let mut children: Vec<NativeView> = Vec::new();

                let mut total_width: f64 = 0.0;
                let mut max_height: f64 = 0.0;

                // First pass: render all children
                let rendered: Vec<(NativeView, f64, f64)> = node.children.iter()
                    .map(|c| Self::render_node(c))
                    .collect();

                let count = rendered.len();

                // Second pass: add to stack and set up constraints
                for (i, (child, w, h)) in rendered.iter().enumerate() {
                    stack.add_subview(child.view());

                    total_width += w;
                    if i > 0 { total_width += 8.0; }
                    if *h > max_height { max_height = *h; }

                    LayoutConstraint::activate(&[
                        child.view().top.constraint_equal_to(&stack.top),
                        child.view().bottom.constraint_equal_to(&stack.bottom),
                    ]);

                    if i == 0 {
                        LayoutConstraint::activate(&[
                            child.view().leading.constraint_equal_to(&stack.leading),
                        ]);
                    } else {
                        // Chain to the prev  sibling
                        let prev = &rendered[i - 1].0;
                        LayoutConstraint::activate(&[
                            child.view().leading.constraint_equal_to(&prev.view().trailing).offset(*spacing),
                        ]);
                    }

                    if i == count - 1 {
                        LayoutConstraint::activate(&[
                            child.view().trailing.constraint_equal_to(&stack.trailing),
                        ]);
                    }
                }

                // Move children into the container
                for (child, _, _) in rendered {
                    children.push(child);
                }

                let native = NativeView::Container { view: stack, _children: children };
                (native, total_width, max_height)
            }

            NodeKind::VStack { spacing } => {
                let stack = View::new();
                let mut children: Vec<NativeView> = Vec::new();

                let mut max_width: f64 = 0.0;
                let mut total_height: f64 = 0.0;

                // Render all children first
                let rendered: Vec<(NativeView, f64, f64)> = node.children.iter()
                    .map(|c| Self::render_node(c))
                    .collect();

                let count = rendered.len();

                // Add all views first (needed for chaining constraints)
                for (child, _, _) in &rendered {
                    stack.add_subview(child.view());
                }

                // Set up constraints with proper vertical chaining
                for (i, (child, w, h)) in rendered.iter().enumerate() {
                    if *w > max_width { max_width = *w; }
                    total_height += h;
                    if i > 0 { total_height += 8.0; }

                    LayoutConstraint::activate(&[
                        child.view().leading.constraint_equal_to(&stack.leading),
                        child.view().trailing.constraint_less_than_or_equal_to(&stack.trailing),
                    ]);

                    if i == 0 {
                        LayoutConstraint::activate(&[
                            child.view().top.constraint_equal_to(&stack.top),
                        ]);
                    } else {
                        // Chain to previous sibling
                        let prev = &rendered[i - 1].0;
                        LayoutConstraint::activate(&[
                            child.view().top.constraint_equal_to(&prev.view().bottom).offset(*spacing),
                        ]);
                    }

                    if i == count - 1 {
                        LayoutConstraint::activate(&[
                            child.view().bottom.constraint_equal_to(&stack.bottom),
                        ]);
                    }
                }

                // Move children into the container
                for (child, _, _) in rendered {
                    children.push(child);
                }

                let native = NativeView::Container { view: stack, _children: children };
                (native, max_width, total_height)
            }

            NodeKind::View | NodeKind::Window { .. } => {
                let view = View::new();
                let mut children: Vec<NativeView> = Vec::new();

                let mut max_width: f64 = 0.0;
                let mut max_height: f64 = 0.0;

                for shadow_child in &node.children {
                    let (child, w, h) = Self::render_node(shadow_child);
                    view.add_subview(child.view());

                    if w > max_width { max_width = w; }
                    if h > max_height { max_height = h; }

                    LayoutConstraint::activate(&[
                        child.view().top.constraint_equal_to(&view.top),
                        child.view().leading.constraint_equal_to(&view.leading),
                        child.view().trailing.constraint_equal_to(&view.trailing),
                        child.view().bottom.constraint_equal_to(&view.bottom),
                    ]);

                    children.push(child);
                }

                let native = NativeView::Container { view, _children: children };
                (native, max_width, max_height)
            }
        }
    }
}

impl WindowDelegate for TurubaiWindowDelegate {
    const NAME: &'static str = "TurubaiWindowDelegate";

    fn did_load(&mut self, window: Window) {
        println!("[DEBUG] Window did load!");
        window.set_content_view(&self.content);
        window.set_content_size(self.content_width, self.content_height);
    }
}
