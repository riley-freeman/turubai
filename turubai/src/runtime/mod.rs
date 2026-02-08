use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::Mutex;

use crate::elements::{Element, Modifiers};
use crate::shadow::ShadowDescriptor;
use crate::Application;

pub struct WindowTemplate {
    inner: Arc<Mutex<WindowTemplateInner>>,
}

struct WindowTemplateInner {
    id: String,
    title: String,
    modifiers: Modifiers,
    child: Option<Box<dyn Element>>,
}

impl WindowTemplate {
    pub fn new_0(
        modifiers: Modifiers,
        children: impl FnOnce(Modifiers) -> Vec<Box<dyn Element>>,
    ) -> Self {
        Self::new_1_impl("default", modifiers, children)
    }

    pub fn new_1(
        id: &str,
        modifiers: Modifiers,
        children: impl FnOnce(Modifiers) -> Vec<Box<dyn Element>>,
    ) -> Self {
        Self::new_1_impl(id, modifiers, children)
    }

    fn new_1_impl(
        id: &str,
        modifiers: Modifiers,
        children: impl FnOnce(Modifiers) -> Vec<Box<dyn Element>>,
    ) -> Self {
        let mods = modifiers.lock().unwrap();
        let title = Some(mods.window_template.title.to_string());
        std::mem::drop(mods);

        let children = children(modifiers.clone());
        let mut children_deque = VecDeque::from(children);

        let child = children_deque.pop_front();
        let inner = WindowTemplateInner {
            id: id.to_string(),
            title,
            modifiers,
            child,
        };

        Self {
            inner: Arc::new(Mutex::new(inner)),
        }
    }

    pub fn identifier(&self) -> usize {
        self.inner.as_ref() as *const _ as usize
    }

    pub fn title(&self) -> String {
        self.inner.lock().unwrap().title.clone()
    }

    pub fn set_title(&self, title: impl Into<String>) {
        self.inner.lock().unwrap().title = title.into();
    }
}

impl Element for WindowTemplate {
    fn name(&self) -> &'static str {
        "window"
    }

    fn display_name(&self) -> &'static str {
        "Window"
    }

    fn shadow_descriptor(&self) -> ShadowDescriptor {
        let inner = self.inner.lock().unwrap();
        ShadowDescriptor::window(inner.title.clone())
    }

    fn child_count(&self) -> usize {
        self.inner.lock().unwrap().child.is_some() as usize
    }

    fn for_each_child(&self, f: &mut dyn FnMut(&dyn Element)) {
        let inner = self.inner.lock().unwrap();
        if let Some(child) = &inner.child {
            f(child.as_ref());
        }
    }
}

#[derive(Default, Clone, PartialEq)]
pub struct WindowModifiers {
    pub title: &'static str,
}

pub fn turubai_main(app: Box<dyn Application>) -> ! {
    crate::pal::takeover(app)
}
