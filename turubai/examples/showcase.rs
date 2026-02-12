use turubai::{
    color::Color,
    composition::{HStack, HorizontalAlignment, Spacer, VStack, VerticalAlignment},
    elements::Modifiers,
    elements::{Element, Text, TextDecoration, TextDecorationLine, TextLineStyle},
    font::Font,
    postprocessing::{background_color, padding},
    runtime::WindowTemplate,
    Application,
    Unit::{Em, Pixels},
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
                VStack(spacing: Em(0.0), alignment: HorizontalAlignment::Center, text::color: white.clone()) {
                    Spacer()
                    HStack(spacing: Em(0.0), text::font: inter_font.clone(), alignment: VerticalAlignment::Center) {
                        Spacer()
                        Text("CRAYON", color: white.clone())
                        HStack(spacing: Pixels(-1.0), text::decoration: thick_decoration) {
                            Text("T", color: Color::SystemRed)
                            Text("U", color: Color::SystemOrange)
                            Text("R", color: Color::SystemYellow)
                            Text("U", color: Color::SystemGreen)
                            Text("B", color: Color::SystemBlue)
                            Text("A", color: Color::SystemIndigo)
                            Text("I", color: Color::SystemPurple)
                            Text("  ")
                        }
                        .padding(all: Em(0.25))
                        Spacer()
                    }
                    Text("Nevermind, I hate commas.", font: courier_font.clone())
                        .padding(all: Em(0.25))
                    Spacer()
                }
                .background_color(Color::new(0, 0, 0))
            }
        )
    }
}

fn main() {
    let app = MyApplication::default();
    turubai::runtime::turubai_main(app);
}
