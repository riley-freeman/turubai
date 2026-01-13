use std::{collections::HashMap, sync::LazyLock};

use syn::{Ident, Path, parse_str};

pub static NAMESPACE: &'static str = "crate";

pub struct ElementEntry {
    path_str: String,
    modifier_memeber: String,
}

impl ElementEntry {
    pub fn path(&self) -> Path {
        parse_str(&self.path_str).expect("Failed to parse path")
    }

    pub fn modifier_member(&self) -> Ident {
        parse_str(&self.modifier_memeber).expect("Failed to parse responsible member in Modifiers struct.")
    }
}

pub static ELEMENTS: LazyLock<HashMap<&str, ElementEntry>> = LazyLock::new(|| {
    let namespace = if env!("CARGO_CRATE_NAME").eq(NAMESPACE) {
        "crate"
    } else {
        NAMESPACE
    };

    // Visuals
    let text = ElementEntry {
        path_str: format!("{namespace}::elements::Text"),
        modifier_memeber: String::from("text"),
    };

    // Composition
    let v_stack = ElementEntry {
        path_str: format!("{namespace}::composition::VStack"),
        modifier_memeber: String::from("stack"),
    };
    let h_stack = ElementEntry {
        path_str: format!("{namespace}::composition::HStack"),
        modifier_memeber: String::from("stack"),
    };

    HashMap::from([
        ("Text", text),
        ("VStack", v_stack),
        ("HStack", h_stack),
    ])
});

