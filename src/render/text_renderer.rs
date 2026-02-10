use glyphon::{
    SwashCache, TextAtlas, TextRenderer as GlyphonRenderer, Cache, Viewport,
    Attrs, Family, Shaping, Buffer, Metrics, TextArea, TextBounds, FontSystem,
    Resolution, Color,
};
use wgpu;

struct TextEntry {
    buffer: Buffer,
    x: f32,
    y: f32,
    scale: f32,
    text: String,
    family: String,
    size: f32,
}

pub struct TextRenderer {
    cache: Cache,
    swash_cache: SwashCache,
    pub atlas: TextAtlas,
    viewport: Viewport,
    renderer: GlyphonRenderer,
    entries: Vec<TextEntry>,
    active: usize,
    screen_width: f32,
    screen_height: f32,
    scale_factor: f64,
}

impl TextRenderer {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, format: wgpu::TextureFormat) -> Self {
        let cache = Cache::new(device);
        let swash_cache = SwashCache::new();
        let mut atlas = TextAtlas::new(device, queue, &cache, format);
        let renderer = GlyphonRenderer::new(
            &mut atlas,
            device,
            wgpu::MultisampleState { count: 4, mask: !0, alpha_to_coverage_enabled: false },
            None,
        );
        let viewport = Viewport::new(device, &cache);

        Self {
            cache,
            swash_cache,
            atlas,
            viewport,
            renderer,
            entries: Vec::new(),
            active: 0,
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
        family: String,
        size: f32,
        text: &str,
        x: f32,
        y: f32,
    ) {
        let scale = self.scale_factor as f32;
        let line_height = size * 1.4;
        let idx = self.active;
        self.active += 1;

        if idx < self.entries.len() {
            let entry = &mut self.entries[idx];
            entry.x = x;
            entry.y = y;
            entry.scale = scale;

            let content_changed = entry.text != text || entry.family != family || entry.size != size;
            if content_changed {
                entry.text = text.to_string();
                entry.family = family.clone();
                entry.size = size;
                entry.buffer.set_metrics(font_system, Metrics::new(size * scale, line_height * scale));
                entry.buffer.set_size(font_system, Some(self.screen_width - x), Some(self.screen_height - y));
                entry.buffer.set_text(
                    font_system,
                    text,
                    &Attrs::new().family(Family::Name(family.as_str())),
                    Shaping::Advanced,
                );
                entry.buffer.shape_until_scroll(font_system, false);
            }
        } else {
            let mut buffer = Buffer::new(font_system, Metrics::new(size * scale, line_height * scale));
            buffer.set_size(font_system, Some(self.screen_width - x), Some(self.screen_height - y));
            buffer.set_text(
                font_system,
                text,
                &Attrs::new().family(Family::Name(family.as_str())),
                Shaping::Advanced,
            );
            buffer.shape_until_scroll(font_system, false);
            self.entries.push(TextEntry {
                buffer, x, y, scale,
                text: text.to_string(),
                family,
                size,
            });
        }
    }

    pub fn render<'pass>(
        &'pass mut self,
        font_system: &mut FontSystem,
        screen_width: f32,
        screen_height: f32,
        scale_factor: f64,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        pass: &mut wgpu::RenderPass<'pass>,
    ) {
        let physical_width = (screen_width * scale_factor as f32) as u32;
        let physical_height = (screen_height * scale_factor as f32) as u32;

        self.viewport.update(queue, Resolution { width: physical_width, height: physical_height });

        if self.active == 0 {
            return;
        }

        let text_areas: Vec<TextArea> = self.entries[..self.active]
            .iter()
            .map(|entry| TextArea {
                buffer: &entry.buffer,
                left: entry.x * entry.scale,
                top: entry.y * entry.scale,
                scale: 1.0,
                bounds: TextBounds {
                    left: 0,
                    top: 0,
                    right: physical_width as i32,
                    bottom: physical_height as i32,
                },
                default_color: Color::rgb(255, 255, 255),
                custom_glyphs: &[],
            })
            .collect();

        self.renderer
            .prepare(
                device,
                queue,
                font_system,
                &mut self.atlas,
                &self.viewport,
                text_areas,
                &mut self.swash_cache,
            )
            .unwrap();

        self.renderer.render(&self.atlas, &self.viewport, pass).unwrap();
    }

    pub fn trim_atlas(&mut self) {
        self.atlas.trim();
    }

    pub fn clear(&mut self) {
        self.active = 0;
    }
}
