use crate::{FontId, Fonts, ShapeRenderer, TextRenderer};

pub struct Drawer<'a> {
    pub(crate) text_renderer: &'a mut TextRenderer,
    pub(crate) shape_renderer: &'a mut ShapeRenderer,
    pub fonts: &'a mut Fonts,
}

impl<'a> Drawer<'a> {
    pub fn new(
        text_renderer: &'a mut TextRenderer,
        shape_renderer: &'a mut ShapeRenderer,
        fonts: &'a mut Fonts,
    ) -> Self {
        Self { text_renderer, shape_renderer, fonts }
    }

    pub fn rect(
        &mut self,
        x: f32, y: f32, w: f32, h: f32,
        color: [f32; 4],
        outline_color: [f32; 4],
        outline_thickness: f32,
    ) {
        self.shape_renderer.rect(x, y, w, h, color, outline_color, outline_thickness);
    }

    pub fn circle(
        &mut self,
        cx: f32, cy: f32, radius: f32,
        color: [f32; 4],
        outline_color: [f32; 4],
        outline_thickness: f32,
    ) {
        self.shape_renderer.circle(cx, cy, radius, color, outline_color, outline_thickness);
    }

    pub fn rounded_rect(
        &mut self,
        x: f32, y: f32, w: f32, h: f32, radius: f32,
        color: [f32; 4],
        outline_color: [f32; 4],
        outline_thickness: f32,
    ) {
        self.shape_renderer.rounded_rect(x, y, w, h, radius, color, outline_color, outline_thickness);
    }

    pub fn text(&mut self, text: &str, font_id: FontId, x: f32, y: f32) {
        // copy out what we need so the immutable borrow on self.fonts ends
        // before i mutably borrow self.fonts.font_system
        let entry = self.fonts.get(font_id);
        let family = entry.family.clone();
        let size = entry.size;
        self.text_renderer.draw(
            &mut self.fonts.font_system,
            family,
            size,
            text,
            x,
            y,
        );
    }
}
