use turubai_macros::turubai;

#[test]
fn hello() {
    use turubai_types::text::TextAlign;
    use turubai_types::font::FontWeight;


    turubai!(
        VStack(text::size: 32.0) {
            Text("Hello, World!", align: TextAlign::Center),
            HStack(text::align: TextAlign::Center) {
                Text("Crayon INK Turubari Compositor", size: 24.0, weight: FontWeight::Bold),
                Text(" • "),
                Text("Version 0.0.1", size: 24.0, weight: FontWeight::Normal),
                Text(" • "),
                Text("Mark Sadiki Ngishu", size: 24.0, weight: FontWeight::Normal),
            },
        },
    );

    let text = turubai!(
        Text("Hello, World!", align: TextAlign::Center),
    );
}