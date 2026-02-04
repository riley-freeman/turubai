use turubai::{
    color::Color,
    composition::{HStack, HorizontalAlignment, VStack},
    elements::{Element, Text},
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
        let arial_font = Font::new("Inter", 16, turubai::font::FontWeight::Black, true);

        Box::new(turubai!(
            WindowTemplate(title: Some("Hello World! (Example)".to_string())) {
                VStack(spacing: 0.0, alignment: HorizontalAlignment::Center) {
                    Text("Hello, World!", font: courier_font),
                    VStack(spacing: 4.0) {
                        HStack(spacing: 16.0, text::font: arial_font) {
                            HStack(spacing: 0.0) {
                                Text("T", color: Color::SystemRed),
                                Text("U", color: Color::SystemOrange),
                                Text("R", color: Color::SystemYellow),
                                Text("U", color: Color::SystemGreen),
                                Text("B", color: Color::SystemBlue),
                                Text("A", color: Color::SystemIndigo),
                                Text("I", color: Color::SystemPurple),
                            },
                            Text("🎨"),
                        },
                        Text("Now supports colors!"),
                    }
                }
            },
        ))
    }
}

fn main() {
    turubai::runtime::turubai_main(Box::new(MyApplication::default()));
}
