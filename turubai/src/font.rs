use std::{path::{Path, PathBuf}, sync::Arc};

#[derive(Default, Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub enum FontWeight {
    ExtraBlack  = 950,
    Black       = 900,
    ExtraBold   = 800,
    Bold        = 700,
    SemiBold    = 600,
    Medium      = 500,

    #[default]
    Regular      = 400,
    SemiLight   = 350,
    Light       = 300,
    ExtraLight  = 200,
    Thin        = 100,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct FontInner {
    family: String,
    size: u32,
    weight: FontWeight,
    italicized: bool,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Font {
    pub(crate) inner: Arc<FontInner>
}

impl Font {
    pub fn new(family: &str, size: u32, weight: FontWeight, italic: bool) -> Self {
        let mut fam = family.to_string();

        let inner = FontInner {
            family: fam.to_string(),
            size,
            weight,
            italicized: italic,
        };

        Self {
            inner: Arc::new(inner)
        }
    }

    pub fn name(&self) -> String {
        self.inner.family.to_string()
    }

    pub fn size(&self) -> f32 {
        self.inner.size as _
    }

    pub fn weight(&self) -> FontWeight {
        self.inner.weight
    }

    pub fn is_italic(&self) -> bool {
        self.inner.italicized
    }
}

impl Default for Font {
    fn default() -> Self {
        Self::new("Arial", 12, FontWeight::Regular, false)
    }
}

