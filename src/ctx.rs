use crate::{Color, Fonts, InputState, MouseState, ShapeRenderer, TextRenderer};

pub struct Ctx<'a> {
    pub mouse: &'a MouseState,
    pub input: &'a InputState,
    pub width: f32,
    pub height: f32,

    fonts: &'a mut Fonts,
    text: &'a mut TextRenderer,
    shape: &'a mut ShapeRenderer,
}

impl<'a> Ctx<'a> {
    pub(crate) fn new(
        mouse: &'a MouseState,
        input: &'a InputState,
        fonts: &'a mut Fonts,
        text: &'a mut TextRenderer,
        shape: &'a mut ShapeRenderer,
        width: f32,
        height: f32,
    ) -> Self {
        Self { mouse, input, fonts, text, shape, width, height }
    }

    pub fn draw_text(&mut self, text: &str, x: f32, y: f32, color: Color) {
        let font = self.fonts.default();
        let entry = self.fonts.get(font);
        let family = entry.family.clone();
        let size = entry.size;
        self.text.draw(&mut self.fonts.font_system, family, size, text, x, y, color);
    }

    pub fn draw_text_with_font(&mut self, text: &str, x: f32, y: f32, color: Color, font_name: &str) {
        let font = self.fonts.get_by_name(font_name).unwrap_or(self.fonts.default());
        let entry = self.fonts.get(font);
        let family = entry.family.clone();
        let size = entry.size;
        self.text.draw(&mut self.fonts.font_system, family, size, text, x, y, color);
    }

    pub fn draw_rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        self.shape.draw_rect(x, y, w, h, color.to_array(), [0.0; 4], 0.0);
    }

    pub fn draw_rect_outlined(&mut self, x: f32, y: f32, w: f32, h: f32, color: Color, outline: Color, thickness: f32) {
        self.shape.draw_rect(x, y, w, h, color.to_array(), outline.to_array(), thickness);
    }

    pub fn draw_circle(&mut self, cx: f32, cy: f32, radius: f32, color: Color) {
        self.shape.draw_circle(cx, cy, radius, color.to_array(), [0.0; 4], 0.0);
    }

    pub fn draw_circle_outlined(&mut self, cx: f32, cy: f32, radius: f32, color: Color, outline: Color, thickness: f32) {
        self.shape.draw_circle(cx, cy, radius, color.to_array(), outline.to_array(), thickness);
    }

    pub fn draw_rounded_rect(&mut self, x: f32, y: f32, w: f32, h: f32, radius: f32, color: Color) {
        self.shape.draw_rounded_rect(x, y, w, h, radius, color.to_array(), [0.0; 4], 0.0);
    }

    pub fn draw_rounded_rect_outlined(&mut self, x: f32, y: f32, w: f32, h: f32, radius: f32, color: Color, outline: Color, thickness: f32) {
        self.shape.draw_rounded_rect(x, y, w, h, radius, color.to_array(), outline.to_array(), thickness);
    }
}
