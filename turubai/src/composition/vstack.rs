use std::sync::Arc;
use std::sync::Mutex;

use crate::elements::{Element, Modifiers};
use crate::shadow::ShadowDescriptor;

pub struct VStack {
    inner: Arc<Mutex<VStackInner>>,
}

struct VStackInner {
    modifiers: Modifiers,
    children: Vec<Box<dyn Element>>,
}

impl VStack {
    pub fn new(modifiers: Modifiers, children: Vec<Box<dyn Element>>) -> Self {
        let inner = VStackInner {
            modifiers,
            children,
        };
        Self {
            inner: Arc::new(Mutex::new(inner)),
        }
    }

    pub fn new_0(modifiers: Modifiers, children: impl FnOnce(Modifiers) -> Vec<Box<dyn Element>>) -> Self {
        let child_elements = children(modifiers.fork());
        Self::new(modifiers, child_elements)
    }
}

impl Element for VStack {
    fn name(&self) -> &'static str {
        "v_stack"
    }

    fn display_name(&self) -> &'static str {
        "VStack"
    }

    fn shadow_descriptor(&self) -> ShadowDescriptor {
        let inner = self.inner.lock().unwrap();
        let mods = inner.modifiers.lock().unwrap();
        ShadowDescriptor::vstack(mods.v_stack.spacing, mods.v_stack.alignment)
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
