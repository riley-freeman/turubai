use std::sync::Arc;
use std::sync::Mutex;

use crate::elements::{Element, Modifiers};
use crate::shadow::{ShadowDescriptor, FontWeight as ShadowFontWeight};

pub struct Text {
    inner: Arc<Mutex<TextInner>>,
}

struct TextInner {
    contents: String,
    modifiers: Modifiers,
    children: Vec<Box<dyn Element>>,
}

impl Text {
    pub fn new(contents: &str, modifiers: Modifiers) -> Self {
        let inner = TextInner {
            contents: String::from(contents),
            modifiers,
            children: vec![],
        };
        Self::from(inner)
    }

    pub fn new_1(contents: &str, modifiers: Modifiers, children: fn(Modifiers) -> Vec<Box<dyn Element>>) -> Self {
        let inner = TextInner {
            contents: String::from(contents),
            modifiers: modifiers.clone(),
            children: children(modifiers.fork()),
        };
        Self::from(inner)
    }
}

impl From<TextInner> for Text {
    fn from(value: TextInner) -> Self {
        Self {
            inner: Arc::new(Mutex::new(value))
        }
    }
}

impl Element for Text {
    fn name(&self) -> &'static str {
        "text"
    }

    fn display_name(&self) -> &'static str {
        "Text"
    }

    fn shadow_descriptor(&self) -> ShadowDescriptor {
        let inner = self.inner.lock().unwrap();
        let mods_inner = inner.modifiers.lock().unwrap();

        let font_weight = match mods_inner.text.weight {
            FontWeight::Thin => ShadowFontWeight::Thin,
            FontWeight::Light => ShadowFontWeight::Light,
            FontWeight::Regular => ShadowFontWeight::Regular,
            FontWeight::Medium => ShadowFontWeight::Medium,
            FontWeight::Semibold => ShadowFontWeight::Semibold,
            FontWeight::Bold => ShadowFontWeight::Bold,
            FontWeight::Heavy => ShadowFontWeight::Heavy,
            FontWeight::Black => ShadowFontWeight::Black,
        };

        ShadowDescriptor::text(
            inner.contents.clone(),
            mods_inner.text.size,
            font_weight,
        )
    }

    fn child_count(&self) -> usize {
        self.inner.lock().unwrap().children.len()
    }

    fn for_each_child(&self, f: &mut dyn FnMut(&dyn Element)) {
        let inner = self.inner.lock().unwrap();
        for child in &inner.children {
            f(child.as_ref());
        }
    }
}

#[derive(Default, PartialEq, Eq, Clone, Copy)]
pub enum TextAlign {
    #[default]
    Leading,
    Center,
    Trailing,
}

#[derive(Default, PartialEq, Eq, Clone, Copy)]
pub enum FontWeight {
    Thin,
    Light,
    #[default]
    Regular,
    Medium,
    Semibold,
    Bold,
    Heavy,
    Black,
}

#[derive(Default, Clone, Copy, PartialEq)]
pub struct TextModifiers {
    pub size: f32,
    pub align: TextAlign,
    pub weight: FontWeight,
}
