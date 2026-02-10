use std::env::join_paths;

use crate::elements::{Element, Modifiers};
use crate::shadow::ShadowDescriptor;
use crate::units::{Em, Unit};

pub struct Padding {
    pub top: Box<dyn Unit>,
    pub bottom: Box<dyn Unit>,
    pub left: Box<dyn Unit>,
    pub right: Box<dyn Unit>,
    pub child: Box<dyn Element>,
}

impl Padding {
    pub fn new(
        top: Box<dyn Unit>,
        bottom: Box<dyn Unit>,
        left: Box<dyn Unit>,
        right: Box<dyn Unit>,
        child: Box<dyn Element>,
    ) -> Self {
        Self {
            top,
            bottom,
            left,
            right,
            child,
        }
    }

    pub fn new_0(
        modifiers: Modifiers,
        children: impl FnOnce(Modifiers) -> Vec<Box<dyn Element>>,
    ) -> Self {
        Self {
            top: Em::new(1.0),
            bottom: Em::new(1.0),
            left: Em::new(1.0),
            right: Em::new(1.0),
            child: children(modifiers).pop().unwrap(),
        }
    }

    pub fn new_1(
        units: Box<dyn Unit>,
        modifiers: Modifiers,
        children: impl FnOnce(Modifiers) -> Vec<Box<dyn Element>>,
    ) -> Self {
        Self {
            top: units.clone_unit(),
            bottom: units.clone_unit(),
            left: units.clone_unit(),
            right: units.clone_unit(),
            child: children(modifiers).pop().unwrap(),
        }
    }

    pub fn new_2(
        top_bottom: Box<dyn Unit>,
        left_right: Box<dyn Unit>,
        modifiers: Modifiers,
        children: impl FnOnce(Modifiers) -> Vec<Box<dyn Element>>,
    ) -> Self {
        Self {
            top: top_bottom.clone_unit(),
            bottom: top_bottom.clone_unit(),
            left: left_right.clone_unit(),
            right: left_right.clone_unit(),
            child: children(modifiers).pop().unwrap(),
        }
    }

    pub fn new_3(
        top: Box<dyn Unit>,
        bottom: Box<dyn Unit>,
        left_right: Box<dyn Unit>,
        modifiers: Modifiers,
        children: impl FnOnce(Modifiers) -> Vec<Box<dyn Element>>,
    ) -> Self {
        Self {
            top: top.clone_unit(),
            bottom: bottom.clone_unit(),
            left: left_right.clone_unit(),
            right: left_right.clone_unit(),
            child: children(modifiers).pop().unwrap(),
        }
    }

    pub fn new_4(
        top: Box<dyn Unit>,
        left: Box<dyn Unit>,
        bottom: Box<dyn Unit>,
        right: Box<dyn Unit>,
        modifiers: Modifiers,
        children: impl FnOnce(Modifiers) -> Vec<Box<dyn Element>>,
    ) -> Self {
        Self {
            top: top.clone_unit(),
            bottom: bottom.clone_unit(),
            left: left.clone_unit(),
            right: right.clone_unit(),
            child: children(modifiers).pop().unwrap(),
        }
    }
}

impl Element for Padding {
    fn name(&self) -> &'static str {
        "padding"
    }

    fn display_name(&self) -> &'static str {
        "Padding"
    }

    fn shadow_descriptor(&self) -> crate::shadow::ShadowDescriptor {
        ShadowDescriptor::padding(
            self.top.to_pixels(None),
            self.bottom.to_pixels(None),
            self.left.to_pixels(None),
            self.right.to_pixels(None),
        )
    }

    fn child_count(&self) -> usize {
        1
    }

    fn for_each_child(&self, f: &mut dyn FnMut(&dyn Element)) {
        f(self.child.as_ref())
    }
}
