use turubai::{
    color::Color,
    composition::{HStack, HorizontalAlignment, Spacer, VStack, VerticalAlignment},
    elements::{Element, Text, TextDecoration, TextDecorationLine, TextLineStyle},
    font::Font,
    runtime::WindowTemplate,
    Application,
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
        let arial_font = Font::new("Inter", 16, turubai::font::FontWeight::Black, true);

        let thick_decoration = TextDecoration {
            underline: TextDecorationLine {
                color: Color::SystemPink,
                style: TextLineStyle::Thick,
            },
            ..Default::default()
        };

        let white = Color::new(255, 255, 255);

        turubai!(
            WindowTemplate(title: "Hello, World!") {
                VStack(spacing: 0.0, alignment: HorizontalAlignment::Center) {
                    // Text("Hello, World!", font: courier_font.clone()),
                    // Dummy text to push the content down
                    Text(" ", font: courier_font.clone()),
                    Spacer(),
                    HStack(spacing: 0.0, text::font: arial_font.clone(), alignment: VerticalAlignment::Center) {
                        Spacer(),
                        Text("CRAYON ", color: white.clone()),
                        HStack(spacing: 0.0, text::decoration: thick_decoration.clone()) {
                            Text("T", color: Color::SystemRed),
                            Text("U", color: Color::SystemOrange),
                            Text("R", color: Color::SystemYellow),
                            Text("U", color: Color::SystemGreen),
                            Text("B", color: Color::SystemBlue),
                            Text("A", color: Color::SystemIndigo),
                            Text("I", color: Color::SystemPurple),
                        },
                        Spacer(),
                    },
                    Spacer(),
                    Text("We support Linux and GTK4!", font: courier_font.clone(), color: white.clone()),
                    Text("2026 Ngishu, Mark Sadiki", font: courier_font.clone(), color: white.clone()),
                    // Dummt text for spacing
                    Text("", font: courier_font.clone(), color: white.clone()),
                }
                // .background_color(Color::new(0, 0, 0))
            },
        )
    }
}

fn main() {
    turubai::runtime::turubai_main(Box::new(MyApplication::default()));
}
