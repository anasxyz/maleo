use crate::{FontId, Fonts, ShapeRenderer, TextRenderer};

pub struct Ui<'a> {
    pub text_renderer: &'a mut TextRenderer,
    pub shape_renderer: &'a mut ShapeRenderer,
    pub fonts: &'a mut Fonts,
}

impl<'a> Ui<'a> {
    pub fn new(
        text_renderer: &'a mut TextRenderer,
        shape_renderer: &'a mut ShapeRenderer,
        fonts: &'a mut Fonts,
    ) -> Self {
        Self {
            text_renderer,
            shape_renderer,
            fonts,
        }
    }

    pub fn rect(
        &mut self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        color: [f32; 4],
        outline_color: [f32; 4],
        outline_thickness: f32,
    ) {
        self.shape_renderer
            .rect(x, y, w, h, color, outline_color, outline_thickness);
    }

    pub fn circle(
        &mut self,
        cx: f32,
        cy: f32,
        radius: f32,
        color: [f32; 4],
        outline_color: [f32; 4],
        outline_thickness: f32,
    ) {
        self.shape_renderer
            .circle(cx, cy, radius, color, outline_color, outline_thickness);
    }

    pub fn rounded_rect(
        &mut self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        radius: f32,
        color: [f32; 4],
        outline_color: [f32; 4],
        outline_thickness: f32,
    ) {
        self.shape_renderer
            .rounded_rect(x, y, w, h, radius, color, outline_color, outline_thickness);
    }

    pub fn text(&mut self, text: &str, font_id: FontId, x: f32, y: f32) {
        let entry = self.fonts.get(font_id);
        let entry = unsafe { &*(entry as *const _) };
        self.text_renderer
            .draw(&mut self.fonts.font_system, entry, text, x, y);
    }
}
