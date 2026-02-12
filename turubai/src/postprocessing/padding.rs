use crate::elements::{Element, Modifiers};
use crate::shadow::ShadowDescriptor;
use crate::units::Unit;

#[derive(Clone, Copy, PartialEq)]
pub struct PaddingModifiers {
    pub all: Unit,
    pub top_bottom: Option<Unit>,
    pub left_right: Option<Unit>,
    pub top: Option<Unit>,
    pub bottom: Option<Unit>,
    pub left: Option<Unit>,
    pub right: Option<Unit>,
}

impl Default for PaddingModifiers {
    fn default() -> Self {
        Self {
            all: Unit::Pixels(0.0),
            top_bottom: None,
            left_right: None,
            top: None,
            bottom: None,
            left: None,
            right: None,
        }
    }
}

impl PaddingModifiers {
    pub fn resolve_top(&self) -> Unit {
        self.top.or(self.top_bottom).unwrap_or(self.all)
    }

    pub fn resolve_bottom(&self) -> Unit {
        self.bottom.or(self.top_bottom).unwrap_or(self.all)
    }

    pub fn resolve_left(&self) -> Unit {
        self.left.or(self.left_right).unwrap_or(self.all)
    }

    pub fn resolve_right(&self) -> Unit {
        self.right.or(self.left_right).unwrap_or(self.all)
    }
}

pub struct Padding {
    pub top: Unit,
    pub bottom: Unit,
    pub left: Unit,
    pub right: Unit,
    pub child: Box<dyn Element>,
}

impl Padding {
    pub fn new(
        top: Unit,
        bottom: Unit,
        left: Unit,
        right: Unit,
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

pub fn padding(child: Box<dyn Element>, modifiers: Modifiers) -> Padding {
    let lock = modifiers.lock().unwrap();
    let pm = &lock.padding;
    Padding::new(
        pm.resolve_top(),
        pm.resolve_bottom(),
        pm.resolve_left(),
        pm.resolve_right(),
        child,
    )
}
