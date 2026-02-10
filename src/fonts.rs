use std::collections::HashMap;
use glyphon::{FontSystem, Attrs, Family, Shaping, Buffer, Metrics};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FontId(usize);

pub struct FontEntry {
    pub family: String,
    pub size: f32,
}

pub struct Fonts {
    pub(crate) font_system: FontSystem,
    entries: Vec<FontEntry>,
    measure_cache: HashMap<(usize, String), (f32, f32)>,
    pub default_padding: f32,
}

impl Fonts {
    pub fn new() -> Self {
        Self {
            font_system: FontSystem::new(),
            entries: Vec::new(),
            measure_cache: HashMap::new(),
            default_padding: 8.0,
        }
    }

    pub fn add(&mut self, family: &str, size: f32) -> FontId {
        let id = FontId(self.entries.len());
        self.entries.push(FontEntry {
            family: family.to_string(),
            size,
        });
        id
    }

    pub fn get(&self, id: FontId) -> &FontEntry {
        &self.entries[id.0]
    }

    pub fn measure(&mut self, text: &str, id: FontId) -> (f32, f32) {
        let key = (id.0, text.to_string());
        if let Some(&cached) = self.measure_cache.get(&key) {
            return cached;
        }

        let family = self.entries[id.0].family.clone();
        let size = self.entries[id.0].size;
        let line_height = size * 1.4;

        let mut buffer = Buffer::new(
            &mut self.font_system,
            Metrics::new(size, line_height),
        );
        buffer.set_size(&mut self.font_system, None, None);
        buffer.set_text(
            &mut self.font_system,
            text,
            &Attrs::new().family(Family::Name(family.as_str())),
            Shaping::Advanced,
        );
        buffer.shape_until_scroll(&mut self.font_system, false);

        let mut width: f32 = 0.0;
        let mut height: f32 = 0.0;
        for run in buffer.layout_runs() {
            width = width.max(run.line_w);
            height += line_height;
        }

        let result = (width, height);
        self.measure_cache.insert(key, result);
        result
    }
}
