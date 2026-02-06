use glyphon::{
    FontSystem, SwashCache, TextAtlas, TextRenderer as GlyphonRenderer,
    Attrs, Family, Shaping, Buffer, Metrics, TextArea, Resolution,
};
use wgpu;

pub struct TextRenderer {
    font_system: FontSystem,
    swash_cache: SwashCache,
    atlas: TextAtlas,
    renderer: GlyphonRenderer,
    text_buffers: Vec<(Buffer, f32, f32, f32)>, // buffer, x, y, scale_factor
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
        let font_system = FontSystem::new();
        
        let swash_cache = SwashCache::new();
        let mut atlas = TextAtlas::new(device, queue, format);
        let renderer = GlyphonRenderer::new(
            &mut atlas,
            device,
            wgpu::MultisampleState {
                count: 4, // enable 4x MSAA to match the app
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            None,
        );

        Self {
            font_system,
            swash_cache,
            atlas,
            renderer,
            text_buffers: Vec::new(),
            screen_width: 800.0,
            screen_height: 600.0,
            scale_factor: 1.0,
        }
    }

    /// update screen dimensions and scale factor
    pub fn resize(&mut self, width: f32, height: f32, scale_factor: f64) {
        self.screen_width = width;
        self.screen_height = height;
        self.scale_factor = scale_factor;
    }

    /// just draw text at x, y
    pub fn draw(&mut self, text: &str, font_size: f32, x: f32, y: f32) {
        self.queue_text(text, x, y, font_size, self.screen_width, self.screen_height, self.scale_factor);
    }

    /// queue text to be drawn, doesnt render yet
    pub fn queue_text(
        &mut self,
        text: &str,
        x: f32,
        y: f32,
        font_size: f32,
        screen_width: f32,
        screen_height: f32,
        scale_factor: f64,
    ) {
        let scale = scale_factor as f32;
        
        // scale font metrics by dpi for consistent visual size
        let mut buffer = Buffer::new(
            &mut self.font_system,
            Metrics::new(font_size * scale, 35.0 * scale),
        );

        // set buffer size in logical coordinates
        buffer.set_size(&mut self.font_system, screen_width - x * 2.0, screen_height - y * 2.0);
        
        // set text with proper wrapping
        buffer.set_text(
            &mut self.font_system,
            text,
            Attrs::new().family(Family::Name("JetBrainsMono Nerd Font")),
            Shaping::Advanced,
        );
        
        // shape the lines so glyphon knows where line breaks are
        buffer.shape_until_scroll(&mut self.font_system);

        // store with scale factor for rendering
        self.text_buffers.push((buffer, x, y, scale));
    }

    /// render all queued text
    pub fn render<'pass>(
        &'pass mut self,
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

        // calculate physical resolution for crisp rendering
        let physical_width = (screen_width * scale_factor as f32) as u32;
        let physical_height = (screen_height * scale_factor as f32) as u32;

        // convert logical coordinates to physical for positioning
        let text_areas: Vec<TextArea> = self.text_buffers
            .iter()
            .map(|(buffer, x, y, stored_scale)| TextArea {
                buffer,
                left: x * stored_scale, // convert to physical coordinates
                top: y * stored_scale,  // convert to physical coordinates
                scale: 1.0,
                bounds: glyphon::TextBounds {
                    left: 0,
                    top: 0,
                    right: physical_width as i32,  // physical bounds
                    bottom: physical_height as i32, // physical bounds
                },
                default_color: glyphon::Color::rgb(255, 255, 255),
            })
            .collect();

        // prepare for rendering with physical resolution 
        self.renderer
            .prepare(
                device,
                queue,
                &mut self.font_system,
                &mut self.atlas,
                Resolution {
                    width: physical_width,
                    height: physical_height,
                },
                text_areas,
                &mut self.swash_cache,
            )
            .unwrap();

        // render all text
        self.renderer.render(&self.atlas, pass).unwrap();
    }

    /// clear all queued text
    pub fn clear(&mut self) {
        self.text_buffers.clear();
    }
}
