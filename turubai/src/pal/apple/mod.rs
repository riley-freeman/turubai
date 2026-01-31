use std::process::exit;
use std::ptr::slice_from_raw_parts;

use cacao::appkit::{App, AppDelegate};
use cacao::appkit::window::{Window, WindowConfig, WindowDelegate, WindowStyle};
use cacao::color::Color;
use cacao::core_foundation::bundle::{CFBundleGetIdentifier, CFBundleGetMainBundle};
use cacao::core_graphics::display::{CGPoint, CGRect, CGSize};
use cacao::geometry::Rect;
use cacao::layout::{Layout, LayoutConstraint};
use cacao::text::Label;
use cacao::view::View;

use libc::strlen;
use taffy::{AvailableSpace, Dimension, Point, Size};

mod measure;

use crate::pal::apple::measure::{get_estimated_size, update_node_sizes};
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
        let mut shadow_tree = ShadowTree::new();

        // Get the root element for the window
        let window_element = app.markup();
        let window_shadow = shadow_tree.create_node_from_element(window_element.as_ref());

        let root_shadow = window_shadow.children.get(0).unwrap();
        let (width, height) = get_estimated_size(root_shadow);

        // Update the sizes and position our nodes with taffy
        update_node_sizes(&root_shadow, &shadow_tree);
        shadow_tree.compute_layout(root_shadow, width as _, height as _);

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
    _root_view: NativeView,
    /// Content size for window fitting
    content_width: f64,
    content_height: f64,
}

impl TurubaiWindowDelegate {
    fn new(root: &ShadowNode, tree: &mut ShadowTree) -> Self {
        eprintln!("[DEBUG] TurubaiWindowDelegate::new called");
        eprintln!("[DEBUG] Root node kind: {:?}", root.kind);
        eprintln!("[DEBUG] Root has {} children", root.children.len());

        let content = View::new();

        // Calcualate the positions and sizes of the elements
        let (available_width, available_height) = get_estimated_size(root);
        eprintln!("[DEBUG] Computed content size: {}x{}", available_width, available_height);

        // Render the root node 
        let root_view = Self::render_node(root, tree);
        content.add_subview(root_view.view());

        // let mut root_views: Vec<NativeView> = Vec::new();
        // for (i, child) in root.children.iter().enumerate() {
        //     eprintln!("[DEBUG] Rendering child {}: {:?}", i, child.kind);

        //     let native_view= Self::render_node(child, tree);

        //     content.add_subview(native_view.view());

        //     // Pin to edges with padding
        //     LayoutConstraint::activate(&[
        //         native_view.view().top.constraint_equal_to(&content.top),
        //         native_view.view().leading.constraint_equal_to(&content.leading),
        //         content.trailing.constraint_equal_to(&native_view.view().trailing),
        //         content.bottom.constraint_equal_to(&native_view.view().bottom),
        //     ]);

        //     root_views.push(native_view);
        // }

        // Add padding to content size
        let content_width = available_width;
        let content_height = available_height;

        Self {
            content,
            _root_view: root_view,
            content_width,
            content_height,
        }
    }

    /// Recursively render a shadow node to native views
    /// Returns (NativeView, estimated_width, estimated_height)
    fn render_node(node: &ShadowNode, tree: &ShadowTree) -> NativeView {
        let layout = tree.get_layout(node.taffy_id).unwrap();
        let x = layout.location.x as f64;
        let y = layout.location.y as f64;
        let width = layout.size.width as f64;
        let height = layout.size.height as f64;

        let frame = CGRect::new(&CGPoint { x, y }, &CGSize { width, height });
        let view = match &node.kind {
            NodeKind::Text { content, font_size, font_weight } => {
                eprintln!("[DEBUG] rendering label: \"{}\"", content);

                let label = Label::new();
                label.set_text_color(Color::Label);
                label.set_text(content);

                let wrapper = View::new();
                wrapper.add_subview(&label);

                // Apply the coords from 
                LayoutConstraint::activate(&[
                    label.top.constraint_equal_to(&wrapper.top),
                    label.bottom.constraint_equal_to(&wrapper.bottom),
                    label.leading.constraint_equal_to(&wrapper.leading),
                    label.trailing.constraint_equal_to(&wrapper.trailing),
                ]);
                wrapper.set_translates_autoresizing_mask_into_constraints(true);


                NativeView::Text { wrapper, _label: label }
            }

            NodeKind::HStack { spacing, alignment } => {
                eprintln!("[DEBUG] rendering Horizontal Stack (Column)");

                let rendered: Vec<NativeView> = node.children
                    .iter().map(|c| Self::render_node(c, tree))
                    .collect();

                let view = View::new();
                view.set_translates_autoresizing_mask_into_constraints(false);
                for cv in &rendered {
                    view.add_subview(cv.view());
                }

                NativeView::Container { view, _children: rendered }
            }

            NodeKind::VStack { spacing, alignment } => {
                eprintln!("[DEBUG] rendering Vertical Stack (Row)");

                let rendered: Vec<NativeView> = node.children
                    .iter().map(|c| Self::render_node(c, tree))
                    .collect();

                let view = View::new();
                view.set_translates_autoresizing_mask_into_constraints(false);
                for cv in &rendered {
                    view.add_subview(cv.view());
                }

                NativeView::Container { view, _children: rendered }
            }



            _ => {
                unimplemented!()
            }

            // NodeKind::View | NodeKind::Window { .. } => {
            //     let view = View::new();
            //     let mut children: Vec<NativeView> = Vec::new();

            //     for shadow_child in &node.children {
            //         let child  = Self::render_node(shadow_child, tree);
            //         view.add_subview(child.view());

            //         LayoutConstraint::activate(&[
            //             child.view().top.constraint_equal_to(&view.top),
            //             child.view().leading.constraint_equal_to(&view.leading),
            //             child.view().trailing.constraint_equal_to(&view.trailing),
            //             child.view().bottom.constraint_equal_to(&view.bottom),
            //         ]);

            //         children.push(child);
            //     }

            //     let native = NativeView::Container { view, _children: children };
            //     native
            // }
        };

        view.view().set_frame(frame);
        view
    }
}

impl WindowDelegate for TurubaiWindowDelegate {
    const NAME: &'static str = "TurubaiWindowDelegate";

    fn did_load(&mut self, window: Window) {
        println!("[DEBUG] Window did load!");
        window.set_content_view(&self.content);
        window.set_content_size(self.content_width, self.content_height);
        window.set_minimum_content_size(self.content_width, self.content_height);
    }
}
