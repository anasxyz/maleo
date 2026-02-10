use std::collections::HashMap;
use glyphon::{Attrs, Buffer, Family, FontSystem, Metrics, Shaping};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FontId(usize);

pub(crate) struct FontEntry {
    pub family: String,
    pub size: f32,
}

pub struct Fonts {
    pub(crate) font_system: FontSystem,
    entries: HashMap<FontId, FontEntry>,
    next_id: usize,
    /// padding applied on each side when a widget auto sizes to its text
    pub default_padding: f32,
}

impl Fonts {
    pub fn new() -> Self {
        Self {
            font_system: FontSystem::new(),
            entries: HashMap::new(),
            next_id: 0,
            default_padding: 12.0,
        }
    }

    pub fn add(&mut self, family: &str, size: f32) -> FontId {
        let id = FontId(self.next_id);
        self.next_id += 1;
        self.entries.insert(id, FontEntry {
            family: family.to_string(),
            size,
        });
        id
    }

    pub fn measure(&mut self, text: &str, font_id: FontId) -> (f32, f32) {
        let entry = self.entries.get(&font_id)
            .expect("FontId not found — was it created from a different Fonts instance?");

        let family = entry.family.clone();
        let size = entry.size;
        let line_height = size * 1.4;

        let mut buffer = Buffer::new(
            &mut self.font_system,
            Metrics::new(size, line_height),
        );

        buffer.set_size(&mut self.font_system, f32::MAX, f32::MAX);
        buffer.set_text(
            &mut self.font_system,
            text,
            Attrs::new().family(Family::Name(family.as_str())),
            Shaping::Advanced,
        );
        buffer.shape_until_scroll(&mut self.font_system);

        let width = buffer
            .layout_runs()
            .next()
            .map(|run| run.line_w)
            .unwrap_or(0.0);

        (width, line_height)
    }

    pub(crate) fn get(&self, font_id: FontId) -> &FontEntry {
        self.entries.get(&font_id)
            .expect("FontId not found — was it created from a different Fonts instance?")
    }
}

impl Default for Fonts {
    fn default() -> Self {
        Self::new()
    }
}
