use turubai_macros::turubai;
use turubai_types::Modifiers;

trait Living {
    fn say_hello(&self, spacing: u32);
}

struct Person {
    name: String,
    children : Vec<Box<dyn Living>>
}


impl Person {
    fn new_0(modifiers: Modifiers, children: fn(Modifiers) -> Vec<Box<dyn Living>>) -> Self {
        Self {
            name: "John Doe".to_string(),
            children: children(modifiers),
        }
    }

    fn new_1(name: &str, modifiers: Modifiers, children: fn(Modifiers) -> Vec<Box<dyn Living>>) -> Self {
        Self {
            name: name.to_string(),
            children: children(modifiers),
        }
    }
}

impl Living for Person {
    fn say_hello(&self, spacing: u32) {
        let indent = " ".repeat(spacing as usize);
        print!("{}{}", indent, self.name);
        
        if self.children.is_empty() {
            println!(".")
        } else {
            println!(":")
        }

        for child in &self.children {
            child.say_hello(spacing + 2);
        }
    }
}
fn main() {
    let john_doe = turubai!(
        Person()
    );

    let family = turubai!(
        Person("Ngishu Family") {
            Person("Mark Ngishu"),
            Person("Rose Ngishu"),
            Person("Ziki"),
            Person("Markelle"),
            Person("Sadiki"),
            Person("Matthew"),
            Person()
        }
    );

    john_doe.say_hello(0);
    family.say_hello(0);
}
