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
    name: String,
    size: u32,
    weight: FontWeight,
    italicized: bool,
    underlined: bool,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Font {
    pub(crate) inner: Arc<FontInner>
}

impl Font {
    pub fn new(name: &str, size: u32, weight: FontWeight, italic: bool, underline: bool) -> Self {
        let mut name = name.to_string();
        name.push_str(&format!("-{:?}", weight));
        if italic {
            name.push_str("-Italic");
        }

        let inner = FontInner {
            name: name.to_string(),
            size,
            weight,
            italicized: italic,
            underlined: underline,
        };

        Self {
            inner: Arc::new(inner)
        }
    }

    pub fn name(&self) -> String {
        self.inner.name.to_string()
    }

    pub fn size(&self) -> f32 {
        self.inner.size as _
    }
}

impl Default for Font {
    fn default() -> Self {
        Self::new("Arial", 12, FontWeight::Regular, false, false)
    }
}

#[cfg(test)]
mod tests {
    use crate::font::Font;

    #[test]
    fn font_name() {
        let font = Font::new("Ariel", 16, super::FontWeight::Regular, false, false);
        assert_eq!(font.name(), "Ariel-Regular");
    }
}


