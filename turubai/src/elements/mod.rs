mod text;

pub use text::*;

use std::sync::{Arc, Mutex, MutexGuard, LockResult};
use crate::{runtime::WindowModifiers, shadow::ShadowDescriptor};

pub trait Element: Send + Sync {
    fn name(&self) -> &'static str;
    fn display_name(&self) -> &'static str;

    /// Returns the shadow descriptor for this element (used to build shadow tree)
    fn shadow_descriptor(&self) -> ShadowDescriptor;

    /// Build shadow tree recursively - returns descriptor with children already built
    fn build_shadow(&self, tree: &mut crate::shadow::ShadowTree) -> crate::shadow::ShadowNode
    where
        Self: Sized,
    {
        tree.create_node_from_element(self)
    }

    /// Number of children
    fn child_count(&self) -> usize { 0 }

    /// Visit each child with a callback
    fn for_each_child(&self, _f: &mut dyn FnMut(&dyn Element)) {}
}

#[derive(Default, Clone, PartialEq)]
pub struct ModifiersInner {
    pub text: TextModifiers,
    pub v_stack: StackModifiers,
    pub h_stack: StackModifiers,
    pub window_template: WindowModifiers,
}

#[derive(Default, Clone)]
pub struct Modifiers {
    inner: Arc<Mutex<ModifiersInner>>
}

impl Modifiers {
    pub fn fork(&self) -> Self {
        let inner = self.inner.lock().unwrap();
        Modifiers { inner: Arc::new(Mutex::new(inner.clone())) }
    }

    pub fn lock(&self) -> LockResult<MutexGuard<'_, ModifiersInner>> {
        self.inner.lock()
    }
}

#[derive(Default, Clone, Copy, PartialEq)]
pub struct StackModifiers {
    pub spacing: f32,
}