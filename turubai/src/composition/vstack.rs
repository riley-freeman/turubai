use std::sync::Arc;
use std::sync::Mutex;

use turubai_types::Modifiers;

use crate::elements::Element;

pub struct VStack {
    inner: Arc<Mutex<VStackInner>>,
}
struct VStackInner {
    contents: String,
    modifiers: Modifiers,
    children: Vec<Box<dyn Element>>,
}



impl VStack {
    pub fn new(modifiers: Modifiers, children: fn(Modifiers) -> Vec<Box<dyn Element>>) -> Self {
        todo!()
    }
}

impl Element for VStack {
    fn name(&self) -> &'static str {
        "vstack"
    }

    fn display_name(&self) -> &'static str {
        "VStack"
    }
}