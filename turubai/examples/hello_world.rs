use turubai::{
    color::Color,
    composition::{HStack, HorizontalAlignment, Spacer, VStack, VerticalAlignment},
    elements::{Element, Text, TextDecoration, TextDecorationLine, TextLineStyle},
    font::Font,
    runtime::WindowTemplate,
    Application, Em,
};
use turubai_macros::turubai;

#[derive(Default)]
struct MyApplication {}

impl Application for MyApplication {
    fn id(&self) -> &'static str {
        "org.example.hello_world"
    }

    fn markup(&self) -> Box<dyn Element> {
        let courier_font = Font::new("Courier New", 12, turubai::font::FontWeight::Regular, false);
        let courier_font_clone = courier_font.clone();
        let inter_font = Font::new("Inter", 16, turubai::font::FontWeight::Black, true);

        let thick_decoration = TextDecoration {
            underline: TextDecorationLine {
                color: Color::SystemPink,
                style: TextLineStyle::Thick,
            },
            ..Default::default()
        };

        let white = Color::new(255, 255, 255);
        let black = Color::new(0, 0, 0);
        let primary = Color::new(255, 67, 86);

        turubai!(
            WindowTemplate(title: "Hello, World!") {
                VStack(spacing: 0.0, alignment: HorizontalAlignment::Center) {
                    // Text("Hello, World!", font: courier_font_clone, color: black.clone())
                    //     .padding(Em::new(0.25))
                    //     .background_color(white.clone())
                    //     .padding(Em::new(1.0)),

                    // Dummy text to push the content down
                    Spacer(),
                    HStack(spacing: 0.0, text::font: inter_font.clone(), alignment: VerticalAlignment::Center) {
                        Spacer(),
                        Text("CRAYON ", color: white.clone()),
                        HStack(spacing: -1.0) {
                            Text("T", color: Color::SystemRed),
                            Text("U", color: Color::SystemOrange),
                            Text("R", color: Color::SystemYellow),
                            Text("U", color: Color::SystemGreen),
                            Text("B", color: Color::SystemBlue),
                            Text("A", color: Color::SystemIndigo),
                            Text("I", color: Color::SystemPurple),
                            Text("  "),
                        }
                        .padding(Em::new(0.5))
                        .background_color(white.clone())
                        .padding(Em::new(0.5)),
                        Spacer(),
                    },
                    Text("I think we need a little space.", font: courier_font.clone())
                        .padding(Em::new(0.25)),
                    Spacer(),
                }
                .background_color(Color::new(0, 0, 0))
            },
        )
    }
}

fn main() {
    turubai::runtime::turubai_main(Box::new(MyApplication::default()));
}
