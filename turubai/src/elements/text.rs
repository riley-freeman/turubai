use std::sync::Arc;
use std::sync::Mutex;
use crate::elements::Element;

pub struct Text {
    inner: Arc<Mutex<TextInner>>,
}
struct TextInner {
    contents: String,
    parameters: TextParameters,
    children: Vec<Box<dyn Element>>,
}

#[derive(Default, PartialEq, Eq, Clone, Copy)]
pub enum TextAlign {
    #[default]
    Leading,
    Center,
    Ending,
}

#[derive(Default, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum FontWeight {
    ExtraBlack  = 950,
    Black       = 900,
    ExtraBold   = 800,
    Bold        = 700,
    SemiBold    = 600,
    Medium      = 500,

    #[default]
    Normal      = 400,
    SemiLight   = 350,
    Light       = 300,
    ExtraLight  = 200,
    Thin        = 100,

}

#[derive(Default, Clone, Copy)]
pub struct TextParameters {
    pub align: TextAlign,
    pub size: f32,

    pub weight: FontWeight,
}

impl Text {
    pub fn new(contents: &str, parameters: TextParameters, children: fn() -> Vec<Box<dyn Element>>) -> Self {
        let inner = TextInner {
            contents: String::from(contents),
            parameters: parameters,
            children: children(),
        };
        Self::from(inner)
    }
}

impl From<TextInner> for Text {
    fn from(value: TextInner) -> Self {
        Self {
            inner: Arc::new(Mutex::new(value))
        }
    }
}

impl Element for Text {
    fn name(&self) -> &'static str {
        "text"
    }
    fn display_name(&self) -> &'static str {
        "Text"
    }
}