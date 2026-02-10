use glyphon::{
    Attrs, Buffer, Family, FontSystem, Metrics, Resolution, Shaping, SwashCache,
    TextArea, TextAtlas, TextRenderer as GlyphonRenderer,
};
use wgpu;

use crate::fonts::FontEntry;

pub struct TextRenderer {
    swash_cache: SwashCache,
    atlas: TextAtlas,
    renderer: GlyphonRenderer,
    text_buffers: Vec<(Buffer, f32, f32, f32)>, // buffer, x, y, scale
    screen_width: f32,
    screen_height: f32,
    scale_factor: f64,
}

impl TextRenderer {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
    ) -> Self {
        let swash_cache = SwashCache::new();
        let mut atlas = TextAtlas::new(device, queue, format);
        let renderer = GlyphonRenderer::new(
            &mut atlas,
            device,
            wgpu::MultisampleState {
                count: 4,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            None,
        );

        Self {
            swash_cache,
            atlas,
            renderer,
            text_buffers: Vec::new(),
            screen_width: 800.0,
            screen_height: 600.0,
            scale_factor: 1.0,
        }
    }

    pub fn resize(&mut self, width: f32, height: f32, scale_factor: f64) {
        self.screen_width = width;
        self.screen_height = height;
        self.scale_factor = scale_factor;
    }

    pub fn draw(
        &mut self,
        font_system: &mut FontSystem,
        font_entry: &FontEntry,
        text: &str,
        x: f32,
        y: f32,
    ) {
        let scale = self.scale_factor as f32;
        let line_height = font_entry.size * 1.4;

        let mut buffer = Buffer::new(
            font_system,
            Metrics::new(font_entry.size * scale, line_height * scale),
        );

        buffer.set_size(font_system, self.screen_width - x, self.screen_height - y);
        buffer.set_text(
            font_system,
            text,
            Attrs::new().family(Family::Name(font_entry.family.as_str())),
            Shaping::Advanced,
        );
        buffer.shape_until_scroll(font_system);

        self.text_buffers.push((buffer, x, y, scale));
    }

    pub fn render<'pass>(
        &'pass mut self,
        font_system: &'pass mut FontSystem,
        screen_width: f32,
        screen_height: f32,
        scale_factor: f64,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        pass: &mut wgpu::RenderPass<'pass>,
    ) {
        if self.text_buffers.is_empty() {
            return;
        }

        let physical_width = (screen_width * scale_factor as f32) as u32;
        let physical_height = (screen_height * scale_factor as f32) as u32;

        let text_areas: Vec<TextArea> = self
            .text_buffers
            .iter()
            .map(|(buffer, x, y, stored_scale)| TextArea {
                buffer,
                left: x * stored_scale,
                top: y * stored_scale,
                scale: 1.0,
                bounds: glyphon::TextBounds {
                    left: 0,
                    top: 0,
                    right: physical_width as i32,
                    bottom: physical_height as i32,
                },
                default_color: glyphon::Color::rgb(255, 255, 255),
            })
            .collect();

        self.renderer
            .prepare(
                device,
                queue,
                font_system,
                &mut self.atlas,
                Resolution {
                    width: physical_width,
                    height: physical_height,
                },
                text_areas,
                &mut self.swash_cache,
            )
            .unwrap();

        self.renderer.render(&self.atlas, pass).unwrap();
    }

    pub fn clear(&mut self) {
        self.text_buffers.clear();
    }
}
