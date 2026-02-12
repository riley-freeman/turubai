use turubai::{
    composition::{HStack, HorizontalAlignment, Spacer, VStack, VerticalAlignment},
    elements::Modifiers,
    elements::{Element, Text},
    runtime::WindowTemplate,
    Application,
    Unit::Em,
};
use turubai_macros::turubai;

#[derive(Default)]
struct MyApplication {}

impl Application for MyApplication {
    fn id(&self) -> &'static str {
        "org.example.spacers"
    }

    fn markup(&self) -> Box<dyn Element> {
        turubai!(
            WindowTemplate(title: "Spacers (Example)") {
                VStack(spacing: Em(0.0), alignment: HorizontalAlignment::Center) {
                    Spacer(),
                    HStack(spacing: Em(0.0), alignment: VerticalAlignment::Center) {
                        Text("👋"),
                        Spacer(),
                        Text("Wow, look at all these spacers!"),
                        Spacer(),
                        Text("🌎"),
                    },
                    Spacer(),
                }
            },
        )
    }
}

fn main() {
    let app = MyApplication::default();
    turubai::runtime::turubai_main(MyApplication::default());
}
