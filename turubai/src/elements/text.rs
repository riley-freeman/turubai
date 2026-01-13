use std::sync::Arc;
use std::sync::Mutex;
use turubai_types::Modifiers;

use crate::elements::Element;

pub struct Text {
    inner: Arc<Mutex<TextInner>>,
}
struct TextInner {
    contents: String,
    modifiers: Modifiers,
    children: Vec<Box<dyn Element>>,
}





impl Text {
    pub fn new(contents: &str, modifiers: Modifiers, children: fn(Modifiers) -> Vec<Box<dyn Element>>) -> Self {
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
}