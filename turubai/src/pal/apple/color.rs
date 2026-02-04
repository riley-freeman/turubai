use cacao::color::Color;

#[derive(Debug, Clone)]
pub struct NativeColor {
    color: Color,
}

impl NativeColor {
    pub fn new(t_color: crate::color::Color) -> Self {
        match t_color {
            crate::color::Color::Text => Self {
                color: Color::Label,
            },

            crate::color::Color::SystemRed => Self {
                color: Color::SystemRed,
            },

            crate::color::Color::SystemOrange => Self {
                color: Color::SystemOrange,
            },

            crate::color::Color::SystemYellow => Self {
                color: Color::SystemYellow,
            },

            crate::color::Color::SystemGreen => Self {
                color: Color::SystemGreen,
            },

            crate::color::Color::SystemBlue => Self {
                color: Color::SystemBlue,
            },

            crate::color::Color::SystemIndigo => Self {
                color: Color::SystemIndigo,
            },
            crate::color::Color::SystemPurple => Self {
                color: Color::SystemPurple,
            },

            crate::color::Color::Custom { r, g, b, a } => Self {
                color: Color::rgba(
                    (r * 255.0) as u8,
                    (g * 255.0) as u8,
                    (b * 255.0) as u8,
                    (a * 255.0) as u8,
                ),
            },
        }
    }

    pub fn os_color(&self) -> Color {
        self.color.clone()
    }
}
