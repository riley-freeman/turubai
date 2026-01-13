use std::{collections::HashMap, sync::LazyLock};

use syn::{Path, parse_str};

pub static NAMESPACE: &'static str = "crate";

pub struct ElementEntry {
    path_str: String,
    parameter_struct_str: String,
}

impl ElementEntry {
    pub fn path(&self) -> Path {
        parse_str(&self.path_str).expect("Failed to parse path")
    }

    pub fn parameter_struct(&self) -> Path {
        parse_str(&self.parameter_struct_str).expect("Failed to parse parameter struct path")
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
        parameter_struct_str: format!("{namespace}::elements::TextParameters"),
    };

    // Composition
    let v_stack = ElementEntry {
        path_str: format!("{namespace}::composition::VStack"),
        parameter_struct_str: format!("{namespace}::composition::StackParameters"),
    };
    let h_stack = ElementEntry {
        path_str: format!("{namespace}::composition::HStack"),
        parameter_struct_str: format!("{namespace}::composition::StackParameters"),
    };

    HashMap::from([
        ("Text", text),
        ("VStack", v_stack),
        ("HStack", h_stack),
    ])
});

