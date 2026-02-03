use cacao::core_foundation::base::{CFType, CFTypeRef, TCFType};
use cacao::core_foundation::boolean::CFBoolean;
use cacao::core_foundation::number::CFNumber;
use cacao::core_foundation::string::CFString;
use cacao::core_graphics::base::CGFloat;
use cacao::core_graphics::display::CFDictionary;
use cacao::foundation::{id, NSString};
use cacao::objc::runtime::Object;
use cacao::objc::{class, msg_send, sel, sel_impl};
use objc_id::ShareId;
use plist::Dictionary;

use crate::font::FontWeight;

#[derive(Clone)]
pub struct NativeFont {
    font: cacao::text::Font,
    size: f64,
}

impl NativeFont {
    pub fn new(family: &str, size: f32, weight: FontWeight, italic: bool) -> Self {
        let cf_family = CFString::new(family);
        let cg_size = CGFloat::from(size);
        let cf_size = CFNumber::from(size);

        let cf_name = if italic {
            CFString::new(&format!("{}-Italic", family))
        } else {
            CFString::new(family)
        };

        let font = unsafe {
            let wght_key = CFNumber::from(0x77676874);
            let wght_value = CFNumber::from(weight as u32 as f64);

            let ital_key = CFNumber::from(0x6974616C);
            let ital_value = CFBoolean::from(true);

            let variation_dict = CFDictionary::from_CFType_pairs(&[
                (wght_key.as_CFType(), wght_value.as_CFType()),
                (ital_key.as_CFType(), ital_value.as_CFType()),
            ]);
            
            let family_attr_key = CFString::new("NSFontFamilyAttribute");
            let name_attr_key = CFString::new("NSFontNameAttribute");
            let size_attr_key = CFString::new("NSFontSizeAttribute");
            let variation_attr_key = CFString::new("NSCTFontVariationAttribute");
            let attributes = CFDictionary::from_CFType_pairs(&[
                (family_attr_key.as_CFType(), cf_family.as_CFType()),
                (name_attr_key.as_CFType(), cf_name.as_CFType()),
                (size_attr_key.as_CFType(), cf_size.as_CFType()),
                (variation_attr_key.as_CFType(), variation_dict.as_CFType()),
            ]);
            let desc: *mut Object = msg_send![class!(NSFontDescriptor), fontDescriptorWithFontAttributes: attributes];
            let font_ptr: id = msg_send![class!(NSFont), fontWithDescriptor: desc size: cg_size];

            if font_ptr.is_null() {
                // Font not found - fall back to system font
                eprintln!("[DEBUG] Font family '{}' not found, falling back to system font", family);
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


