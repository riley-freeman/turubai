use turubai::{
    color::Color,
    composition::VStack,
    elements::{Modifiers, Text},
    postprocessing::{background_color, padding},
    runtime::WindowTemplate,
    Application,
    Unit::Em,
};
use turubai_macros::turubai;

#[derive(Default)]
struct MyApplication {}

impl Application for MyApplication {
    fn id(&self) -> &'static str {
        "com.itsjustbox.crayon.turubai.hello_world"
    }
    fn markup(&self) -> Box<dyn turubai::elements::Element> {
        let black = Color::new_with_alpha(0, 0, 0, 0.75);
        let white = Color::new(255, 255, 255);
        turubai!(
            WindowTemplate(title: "Turubai Composed.") {
                VStack(text::color: white) {
                    Text("👋")
                    Text("Hello, World!")
                    Text("From the Turubai Compositor")
                }
                .padding(all: Em(1.0))
                .background_color(black)
            }
        )
    }
}

fn main() {
    let app = MyApplication::default();
    turubai::runtime::turubai_main(app);
}
