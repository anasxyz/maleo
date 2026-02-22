use glyphon::{Attrs, Buffer, Family, FontSystem, Metrics, Shaping};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FontId(pub(crate) usize);

pub struct FontEntry {
    pub family: String,
    pub size: f32,
}

pub struct Fonts {
    pub(crate) font_system: FontSystem,
    entries: Vec<FontEntry>,
    measure_cache: HashMap<(usize, String, u32), (f32, f32)>,
    name_to_id: HashMap<String, FontId>,
    pub(crate) default: Option<FontId>,
}

// returned by add() so the user can chain .default()
pub struct FontBuilder<'a> {
    fonts: &'a mut Fonts,
    id: FontId,
}

impl<'a> FontBuilder<'a> {
    pub fn default(self) -> FontId {
        self.fonts.default = Some(self.id);
        self.id
    }
}

impl Fonts {
    pub fn new() -> Self {
        Self {
            font_system: FontSystem::new(),
            entries: Vec::new(),
            measure_cache: HashMap::new(),
            name_to_id: HashMap::new(),
            default: None,
        }
    }

    pub fn add(&mut self, name: &str, family: &str, size: f32) -> FontBuilder<'_> {
        let id = if let Some(&existing) = self.name_to_id.get(name) {
            existing
        } else {
            let id = FontId(self.entries.len());
            self.entries.push(FontEntry {
                family: family.to_string(),
                size,
            });
            self.name_to_id.insert(name.to_string(), id);
            id
        };
        FontBuilder { fonts: self, id }
    }

    pub fn get(&self, id: FontId) -> &FontEntry {
        &self.entries[id.0]
    }

    pub fn get_by_name(&self, name: &str) -> Option<FontId> {
        self.name_to_id.get(name).copied()
    }

    pub fn default_id(&self) -> Option<FontId> {
        self.default
    }

    // resolves a font name to an id, falling back to default
    pub fn resolve(&self, name: Option<&str>) -> Option<FontId> {
        match name {
            Some(n) => self.get_by_name(n).or(self.default),
            None => self.default,
        }
    }

    pub fn measure(&mut self, text: &str, id: FontId) -> (f32, f32) {
        let size = self.entries[id.0].size;
        self.measure_sized(text, id, size)
    }

    pub fn measure_sized(&mut self, text: &str, id: FontId, size: f32) -> (f32, f32) {
        let key = (id.0, text.to_string(), (size * 10.0) as u32);
        if let Some(&cached) = self.measure_cache.get(&key) {
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
        self.measure_cache.insert(key, result);
        result
    }
}
