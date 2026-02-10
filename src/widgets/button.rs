use std::any::Any;
use crate::{
    FontId, MouseState, Ui,
    widgets::{Rect, Widget},
};

pub struct ButtonWidget {
    pub(crate) id: usize,
    pub(crate) bounds: Rect,
    pub(crate) text: String,
    pub(crate) font: Option<FontId>,
    pub(crate) color: [f32; 4],
    pub(crate) hover_color: Option<[f32; 4]>,
    pub(crate) press_color: Option<[f32; 4]>,
    pub(crate) auto_size: bool,

    pub hovered: bool,
    pub pressed: bool,
    pub just_hovered: bool,
    pub just_unhovered: bool,
    pub just_clicked: bool,
    pub just_pressed: bool,
    pub right_clicked: bool,
}

impl ButtonWidget {
    pub fn new(id: usize, text: &str) -> Self {
        Self {
            id,
            text: text.to_string(),
            font: None,
            bounds: Rect { x: 0.0, y: 0.0, w: 100.0, h: 40.0 },
            color: [0.0; 4],
            hover_color: None,
            press_color: None,
            auto_size: false,
            hovered: false,
            pressed: false,
            just_hovered: false,
            just_unhovered: false,
            just_clicked: false,
            just_pressed: false,
            right_clicked: false,
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

    pub fn color(&mut self, color: [f32; 4]) -> &mut Self {
        self.color = color;
        self
    }

    pub fn text(&mut self, text: impl Into<String>) -> &mut Self {
        self.text = text.into();
        self
    }
}

impl Widget for ButtonWidget {
    fn id(&self) -> usize { self.id }
    fn bounds(&self) -> Rect { self.bounds }
    fn set_bounds(&mut self, bounds: Rect) { self.bounds = bounds; }

    fn update(&mut self, mouse: &MouseState) {
        let over = self.bounds.contains(mouse.x, mouse.y);

        self.just_hovered   = over && !self.hovered;
        self.just_unhovered = !over && self.hovered;
        self.hovered        = over;
        self.just_pressed   = over && mouse.left_just_pressed;
        self.just_clicked   = over && mouse.left_just_released && self.pressed;
        self.right_clicked  = over && mouse.right_just_released;

        if self.just_pressed {
            self.pressed = true;
        } else if mouse.left_just_released {
            self.pressed = false;
        }
    }

    fn render(&mut self, ui: &mut Ui) {
        let font_id = self.font.expect(
            "ButtonWidget has no font — call .font(font_id) before rendering"
        );

        let padding = ui.fonts.default_padding;
        let (text_w, text_h) = ui.fonts.measure(&self.text, font_id);

        if self.auto_size {
            self.bounds.w = text_w + padding * 2.0;
            self.bounds.h = text_h + padding * 2.0;
        }

        let color = if self.pressed {
            self.press_color.unwrap_or_else(|| darken(self.color, 0.7))
        } else if self.hovered {
            self.hover_color.unwrap_or_else(|| lighten(self.color, 1.2))
        } else {
            self.color
        };

        ui.rect(self.bounds.x, self.bounds.y, self.bounds.w, self.bounds.h, color, [0.0; 4], 0.0);

        let text_x = self.bounds.x + (self.bounds.w - text_w) / 2.0;
        let text_y = self.bounds.y + (self.bounds.h - text_h) / 2.0;
        ui.text(&self.text, font_id, text_x, text_y);
    }

    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}

#[inline]
fn darken(color: [f32; 4], factor: f32) -> [f32; 4] {
    [color[0] * factor, color[1] * factor, color[2] * factor, color[3]]
}

#[inline]
fn lighten(color: [f32; 4], factor: f32) -> [f32; 4] {
    [
        (color[0] * factor).min(1.0),
        (color[1] * factor).min(1.0),
        (color[2] * factor).min(1.0),
        color[3],
    ]
}
