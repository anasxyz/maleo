use crate::{
    Ui,
    widgets::{Rect, Widget},
};

pub struct ButtonWidget {
    pub id: usize,
    pub bounds: Rect,
    pub text: String,
    pub font_size: f32,
    pub color: [f32; 4],
}

// builder pattern
impl ButtonWidget {
    pub fn new(id: usize, text: &str) -> Self {
        Self {
            id,
            text: text.to_string(),
            font_size: 16.0,
            bounds: Rect {
                x: 0.0,
                y: 0.0,
                w: 100.0,
                h: 40.0,
            },
            color: [0.0; 4],
        }
    }

    pub fn position(mut self, x: f32, y: f32) -> Self {
        self.bounds.x = x;
        self.bounds.y = y;
        self
    }

    pub fn size(mut self, w: f32, h: f32) -> Self {
        self.bounds.w = w;
        self.bounds.h = h;
        self
    }

    pub fn font_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        self
    }

    pub fn color(mut self, color: [f32; 4]) -> Self {
        self.color = color;
        self
    }
}

impl Widget for ButtonWidget {
    fn id(&self) -> usize {
        self.id
    }

    fn bounds(&self) -> Rect {
        self.bounds
    }

    fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
    }

    fn render(&self, ui: &mut Ui) {
        let bounds = self.bounds;

        ui.rect(
            bounds.x, bounds.y, bounds.w, bounds.h, self.color, [0.0; 4], 0.0,
        );
        ui.text(self.text.as_str(), self.font_size, bounds.x, bounds.y);
    }
}
