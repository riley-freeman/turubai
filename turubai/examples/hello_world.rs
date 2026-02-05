use turubai::{
    color::Color,
    composition::{HStack, HorizontalAlignment, VStack},
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
        let arial_font = Font::new("Inter", 16, turubai::font::FontWeight::Black, true);

        let strike_though = TextDecoration {
            strike_through: TextDecorationLine {
                style: TextLineStyle::Single,
                color: Color::SystemRed,
            },
            ..Default::default()
        };

        let underline = TextDecoration {
            underline: TextDecorationLine {
                style: TextLineStyle::Double,
                ..Default::default()
            },
            ..Default::default()
        };

        let thick = TextDecoration {
            underline: TextDecorationLine {
                color: Color::SystemPink,
                style: TextLineStyle::Thick,
            },
            ..Default::default()
        };

        Box::new(turubai!(
            WindowTemplate(title: Some("Hello World! (Example)".to_string())) {
                VStack(spacing: 0.0, alignment: HorizontalAlignment::Center) {
                    Text("Hello, World!", font: courier_font),
                    VStack(spacing: 2.0) {
                        HStack(spacing: 8.0, text::font: arial_font) {
                            Text("CRAYON"),
                            HStack(spacing: 0.0, text::decoration: thick) {
                                Text("T", color: Color::SystemRed),
                                Text("U", color: Color::SystemOrange),
                                Text("R", color: Color::SystemYellow),
                                Text("U", color: Color::SystemGreen),
                                Text("B", color: Color::SystemBlue),
                                Text("A", color: Color::SystemIndigo),
                                Text("I", color: Color::SystemPurple),
                            },
                        },
                        // HStack() {
                        //     Text("Now supports "),
                        //     Text("colors", decoration: strike_though),
                        //     Text(" lines!", decoration: underline),
                        // }
                    }
                }
            },
        ))
    }
}

fn main() {
    turubai::runtime::turubai_main(Box::new(MyApplication::default()));
}
