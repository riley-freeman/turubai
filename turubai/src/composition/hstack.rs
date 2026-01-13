use std::sync::Arc;
use std::sync::Mutex;

use crate::composition::StackParameters;
use crate::elements::Element;

pub struct HStack {
    inner: Arc<Mutex<HStackInner>>,
}
struct HStackInner {
    contents: String,
    parameters: StackParameters,
    children: Vec<Box<dyn Element>>,
}



impl HStack {
    pub fn new(parameters: StackParameters, children: fn() -> Vec<Box<dyn Element>>) -> Self {
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