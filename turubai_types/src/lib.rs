use std::sync::{Arc, LockResult, Mutex, MutexGuard};

use crate::{composition::StackModifiers, text::TextModifiers};

pub mod text;
pub mod font;
pub mod composition;

#[derive(Default, Clone, Copy, PartialEq)]
pub struct ModifiersInner {
    pub text: TextModifiers,
    pub stack: StackModifiers,
}

#[derive(Default, Clone)]
pub struct Modifiers {
    inner: Arc<Mutex<ModifiersInner>>
}

impl Modifiers {
    pub fn fork(&self) -> Self {
        let inner = self.inner.lock().unwrap();
        Modifiers { inner: Arc::new(Mutex::new(*inner)) }
    }

    pub fn lock(&self) -> LockResult<MutexGuard<'_, ModifiersInner>> {
        self.inner.lock()
    }
}


