use gtk4::{
    gio::prelude::{ApplicationExt, ApplicationExtManual},
    prelude::*,
    Fixed, Orientation, Window,
};

use crate::{
    color::Color,
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

static DEFAULT_WINDOW_WIDTH: i32 = 320;
static DEFAULT_WINDOW_HEIGHT: i32 = 200;

pub struct Context {}

impl Context {
    fn create_pango_attr_list(
        font: &crate::font::Font,
        color: &Color,
        decoration: &crate::elements::TextDecoration,
    ) -> (gtk4::pango::AttrList, gtk4::pango::FontDescription) {
        // Font
        let mut font_desc = gtk4::pango::FontDescription::new();
        font_desc.set_family(&font.name());
        let size = font.size() * 1024.0; // Pango units are 1/1024
        font_desc.set_size(size as i32);

        font_desc.set_weight(conv::conv_weight(font.weight()));

        if font.is_italic() {
            font_desc.set_style(gtk4::pango::Style::Italic);
        }

        // Attributes
        let attrs = gtk4::pango::AttrList::new();

        // Apply Font Description as Attribute
        let mut attr_font = gtk4::pango::AttrFontDesc::new(&font_desc);
        attr_font.set_start_index(0);
        attr_font.set_end_index(gtk4::pango::ATTR_INDEX_TO_TEXT_END);
        attrs.insert(attr_font);

        // Color
        if let Color::Custom { r, g, b, a } = color {
            let red = (r * 65535.0) as u16;
            let green = (g * 65535.0) as u16;
            let blue = (b * 65535.0) as u16;
            let alpha = (a * 65535.0) as u16;

            let mut attr_fg = gtk4::pango::AttrColor::new_foreground(red, green, blue);
            attr_fg.set_start_index(0);
            attr_fg.set_end_index(gtk4::pango::ATTR_INDEX_TO_TEXT_END);
            attrs.insert(attr_fg);

            let mut attr_alpha = gtk4::pango::AttrInt::new_foreground_alpha(alpha);
            attr_alpha.set_start_index(0);
            attr_alpha.set_end_index(gtk4::pango::ATTR_INDEX_TO_TEXT_END);
            attrs.insert(attr_alpha);
        }

        // Decoration - Underline
        let underline_style = conv::conv_underline_style(&decoration.underline.style);

        if underline_style != gtk4::pango::Underline::None {
            let mut attr_u = gtk4::pango::AttrInt::new_underline(underline_style);
            attr_u.set_start_index(0);
            attr_u.set_end_index(gtk4::pango::ATTR_INDEX_TO_TEXT_END);
            attrs.insert(attr_u);

            if let Color::Custom { r, g, b, a: _ } = decoration.underline.color {
                let red = (r * 65535.0) as u16;
                let green = (g * 65535.0) as u16;
                let blue = (b * 65535.0) as u16;
                let mut attr_uc = gtk4::pango::AttrColor::new_underline_color(red, green, blue);
                attr_uc.set_start_index(0);
                attr_uc.set_end_index(gtk4::pango::ATTR_INDEX_TO_TEXT_END);
                attrs.insert(attr_uc);
            }
        }

        // Decoration - Strikethrough
        if decoration.strike_through.style.clone() != crate::elements::TextLineStyle::None {
            let mut attr_s = gtk4::pango::AttrInt::new_strikethrough(true);
            attr_s.set_start_index(0);
            attr_s.set_end_index(gtk4::pango::ATTR_INDEX_TO_TEXT_END);
            attrs.insert(attr_s);

            if let Color::Custom { r, g, b, a: _ } = decoration.strike_through.color {
                let red = (r * 65535.0) as u16;
                let green = (g * 65535.0) as u16;
                let blue = (b * 65535.0) as u16;
                let mut attr_sc = gtk4::pango::AttrColor::new_strikethrough_color(red, green, blue);
                attr_sc.set_start_index(0);
                attr_sc.set_end_index(gtk4::pango::ATTR_INDEX_TO_TEXT_END);
                attrs.insert(attr_sc);
            }
        }

        (attrs, font_desc)
    }

    pub fn request_dimensions(&self, node: &ShadowNode) -> (i32, i32) {
        match &node.kind {
            NodeKind::Text {
                content,
                font,
                color,
                decoration,
            } => {
                let label = gtk4::Label::new(Some(content.as_str()));
                let (attrs, _) = Self::create_pango_attr_list(font, color, decoration);
                label.set_attributes(Some(&attrs));

                let (_, natural_width, _, _) = label.measure(gtk4::Orientation::Horizontal, -1);
                let (_, natural_height, _, _) =
                    label.measure(gtk4::Orientation::Vertical, natural_width);

                (natural_width, natural_height)
            }
            _ => (0, 0),
        }
    }

    pub fn update_layout(&self, node: &ShadowNode, tree: &ShadowTree) -> (bool, bool) {
        match &node.kind {
            NodeKind::Text { .. } => {
                let (w, h) = self.request_dimensions(node);
                tree.set_size(
                    node.taffy_id,
                    taffy::Dimension::length(w as f32),
                    taffy::Dimension::length(h as f32),
                );
                (false, false)
            }
            NodeKind::Spacer => {
                // Spacers are handled by Taffy (flex: 1)
                (true, true)
            }

            _ => {
                let mut max_full_width = false;
                let mut max_full_height = false;
                for child in &node.children {
                    let (full_width, full_height) = self.update_layout(child, tree);
                    max_full_width = max_full_width || full_width;
                    max_full_height = max_full_height || full_height;
                }
                (max_full_width, max_full_height)
            }
        }
    }

    pub fn render_node(&self, node: &ShadowNode, tree: &ShadowTree) -> gtk4::Widget {
        match &node.kind {
            NodeKind::Text {
                content,
                font,
                color,
                decoration,
            } => {
                let label = gtk4::Label::new(Some(content.as_str()));
                let (attrs, _) = Self::create_pango_attr_list(font, color, decoration);
                label.set_attributes(Some(&attrs));

                if let Some(layout) = tree.get_layout(node.taffy_id) {
                    label.set_size_request(layout.size.width as i32, layout.size.height as i32);
                }

                label.into()
            }
            NodeKind::VStack { .. } | NodeKind::HStack { .. } => {
                let container = gtk4::Fixed::new();

                for child in &node.children {
                    let child_widget = self.render_node(child, tree);

                    let (x, y) = if let Some(layout) = tree.get_layout(child.taffy_id) {
                        (layout.location.x as f64, layout.location.y as f64)
                    } else {
                        (0.0, 0.0)
                    };

                    container.put(&child_widget, x, y);
                }

                if let Some(layout) = tree.get_layout(node.taffy_id) {
                    container.set_size_request(layout.size.width as i32, layout.size.height as i32);
                }

                container.into()
            }
            _ => gtk4::Label::new(Some("Unsupported Node")).into(),
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

        // Use Rc<RefCell> to allow mutation in the callback
        let shadow_tree = std::rc::Rc::new(std::cell::RefCell::new(ShadowTree::new()));

        let mut window_node = shadow_tree
            .borrow()
            .create_node_from_element(window_element.as_ref());
        let root_node = window_node.children.pop().unwrap();

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
                context.update_layout(&root_node, &shadow_tree.borrow());

                // 2. Compute Layout
                shadow_tree.borrow_mut().compute_layout(
                    &root_node,
                    DEFAULT_WINDOW_WIDTH as f32,
                    DEFAULT_WINDOW_HEIGHT as f32,
                );

                // 3. Render
                let root_widget = context.render_node(&root_node, &shadow_tree.borrow());
                window.set_child(Some(&root_widget));
                window.show();
            }
        });

        std::process::exit(gtk_app.run().into())
    }
}
