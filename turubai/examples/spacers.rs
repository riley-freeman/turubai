use turubai::{
    color::Color,
    composition::{HStack, HorizontalAlignment, Spacer, VStack},
    elements::{Element, Text, TextDecoration, TextDecorationLine, TextLineStyle},
    font::Font,
    runtime::WindowTemplate,
    Application,
};
use turubai_macros::turubai;

#[derive(Default)]
struct MyApplication {}

impl Application for MyApplication {
    fn markup(&self) -> Box<dyn Element> {
        let courier_font = Font::new("Courier New", 12, turubai::font::FontWeight::Regular, false);

        Box::new(turubai!(
            WindowTemplate(title: Some("Hello World! (Example)".to_string())) {
                VStack(spacing: 0.0, alignment: HorizontalAlignment::Center) {
                    Text("One", font: courier_font.clone()),
                    Spacer(),
                    Text("Two", font: courier_font.clone()),
                    Spacer(),
                    Text("Three", font: courier_font.clone()),
                }
            },
        ))
    }
}

fn main() {
    turubai::runtime::turubai_main(Box::new(MyApplication::default()));
}
