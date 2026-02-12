use crate::{
    elements::{Element, Modifiers},
    shadow::{ShadowDescriptor, ShadowNode},
};

pub struct Spacer {
    modifiers: Modifiers,
    children: Vec<Box<dyn Element>>,
}

impl Spacer {
    fn new(
        modifiers: Modifiers,
        children: impl FnOnce(Modifiers) -> Vec<Box<dyn Element>>,
    ) -> Self {
        Self {
            modifiers: modifiers.clone(),
            children: children(modifiers.fork()),
        }
    }

    pub fn turubai_new_with_0_args(
        modifiers: Modifiers,
        children: impl FnOnce(Modifiers) -> Vec<Box<dyn Element>>,
    ) -> Self {
        Self::new(modifiers, children)
    }
}

impl Element for Spacer {
    fn name(&self) -> &'static str {
        "spacer"
    }

    fn display_name(&self) -> &'static str {
        "Spacer"
    }

    // Children will basically be forgotten
    fn child_count(&self) -> usize {
        0_usize
    }
    fn for_each_child(&self, _f: &mut dyn FnMut(&dyn Element)) {}

    fn shadow_descriptor(&self) -> crate::shadow::ShadowDescriptor {
        ShadowDescriptor::spacer()
    }
}
