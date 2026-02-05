use std::hash::Hash;

#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    Text,
    SystemRed,
    SystemOrange,
    SystemYellow,
    SystemGreen,
    SystemBlue,
    SystemIndigo,
    SystemPurple,
    SystemPink,
    Custom { r: f32, g: f32, b: f32, a: f32 },
}

impl Hash for Color {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Color::Text => {
                "system_text".hash(state);
            }
            Color::SystemRed => {
                "system_red".hash(state);
            }
            Color::SystemOrange => {
                "system_orange".hash(state);
            }
            Color::SystemYellow => {
                "system_yellow".hash(state);
            }
            Color::SystemGreen => {
                "system_green".hash(state);
            }
            Color::SystemBlue => {
                "system_blue".hash(state);
            }
            Color::SystemIndigo => {
                "system_indigo".hash(state);
            }
            Color::SystemPurple => {
                "system_violet".hash(state);
            }
            Color::SystemPink => {
                "system_pink".hash(state);
            }
            Color::Custom { r, g, b, a } => {
                r.to_bits().hash(state);
                g.to_bits().hash(state);
                b.to_bits().hash(state);
                a.to_bits().hash(state);
            }
        }
    }
}

impl Eq for Color {}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self::new_with_alpha(r, g, b, 1.0)
    }

    pub fn new_with_alpha(r: u8, g: u8, b: u8, a: f32) -> Self {
        Self::Custom {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a,
        }
    }
}
