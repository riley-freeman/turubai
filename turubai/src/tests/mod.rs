use turubai_macros::turubai;

#[test]
fn hello() {
    turubai!(
        VStack() {
            Text("Hello, World!", align: crate::elements::TextAlign::Center),
            HStack() {
                Text("Crayon INK Turubari Compositor", size: 24.0, weight: crate::elements::FontWeight::Normal),
                Text(" • "),
                Text("Version 0.0.1", size: 24.0, weight: crate::elements::FontWeight::Bold),
                Text(" • "),
                Text("Mark Sadiki Ngishu, 2026", size: 24.0, weight: crate::elements::FontWeight::Bold),
            },
        },
    );

    let text = turubai!(
        Text("Hello, World!", align: crate::elements::TextAlign::Center),
    );
}