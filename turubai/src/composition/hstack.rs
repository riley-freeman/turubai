use std::sync::Arc;
use std::sync::Mutex;

use turubai_types::Modifiers;

use crate::elements::Element;

pub struct HStack {
    inner: Arc<Mutex<HStackInner>>,
}
struct HStackInner {
    contents: String,
    modifiers: Modifiers,
    children: Vec<Box<dyn Element>>,
}



impl HStack {
    pub fn new(modifiers: Modifiers, children: fn(Modifiers) -> Vec<Box<dyn Element>>) -> Self {
        todo!()
    }
}

impl Element for HStack {
    fn name(&self) -> &'static str {
        "hstack"
    }

    fn display_name(&self) -> &'static str {
        "HStack"
    }
}