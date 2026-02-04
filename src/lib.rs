// text-render/src/lib.rs

use glyphon::{
    Attrs, Buffer, Family, FontSystem, Metrics, Resolution, Shaping, SwashCache, TextArea,
    TextAtlas, TextRenderer as GlyphonRenderer,
};

pub struct TextRenderer {
    font_system: FontSystem,
    swash_cache: SwashCache,
    atlas: TextAtlas,
    renderer: GlyphonRenderer,
}

impl TextRenderer {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, format: wgpu::TextureFormat) -> Self {
        let font_system = FontSystem::new(); // Remove 'mut'
        let swash_cache = SwashCache::new();
        let mut atlas = TextAtlas::new(device, queue, format);
        let renderer =
            GlyphonRenderer::new(&mut atlas, device, wgpu::MultisampleState::default(), None);

        Self {
            font_system,
            swash_cache,
            atlas,
            renderer,
        }
    }

    /// Draw a single line of text at position (x, y)
    pub fn draw_text<'pass>(
        &'pass mut self,
        text: &str,
        x: f32,
        y: f32,
        screen_width: f32,
        screen_height: f32,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        pass: &mut wgpu::RenderPass<'pass>,
    ) {
        // Create a text buffer
        let mut buffer = Buffer::new(
            &mut self.font_system,
            Metrics::new(14.0, 20.0), // font_size, line_height
        );

        buffer.set_size(&mut self.font_system, screen_width, screen_height);
        buffer.set_text(
            &mut self.font_system,
            text,
            Attrs::new().family(Family::Monospace),
            Shaping::Advanced,
        );

        // Create text area
        let text_area = TextArea {
            buffer: &buffer,
            left: x,
            top: y,
            scale: 1.0,
            bounds: glyphon::TextBounds {
                left: 0,
                top: 0,
                right: screen_width as i32,
                bottom: screen_height as i32,
            },
            default_color: glyphon::Color::rgb(255, 255, 255),
        };

        // Prepare for rendering
        self.renderer
            .prepare(
                device,
                queue,
                &mut self.font_system,
                &mut self.atlas,
                Resolution {
                    width: screen_width as u32,
                    height: screen_height as u32,
                },
                [text_area],
                &mut self.swash_cache,
            )
            .unwrap();

        // Render
        self.renderer.render(&self.atlas, pass).unwrap();
    }
}
