// src/lib.rs

mod shapes;
mod app;

pub use shapes::ShapeRenderer;
pub use app::{App, Canvas};

use glyphon::{
    FontSystem, SwashCache, TextAtlas, TextRenderer as GlyphonRenderer,
    Attrs, Family, Shaping, Buffer, Metrics, TextArea, Resolution,
};

pub struct TextRenderer {
    font_system: FontSystem,
    swash_cache: SwashCache,
    atlas: TextAtlas,
    renderer: GlyphonRenderer,
}

impl TextRenderer {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
    ) -> Self {
        let mut font_system = FontSystem::new();
        
        let swash_cache = SwashCache::new();
        let mut atlas = TextAtlas::new(device, queue, format);
        let renderer = GlyphonRenderer::new(
            &mut atlas,
            device,
            wgpu::MultisampleState::default(),
            None,
        );

        Self {
            font_system,
            swash_cache,
            atlas,
            renderer,
        }
    }

    /// Draw text with multiple lines
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
            Metrics::new(22.0, 35.0), // font_size, line_height
        );

        // Set buffer size to enable proper text layout
        buffer.set_size(&mut self.font_system, screen_width - x * 2.0, screen_height - y * 2.0);
        
        // Set text with proper wrapping
        buffer.set_text(
            &mut self.font_system,
            text,
            Attrs::new().family(Family::Name("JetBrainsMono Nerd Font")),
            Shaping::Advanced,
        );
        
        // Important: shape the lines so glyphon knows where line breaks are
        buffer.shape_until_scroll(&mut self.font_system);

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
