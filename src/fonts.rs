use glyphon::{Attrs, Buffer, Family, FontSystem, Metrics, Shaping};
use std::collections::HashMap;

pub enum Font {
    Name(String),
    Default,
}

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
    measure_cache_sized: HashMap<(usize, String, u32), (f32, f32)>,
    name_to_id: HashMap<String, FontId>,
    pub default_padding: f32,
    pub default: Option<FontId>,
}

impl Fonts {
    pub fn new() -> Self {
        Self {
            font_system: FontSystem::new(),
            entries: Vec::new(),
            measure_cache: HashMap::new(),
            measure_cache_sized: HashMap::new(),
            name_to_id: HashMap::new(),
            default_padding: 8.0,
            default: None,
        }
    }

    pub fn add(&mut self, name: &str, family: &str, size: f32) -> FontId {
        if let Some(&id) = self.name_to_id.get(name) {
            return id;
        }

        let id = FontId(self.entries.len());
        self.entries.push(FontEntry {
            family: family.to_string(),
            size,
        });
        self.name_to_id.insert(name.to_string(), id);
        id
    }

    pub fn get(&self, id: FontId) -> &FontEntry {
        &self.entries[id.0]
    }

    pub fn get_by_name(&self, name: &str) -> Option<FontId> {
        self.name_to_id.get(name).copied()
    }

    pub fn default(&self) -> Option<FontId> {
        self.default
    }

    pub fn set_default(&mut self, id: FontId) {
        self.default = Some(id);
    }

    // displays all fonts info, id, name, family, size
    pub fn display_all_fonts(&self) {}

    pub fn measure(&mut self, text: &str, id: FontId) -> (f32, f32) {
        let size = self.entries[id.0].size;
        self.measure_sized(text, id, size)
    }

    pub fn measure_sized(&mut self, text: &str, id: FontId, size: f32) -> (f32, f32) {
        let key = (id.0, text.to_string(), (size * 10.0) as u32);
        if let Some(&cached) = self.measure_cache_sized.get(&key) {
            return cached;
        }

        let family = self.entries[id.0].family.clone();
        let line_height = size * 1.4;

        let mut buffer = Buffer::new(&mut self.font_system, Metrics::new(size, line_height));
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
        self.measure_cache_sized.insert(key, result);
        result
    }
}
