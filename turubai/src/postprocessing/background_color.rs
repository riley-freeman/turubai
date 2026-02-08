use std::sync::{Arc, Mutex};

use crate::{color::Color, elements::{Element, Modifiers}, shadow::ShadowDescriptor};

pub struct BackgroundColorInner {
    color: Color,
    child: Box<dyn Element>
}

pub struct BackgroundColor {
    inner: Arc<Mutex<BackgroundColorInner>>
}

impl BackgroundColor {
    pub fn new(color: &Color, child: Box<dyn Element>) -> Self {
        let inner = BackgroundColorInner {
            color: color.clone(),
            child: child
        };
        Self::from(inner)
    }

    pub fn new_1(
        color: Color,
        modifiers: Modifiers,
        children: impl FnOnce(Modifiers) -> Vec<Box<dyn Element>>,
    ) -> Self {
        let mut children = children(modifiers.clone());
        Self::new(&color, children.pop().unwrap())
    }
}

impl Element for BackgroundColor {
    fn name(&self) -> &'static str {
        "background_color"
    }

    fn display_name(&self) -> &'static str {
        "Background Color"
    }

    fn child_count(&self) -> usize {
        1
    }

    fn for_each_child(&self, f: &mut dyn FnMut(&dyn Element)) {
        f(self.inner.lock().unwrap().child.as_ref())
    }

    fn shadow_descriptor(&self) -> crate::shadow::ShadowDescriptor {
        ShadowDescriptor::background_color(self.inner.lock().unwrap().color.clone())
    }
}


impl From<BackgroundColorInner> for BackgroundColor {
    fn from(value: BackgroundColorInner) -> Self {
        Self {
            inner: Arc::new(Mutex::new(value)),
        }
    }
}


