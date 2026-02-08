use std::collections::HashMap;
use std::process::exit;
use std::ptr::slice_from_raw_parts;
use std::sync::{Arc, Mutex, Weak};

use cacao::appkit::window::{Window, WindowConfig, WindowDelegate, WindowStyle};
use cacao::appkit::{window, App, AppDelegate};
use cacao::core_foundation::bundle::{CFBundleGetIdentifier, CFBundleGetMainBundle};
use cacao::core_graphics::display::{CGPoint, CGRect, CGSize};
use cacao::layout::{Layout, LayoutConstraint};
use cacao::objc::{msg_send, sel, sel_impl};
use cacao::text::Label;
use cacao::view::View;

use libc::strlen;

mod color;
mod font;
mod measure;
mod stack;
mod text;

use crate::color::Color;
use crate::font::Font;
use crate::pal::apple::color::NativeColor;
use crate::pal::apple::font::NativeFont;
use crate::pal::apple::measure::{
    request_dimensions, request_minimum_dimensions, update_node_sizes,
};
use crate::pal::apple::stack::{render_h_stack, render_spacer, render_v_stack};
use crate::pal::apple::text::render_text;
use crate::pal::{apple, DynContext};
use crate::shadow::{NodeKind, ShadowNode, ShadowTree};
use crate::units::Percent;
use crate::{Application, Backend, Unit};

static DEFAULT_WINDOW_WIDTH: f64 = 480.0;
static DEFAULT_WINDOW_HEIGHT: f64 = 270.0;

pub struct API;

impl crate::pal::API for API {
    const VARIANT: Backend = Backend::Apple;
    type Context = Context;
}

/// Main application context - holds the app and manages windows
pub struct ContextInner {
    user_app: Box<dyn Application>,
    windows: Mutex<Vec<Window<TurubaiWindowDelegate>>>,
    native_colors: Mutex<HashMap<Color, Weak<NativeColor>>>,
    native_fonts: Mutex<HashMap<Font, Weak<NativeFont>>>,
}

#[derive(Clone)]
pub struct Context {
    inner: Arc<ContextInner>,
}

impl Context {
    fn new(app: Box<dyn Application>) -> Self {
        let inner = ContextInner {
            user_app: app,
            windows: Mutex::new(Vec::new()),
            native_colors: Mutex::new(HashMap::new()),
            native_fonts: Mutex::new(HashMap::new()),
        };
        let output = Self {
            inner: Arc::new(inner),
        };

        output
    }

    /// Recursively render a shadow node to native views
    /// Returns (NativeView, estimated_width, estimated_height)
    fn render_node(node: &ShadowNode, tree: &ShadowTree, context: Context) -> NativeView {
        let layout = tree.get_layout(node.taffy_id).unwrap();
        let x = layout.location.x as f64;
        let y = layout.location.y as f64;
        let width = layout.size.width as f64;
        let height = layout.size.height as f64;

        let frame = CGRect::new(&CGPoint { x, y }, &CGSize { width, height });
        let view = match &node.kind {
            NodeKind::Text {
                content,
                font,
                color,
                decoration,
            } => render_text(content, font, color, decoration, node, context.clone()),
            NodeKind::HStack { .. } => render_h_stack(node, tree, context.clone()),
            NodeKind::VStack { .. } => render_v_stack(node, tree, context.clone()),
            NodeKind::Spacer { .. } => render_spacer(),

            NodeKind::BackgroundColor { color } => {
                let child_node = node
                    .children
                    .first()
                    .expect("BackgroundColor must have a child");
                let child_view = Context::render_node(child_node, tree, context.clone());

                let view = View::new();
                let native_color = context.get_native_color(color);
                view.set_background_color(native_color.os_color());
                view.set_translates_autoresizing_mask_into_constraints(true);
                view.add_subview(child_view.view());

                NativeView::Container {
                    view,
                    _children: vec![child_view],
                }
            }

            _ => {
                unimplemented!()
            }
        };
        view.view().set_frame(frame);
        view
    }

    fn create_window(
        &self,
        window_node: ShadowNode,
        root: ShadowNode,
        tree: ShadowTree,
    ) -> Window<TurubaiWindowDelegate> {
        let title = match &window_node.kind {
            NodeKind::Window { title } => {
                title.clone().unwrap_or_else(|| "Turubai App".to_string())
            }
            _ => "Untitled Window".to_string(),
        };

        let mut config = WindowConfig::default();
        config.set_styles(&[
            WindowStyle::Titled,
            WindowStyle::Closable,
            WindowStyle::Miniaturizable,
            WindowStyle::Resizable,
        ]);

        let delegate = TurubaiWindowDelegate::new(root, tree, self.clone());
        let window = Window::with(config, delegate);
        window.set_title(&title);

        window
    }

    fn get_native_color(&self, color: &Color) -> Arc<NativeColor> {
        let mut colors = self.inner.native_colors.lock().unwrap();
        if let Some(color_weak) = colors.get(color) {
            if let Some(color) = color_weak.upgrade() {
                return color;
            }
        }
        let native_color = Arc::new(NativeColor::new(color.clone()));
        colors.insert(color.clone(), Arc::downgrade(&native_color));
        native_color
    }

    fn get_native_font(&self, font: &Font) -> Arc<NativeFont> {
        let mut fonts = self.inner.native_fonts.lock().unwrap();
        if let Some(font_weak) = fonts.get(font) {
            if let Some(font) = font_weak.upgrade() {
                return font;
            }
        }
        let native_font = Arc::new(NativeFont::new(
            &font.name(),
            font.size(),
            font.weight(),
            font.is_italic(),
        ));
        fonts.insert(font.clone(), Arc::downgrade(&native_font));
        native_font
    }
}

impl AppDelegate for Context {
    fn will_finish_launching(&self) {
        let window_element = self.inner.user_app.markup();

        let mut shadow_tree = ShadowTree::new();

        let mut window_node = shadow_tree.create_node_from_element(window_element.as_ref());

        let root_shadow = window_node.children.pop().unwrap();
        let (width_unit, height_unit) = request_dimensions(
            &root_shadow,
            self.clone(),
            DEFAULT_WINDOW_WIDTH,
            DEFAULT_WINDOW_HEIGHT,
        );
        let width = width_unit.to_pixels(Some(DEFAULT_WINDOW_WIDTH));
        let height = height_unit.to_pixels(Some(DEFAULT_WINDOW_HEIGHT));

        update_node_sizes(
            &root_shadow,
            &shadow_tree,
            self.clone(),
            DEFAULT_WINDOW_WIDTH,
            DEFAULT_WINDOW_HEIGHT,
        );
        shadow_tree.compute_layout(&root_shadow, width as f32, height as f32);

        let window = self.create_window(window_node, root_shadow, shadow_tree);

        self.inner.windows.lock().unwrap().push(window);
    }

    fn did_finish_launching(&self) {
        App::activate();
        let windows = self.inner.windows.lock().unwrap();
        for window in windows.iter() {
            window.show();
        }
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
                turubai_app.id()
            } else {
                let os_bundle_id = CFBundleGetIdentifier(bundle);
                if !os_bundle_id.is_null() {
                    let len = strlen(os_bundle_id as _);
                    let slice = &*slice_from_raw_parts(os_bundle_id as *const u8, len);
                    str::from_utf8_unchecked(slice)
                } else {
                    turubai_app.id()
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
    /// A view that separates.
    Spacer { view: View },
    /// A text label wrapped in a view
    Text {
        wrapper: View,
        _label: Label,
        _font: Arc<NativeFont>,
        _underline_color: Option<Arc<NativeColor>>,
        _strike_through_color: Option<Arc<NativeColor>>,
    },
}

impl NativeView {
    /// Get the view to add as subview
    fn view(&self) -> &View {
        match self {
            NativeView::Container { view, .. } => view,
            NativeView::Spacer { view } => view,
            NativeView::Text { wrapper, .. } => wrapper,
        }
    }

    /// Recursively update frames from the shadow tree layout
    /// If `skip_self` is true, only update children (used when parent is positioned by constraints)
    fn update_frames(&self, node: &ShadowNode, tree: &ShadowTree, skip_self: bool) {
        if !skip_self {
            let layout = tree.get_layout(node.taffy_id).unwrap();
            let x = layout.location.x as f64;
            let y = layout.location.y as f64;
            let width = layout.size.width as f64;
            let height = layout.size.height as f64;

            let frame = CGRect::new(&CGPoint { x, y }, &CGSize { width, height });
            self.view().set_frame(frame);
        }

        // Recursively update children
        if let NativeView::Container { _children, .. } = self {
            for (child_view, child_node) in _children.iter().zip(node.children.iter()) {
                child_view.update_frames(child_node, tree, false);
            }
        }
    }
}

/// Window delegate - manages the content of a single window
pub struct TurubaiWindowDelegate {
    context: Context,

    content: View,
    window: Option<Window>,

    shadow_tree: Mutex<ShadowTree>,
    root_node: ShadowNode,

    _root_view: NativeView,
    content_width: f64,
    content_height: f64,

    // Active layout constraints (stored so they can be updated on resize)
    active_constraints: Mutex<Vec<LayoutConstraint>>,
}

impl TurubaiWindowDelegate {
    /// Build layout constraints for the root view based on content dimensions
    fn build_constraints(
        root_view: &NativeView,
        content: &View,
        width_unit: &dyn Unit,
        height_unit: &dyn Unit,
        available_width: f64,
        available_height: f64,
    ) -> Vec<LayoutConstraint> {
        let mut constraints = Vec::new();

        let width_is_percent = width_unit.downcast_ref::<Percent>().is_some();
        let height_is_percent = height_unit.downcast_ref::<Percent>().is_some();

        // X axis: if width is percentage, pin left/right; otherwise center horizontally
        if width_is_percent {
            constraints.push(root_view.view().left.constraint_equal_to(&content.left));
            constraints.push(root_view.view().right.constraint_equal_to(&content.right));
        } else {
            constraints.push(
                root_view
                    .view()
                    .center_x
                    .constraint_equal_to(&content.center_x),
            );
            constraints.push(
                root_view
                    .view()
                    .width
                    .constraint_equal_to_constant(available_width),
            );
        }

        // Y axis: if height is percentage, pin top/bottom; otherwise center vertically
        if height_is_percent {
            constraints.push(root_view.view().top.constraint_equal_to(&content.top));
            constraints.push(root_view.view().bottom.constraint_equal_to(&content.bottom));
        } else {
            constraints.push(
                root_view
                    .view()
                    .center_y
                    .constraint_equal_to(&content.center_y),
            );
            constraints.push(
                root_view
                    .view()
                    .height
                    .constraint_equal_to_constant(available_height),
            );
        }

        constraints
    }

    fn new(root: ShadowNode, tree: ShadowTree, context: Context) -> Self {
        eprintln!("[DEBUG] TurubaiWindowDelegate::new called");
        eprintln!("[DEBUG] Root node kind: {:?}", root.kind);
        eprintln!("[DEBUG] Root has {} children", root.children.len());

        let content = View::new();

        // Calculate the positions and sizes of the elements
        let (width_unit, height_unit) = request_dimensions(
            &root,
            context.clone(),
            DEFAULT_WINDOW_WIDTH,
            DEFAULT_WINDOW_HEIGHT,
        );
        let available_width = width_unit.to_pixels(Some(DEFAULT_WINDOW_WIDTH));
        let available_height = height_unit.to_pixels(Some(DEFAULT_WINDOW_HEIGHT));

        // Calculate minimum content size (spacers treated as 0)
        let (min_width, min_height) =
            request_minimum_dimensions(&root, context.clone(), DEFAULT_WINDOW_WIDTH);
        eprintln!(
            "[DEBUG] Computed content size: {}x{}, minimum: {}x{}",
            available_width, available_height, min_width, min_height
        );

        // Render the root node
        let root_view = Context::render_node(&root, &tree, context.clone());
        content.add_subview(root_view.view());

        // Build and activate constraints
        let constraints = Self::build_constraints(
            &root_view,
            &content,
            width_unit.as_ref(),
            height_unit.as_ref(),
            available_width,
            available_height,
        );
        LayoutConstraint::activate(&constraints);

        // Must disable autoresizing mask translation to use Auto Layout constraints
        root_view
            .view()
            .set_translates_autoresizing_mask_into_constraints(false);

        Self {
            context,
            content,
            window: None,

            shadow_tree: Mutex::new(tree),
            root_node: root,

            _root_view: root_view,
            content_width: min_width,
            content_height: min_height,

            active_constraints: Mutex::new(constraints),
        }
    }

    /// Update constraints based on new dimensions
    fn update_constraints(&self, width: f64, height: f64) {
        // Recalculate dimensions
        let (width_unit, height_unit) =
            request_dimensions(&self.root_node, self.context.clone(), width, height);
        let available_width = width_unit.to_pixels(Some(width));
        let available_height = height_unit.to_pixels(Some(height));

        // Deactivate old constraints
        {
            let old_constraints = self.active_constraints.lock().unwrap();
            LayoutConstraint::deactivate(&old_constraints);
        }

        // Build and activate new constraints
        let new_constraints = Self::build_constraints(
            &self._root_view,
            &self.content,
            width_unit.as_ref(),
            height_unit.as_ref(),
            available_width,
            available_height,
        );
        LayoutConstraint::activate(&new_constraints);

        // Store new constraints
        *self.active_constraints.lock().unwrap() = new_constraints;
    }
}

impl WindowDelegate for TurubaiWindowDelegate {
    const NAME: &'static str = "TurubaiWindowDelegate";

    fn did_load(&mut self, window: Window) {
        println!("[DEBUG] Window did load!");
        window.set_content_view(&self.content);
        window.set_content_size(DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT);
        window.set_minimum_content_size(self.content_width, self.content_height);

        self.window = Some(window);
    }

    fn did_resize(&self) {
        let mut shadow_tree = self.shadow_tree.lock().unwrap();

        // Get actual content view bounds (excludes title bar)
        let content_frame: CGRect = self
            .content
            .objc
            .get(|view| unsafe { msg_send![view, bounds] });
        let layout_width = content_frame.size.width;
        let layout_height = content_frame.size.height;

        // Update Auto Layout constraints for new dimensions
        self.update_constraints(layout_width, layout_height);

        // Recompute taffy layout with content view dimensions
        update_node_sizes(
            &self.root_node,
            &shadow_tree,
            self.context.clone(),
            layout_width,
            layout_height,
        );
        shadow_tree.compute_layout(&self.root_node, layout_width as f32, layout_height as f32);

        // Update all view frames from the new layout (including root)
        self._root_view
            .update_frames(&self.root_node, &shadow_tree, false);
    }
}
