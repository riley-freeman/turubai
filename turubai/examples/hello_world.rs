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
        let arial_font = Font::new("Inter", 16, turubai::font::FontWeight::Black, true);

        let thick_decoration = TextDecoration {
            underline: TextDecorationLine {
                color: Color::SystemPink,
                style: TextLineStyle::Thick,
            },
            ..Default::default()
        };

        Box::new(turubai!(
            WindowTemplate(title: Some("Crayon Turubai!".to_string())) {
                VStack(spacing: 0.0, alignment: HorizontalAlignment::Center) {
                    // Text("Hello, World!", font: courier_font.clone()),
                    // Dummy text to push the content down
                    Text(" ", font: courier_font.clone()),
                    Spacer(),
                    VStack(spacing: 2.0) {
                        HStack(spacing: 8.0, text::font: arial_font.clone()) {
                            Text("CRAYON"),
                            HStack(spacing: 0.0, text::decoration: thick_decoration.clone()) {
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
                    },
                    Spacer(),
                    Text("We support Spacers!", font: courier_font.clone()),
                    Text("(C) 2026 Sadiki Industries!", font: courier_font.clone()),
                }
            },
        ))
    }
}

fn main() {
    turubai::runtime::turubai_main(Box::new(MyApplication::default()));
}
