use turubai::{Application, composition::{HStack, VStack}, elements::{Element, Text}, runtime::WindowTemplate};
use turubai_macros::turubai;

#[derive(Default)]
struct MyApplication {
}

impl Application for MyApplication {
    fn markup(&self) -> Box<dyn Element> {
        Box::new(turubai!(
            WindowTemplate(title: Some("Hello World! (Example)".to_string())) {
                VStack(spacing: 8.0) {
                    Text("Hello, World!"),
                    HStack(spacing: 8.0) {
                        Text("Crayon"),
                        Text("🖍️"),
                        Text("️Turubari"),
                    }
                    // HStack {
                    //     Text("Turubai!"),
                    //     Text("🎨"),
                    //     Text("❤️"),
                    // },
                    // HStack {
                    //     Text("Mark Sadiki"),
                    //     Text("🥷🏿"),
                    //     Text("Ngishu"),
                    //     Text("❤️"),
                    // }
                }
            },
        ))
    }
} 

fn main() {
    turubai::runtime::turubai_main(Box::new(MyApplication::default()));
}

