use crate::Color;

#[derive(Clone)]
pub struct Settings {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub clear_color: Color,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            title: "demo".to_string(),
            width: 800,
            height: 600,
            clear_color: Color::new(0.1, 0.1, 0.12, 1.0),
        }
    }
}
