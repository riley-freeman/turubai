use cacao::core_graphics::base::CGFloat;
use cacao::foundation::{id, NSString};
use cacao::objc::{class, msg_send, sel, sel_impl};
use objc_id::ShareId;

#[derive(Clone)]
pub struct NativeFont {
    font: cacao::text::Font,
    size: f64,
}

impl NativeFont {
    pub fn new(name: &str, size: f32) -> Self {
        let cg_size = CGFloat::from(size);

        let font = unsafe {
            let ns_name = NSString::new(name);
            let font_ptr: id = msg_send![class!(NSFont), fontWithName: &*ns_name size: cg_size];

            if font_ptr.is_null() {
                // Font not found - fall back to system font
                eprintln!("[DEBUG] Font '{}' not found, falling back to system font", name);
                cacao::text::Font::system(size as f64)
            } else {
                cacao::text::Font(ShareId::from_ptr(font_ptr))
            }
        };

        Self {
            font,
            size: cg_size.into(),
        }
    }

    pub fn os_font(&self) -> cacao::text::Font {
        self.font.clone()
    }

    pub fn size(&self) -> f64 {
        self.size
    }
}


