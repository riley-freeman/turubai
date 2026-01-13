mod text;

pub use text::*;

pub trait Element {
    fn name(&self) -> &'static str;
    fn display_name(&self) -> &'static str;
}
