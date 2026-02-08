use turubai::{
    color::Color,
    composition::{HStack, HorizontalAlignment, Spacer, VStack},
    elements::{Element, Modifiers, Text, TextDecoration, TextDecorationLine, TextLineStyle},
    font::{Font, FontWeight},
    runtime::WindowTemplate,
    Application,
};
use turubai_macros::turubai;

#[derive(Default)]
struct TextDemoApp {}

impl Application for TextDemoApp {
    fn id(&self) -> &'static str {
        "org.example.text_demo"
    }

    fn markup(&self) -> Box<dyn Element> {
        let base_font = Font::new("Sans", 14, FontWeight::Regular, false);
        let bold_font = Font::new("Sans", 14, FontWeight::Bold, false);
        let italic_font = Font::new("Sans", 14, FontWeight::Regular, true);
        let large_font = Font::new("Serif", 24, FontWeight::ExtraBold, false);

        let red = Color::SystemRed;
        let blue = Color::SystemBlue;
        let green = Color::SystemGreen;

        let underline = TextDecoration {
            underline: TextDecorationLine {
                style: TextLineStyle::Single,
                color: Color::Text,
            },
            ..Default::default()
        };

        let strike = TextDecoration {
            strike_through: TextDecorationLine {
                style: TextLineStyle::Single,
                color: Color::SystemRed,
            },
            ..Default::default()
        };

        let colored_underline = TextDecoration {
            underline: TextDecorationLine {
                style: TextLineStyle::Double,
                color: Color::SystemBlue,
            },
            ..Default::default()
        };

        Box::new(turubai!(
            WindowTemplate(title: "GTK Text Demo".to_string()) {
                VStack(spacing: 10.0, alignment: HorizontalAlignment::Center) {
                    Text("Standard Text", font: base_font.clone()),

                    Text("Bold Text", font: bold_font.clone()),

                    Text("Italic Text", font: italic_font.clone()),

                    Text("Large Serif", font: large_font.clone()),

                    Text("Red Text", color: red.clone(), font: base_font.clone()),

                    Text("Underlined", decoration: underline.clone(), font: base_font.clone()),

                    Text("Strikethrough (Red Line)", decoration: strike.clone(), font: base_font.clone()),

                    Text("Double Blue Underline", decoration: colored_underline.clone(), font: base_font.clone()),

                    HStack(spacing: 5.0) {
                        Text("Mixed: ", font: base_font.clone()),
                        Text("Red", color: red.clone(), font: bold_font.clone()),
                        Text(" & ", font: base_font.clone()),
                        Text("Blue", color: blue.clone(), font: italic_font.clone()),
                    }
                }
            },
        ))
    }
}

fn main() {
    turubai::runtime::turubai_main(Box::new(TextDemoApp::default()));
}
