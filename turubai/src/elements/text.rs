use std::sync::Arc;
use std::sync::Mutex;

use crate::color::Color;
use crate::elements::{Element, Modifiers};
use crate::font::Font;
use crate::font::FontWeight;
use crate::shadow::ShadowDescriptor;

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

    pub fn turubai_new_with_1_args(
        contents: &str,
        modifiers: Modifiers,
        children: impl FnOnce(Modifiers) -> Vec<Box<dyn Element>>,
    ) -> Self {
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
            inner: Arc::new(Mutex::new(value)),
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
        let text_mods = &mods_inner.text;

        ShadowDescriptor::text(
            inner.contents.clone(),
            text_mods.font.clone(),
            text_mods.color.clone(),
            text_mods.decoration.clone(),
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

#[derive(Clone, PartialEq)]
pub struct TextModifiers {
    pub font: Font,
    pub color: Color,
    pub decoration: TextDecoration,
}

impl Default for TextModifiers {
    fn default() -> Self {
        Self {
            color: Color::Text,
            font: Font::default(),
            decoration: TextDecoration::default(),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct TextDecoration {
    pub underline: TextDecorationLine,
    pub strike_through: TextDecorationLine,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct TextDecorationLine {
    pub style: TextLineStyle,
    pub color: Color,
}

impl Default for TextDecorationLine {
    fn default() -> Self {
        Self {
            style: TextLineStyle::None,
            color: Color::Text,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum TextLineStyle {
    #[default]
    None,
    Single,
    Thick,
    Double,
    Dotted,
    Dashed,
}
