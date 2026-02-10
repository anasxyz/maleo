use std::any::Any;
use crate::{
    FontId,
    Ui,
    widgets::{Rect, Widget},
};

pub struct ButtonWidget {
    id: usize,
    bounds: Rect,
    text: String,
    font: Option<FontId>,
    color: [f32; 4],
    auto_size: bool,
}

impl ButtonWidget {
    pub fn new(id: usize, text: &str) -> Self {
        Self {
            id,
            text: text.to_string(),
            font: None,
            bounds: Rect { x: 0.0, y: 0.0, w: 100.0, h: 40.0 },
            color: [0.0; 4],
            auto_size: false,
        }
    }

    pub fn position(&mut self, x: f32, y: f32) -> &mut Self {
        self.bounds.x = x;
        self.bounds.y = y;
        self
    }

    pub fn size(&mut self, w: f32, h: f32) -> &mut Self {
        self.auto_size = false;
        self.bounds.w = w;
        self.bounds.h = h;
        self
    }

    pub fn font(&mut self, font_id: FontId) -> &mut Self {
        self.font = Some(font_id);
        self
    }

    pub fn auto_size(&mut self) -> &mut Self {
        self.auto_size = true;
        self
    }

    pub fn text(&mut self, text: impl Into<String>) -> &mut Self {
        self.text = text.into();
        self
    }

    pub fn color(&mut self, color: [f32; 4]) -> &mut Self {
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
        let font_id = self.font.expect(
            "ButtonWidget has no font set — call .font(font_id) before rendering"
        );

        let padding = ui.fonts.default_padding;
        let (text_w, text_h) = ui.fonts.measure(&self.text, font_id);

        let bounds = if self.auto_size {
            Rect {
                x: self.bounds.x,
                y: self.bounds.y,
                w: text_w + padding * 2.0,
                h: text_h + padding * 2.0,
            }
        } else {
            self.bounds
        };

        ui.rect(bounds.x, bounds.y, bounds.w, bounds.h, self.color, [0.0; 4], 0.0);

        let text_x = bounds.x + (bounds.w - text_w) / 2.0;
        let text_y = bounds.y + (bounds.h - text_h) / 2.0;
        ui.text(&self.text, font_id, text_x, text_y);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
