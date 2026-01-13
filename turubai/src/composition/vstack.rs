use std::sync::Arc;
use std::sync::Mutex;

use crate::composition::StackParameters;
use crate::elements::Element;

pub struct VStack {
    inner: Arc<Mutex<VStackInner>>,
}
struct VStackInner {
    contents: String,
    parameters: StackParameters,
    children: Vec<Box<dyn Element>>,
}



impl VStack {
    pub fn new(parameters: StackParameters, children: fn() -> Vec<Box<dyn Element>>) -> Self {
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