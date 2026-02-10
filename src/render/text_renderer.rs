use glyphon::{
    SwashCache, TextAtlas, TextRenderer as GlyphonRenderer,
    Attrs, Family, Shaping, Buffer, Metrics, TextArea, Resolution, FontSystem,
};
use wgpu;

struct TextEntry {
    buffer: Buffer,
    x: f32,
    y: f32,
    scale: f32,
    // track what's currently in the buffer so we only re-shape on change
    text: String,
    family: String,
    size: f32,
}

pub struct TextRenderer {
    swash_cache: SwashCache,
    atlas: TextAtlas,
    renderer: GlyphonRenderer,
    // persistent pool — reused across frames
    entries: Vec<TextEntry>,
    // how many entries are active this frame
    active: usize,
    screen_width: f32,
    screen_height: f32,
    scale_factor: f64,
}

impl TextRenderer {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, format: wgpu::TextureFormat) -> Self {
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
            // reuse existing entry — only re-shape if content changed
            let entry = &mut self.entries[idx];
            entry.x = x;
            entry.y = y;
            entry.scale = scale;

            let content_changed = entry.text != text
                || entry.family != family
                || entry.size != size;

            if content_changed {
                entry.text = text.to_string();
                entry.family = family.clone();
                entry.size = size;
                entry.buffer.set_metrics(
                    font_system,
                    Metrics::new(size * scale, line_height * scale),
                );
                entry.buffer.set_size(font_system, self.screen_width - x, self.screen_height - y);
                entry.buffer.set_text(
                    font_system,
                    text,
                    Attrs::new().family(Family::Name(family.as_str())),
                    Shaping::Advanced,
                );
                entry.buffer.shape_until_scroll(font_system);
            }
        } else {
            // pool exhausted — allocate a new entry
            let mut buffer = Buffer::new(
                font_system,
                Metrics::new(size * scale, line_height * scale),
            );
            buffer.set_size(font_system, self.screen_width - x, self.screen_height - y);
            buffer.set_text(
                font_system,
                text,
                Attrs::new().family(Family::Name(family.as_str())),
                Shaping::Advanced,
            );
            buffer.shape_until_scroll(font_system);

            self.entries.push(TextEntry {
                buffer,
                x,
                y,
                scale,
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
        if self.active == 0 {
            return;
        }

        let physical_width = (screen_width * scale_factor as f32) as u32;
        let physical_height = (screen_height * scale_factor as f32) as u32;

        let text_areas: Vec<TextArea> = self.entries[..self.active]
            .iter()
            .map(|entry| TextArea {
                buffer: &entry.buffer,
                left: entry.x * entry.scale,
                top: entry.y * entry.scale,
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
                Resolution { width: physical_width, height: physical_height },
                text_areas,
                &mut self.swash_cache,
            )
            .unwrap();

        self.renderer.render(&self.atlas, pass).unwrap();
    }

    /// Trim the glyph atlas — call this AFTER the render pass has ended.
    /// Evicts glyphs not used in the last frame, keeping memory flat.
    pub fn trim_atlas(&mut self) {
        self.atlas.trim();
        println!("trim called, entries: {}, active: {}", self.entries.len(), self.active);
    }

    /// Reset active count — entries stay allocated in the pool.
    pub fn clear(&mut self) {
        self.active = 0;
    }
}
