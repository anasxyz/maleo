// src/widgets.rs

use crate::{ShapeRenderer, TextRenderer};

pub struct WidgetRenderer;

impl WidgetRenderer {
    pub fn new() -> Self {
        Self
    }

    /// Draw a button with properly centered text
    pub fn button(
        &mut self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        label: &str,
        shapes: &mut ShapeRenderer,
        text: &mut TextRenderer,
    ) {
        // Draw background
        shapes.rounded_rect(x, y, w, h, 8.0, [0.2, 0.4, 0.8, 1.0]);
        
        // Measure and center text
        let base_font_size = 22.0;
        let available_width = w - 5.0;
        let available_height = h - 10.0;
        
        let (text_width, _) = text.measure_text(label, base_font_size);
        
        let scale_w = if text_width > available_width { 
            available_width / text_width 
        } else { 
            1.0 
        };
        let scale_h = if base_font_size > available_height { 
            available_height / base_font_size 
        } else { 
            1.0 
        };
        let font_size = base_font_size * scale_w.min(scale_h);
        
        let (final_w, _) = text.measure_text(label, font_size);
        
        // Center using font size, not measured height
        let text_x = x + (w - final_w) / 2.0;
        let text_y = y + h / 2.0 - font_size * 0.69;
        
        text.draw_sized(label, text_x, text_y, font_size);
    }
}
