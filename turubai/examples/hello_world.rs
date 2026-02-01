use turubai::{Application, composition::{HStack, HorizontalAlignment, VStack}, elements::{Element, Text}, runtime::WindowTemplate, font::Font};
use turubai_macros::turubai;

#[derive(Default)]
struct MyApplication {
}

impl Application for MyApplication {
    fn markup(&self) -> Box<dyn Element> {
        let font = Font::new("Arial", 16, turubai::font::FontWeight::Black, false, false);

        Box::new(turubai!(
            WindowTemplate(title: Some("Hello World! (Example)".to_string())) {
                VStack(spacing: 16.0, alignment: HorizontalAlignment::Center) {
                    Text("Hello, World!"),
                    HStack(spacing: 16.0, text::font: font) {
                        Text("Crayon "),
                        Text("🖍️"),
                        Text("️Turubai"),
                    },
                }
            },
        ))
    }
} 

fn main() {
    turubai::runtime::turubai_main(Box::new(MyApplication::default()));
}

