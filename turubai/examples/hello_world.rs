use turubai::{Application, composition::{HStack, HorizontalAlignment, VStack}, elements::{Element, Text}, runtime::WindowTemplate};
use turubai_macros::turubai;

#[derive(Default)]
struct MyApplication {
}

impl Application for MyApplication {
    fn markup(&self) -> Box<dyn Element> {
        Box::new(turubai!(
            WindowTemplate(title: Some("Hello World! (Example)".to_string())) {
                VStack(spacing: 16.0, alignment: HorizontalAlignment::Center) {
                    Text("Hello, World!"),
                    VStack(spacing: 0.0) {
                        Text("Now with TAFFY!"),
                        Text("Now with Center Alignment!"),
                    },
                    HStack(spacing: 16.0) {
                        Text("Crayon"),
                        Text("🖍️"),
                        Text("️Turubari"),
                    },
                }
            },
        ))
    }
} 

fn main() {
    turubai::runtime::turubai_main(Box::new(MyApplication::default()));
}

