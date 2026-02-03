use turubai::{Application, composition::{HStack, HorizontalAlignment, VStack}, elements::{Element, Text}, runtime::WindowTemplate, font::Font};
use turubai_macros::turubai;

#[derive(Default)]
struct MyApplication {
}

impl Application for MyApplication {
    fn markup(&self) -> Box<dyn Element> {
        let courier_font = Font::new("Courier New", 12, turubai::font::FontWeight::Regular, false, false);
        let arial_font = Font::new("Arial", 16, turubai::font::FontWeight::Black, false, false);

        Box::new(turubai!(
            WindowTemplate(title: Some("Hello World! (Example)".to_string())) {
                VStack(spacing: 0.0, alignment: HorizontalAlignment::Center) {
                    Text("Hello, World!", font: courier_font),
                    VStack(spacing: 4.0) {
                        HStack(spacing: 16.0, text::font: arial_font) {
                            Text("️Turubai"),
                            Text("🎨"),
                            Text("Crayon"),
                        },
                        Text("2026 Ngishu, Mark Sadiki."),
                    }
                }
            },
        ))
    }
} 

fn main() {
    turubai::runtime::turubai_main(Box::new(MyApplication::default()));
}

