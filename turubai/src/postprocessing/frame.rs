use crate::{Unit, elements::{Element, Modifiers}};

pub struct Frame {
    max_width: Unit,
    max_height: Unit,
    min_width: Unit,
    min_height: Unit,
    width: Unit,
    height: Unit,

    child: Box<dyn Element>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct FrameModifiers {
    pub max_width: Unit,
    pub max_height: Unit,
    pub min_width: Unit,
    pub min_height: Unit,
    pub width: Unit,
    pub height: Unit,
}

impl Default for FrameModifiers {
    fn default() -> Self {
        Self {
            max_width: Unit::Auto,
            max_height: Unit::Auto,
            min_width: Unit::Auto,
            min_height: Unit::Auto,
            width: Unit::Auto,
            height: Unit::Auto,
        }
    }
}

impl Frame {
    pub fn new(modifiers: Modifiers, child: Box<dyn Element>) -> Self {
        let lock = modifiers.lock().unwrap();
        Self {
            max_width: lock.frame.max_width,
            max_height: lock.frame.max_height,
            min_width: lock.frame.min_width,
            min_height: lock.frame.min_height,
            width: lock.frame.width,
            height: lock.frame.height,
            child
        }
    }
}

pub fn frame(child: Box<dyn Element>, modifiers: Modifiers) -> Frame {
    Frame::new(modifiers, child)
}
