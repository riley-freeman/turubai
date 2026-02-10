use std::{collections::HashMap, sync::LazyLock};

use syn::{parse_str, Ident, Path};

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
        parse_str(&self.modifier_memeber)
            .expect("Failed to parse responsible member in Modifiers struct.")
    }
}

pub static POSTPROCESSING_ELEMENTS: LazyLock<HashMap<String, ElementEntry>> = LazyLock::new(|| {
    let namespace = if env!("CARGO_CRATE_NAME").eq(NAMESPACE) {
        "crate"
    } else {
        NAMESPACE
    };

    // Visuals
    let background_color = ElementEntry {
        path_str: format!("turubai::postprocessing::BackgroundColor"),
        modifier_memeber: String::from("background_color"),
    };

    let padding = ElementEntry {
        path_str: "turubai::postprocessing::Padding".to_string(),
        modifier_memeber: "padding".to_string(),
    };

    HashMap::from([
        ("background_color".to_string(), background_color),
        ("padding".to_string(), padding),
    ])
});
