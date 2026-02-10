use std::any::Any;
use crate::{
    FontId, MouseState, Drawer,
    widgets::{Rect, Widget},
};

pub struct SliderWidget {
    pub(crate) id: usize,
    pub(crate) bounds: Rect,
    pub(crate) track_color: [f32; 4],
    pub(crate) fill_color: [f32; 4],
    pub(crate) thumb_color: [f32; 4],
    pub(crate) min: f32,
    pub(crate) max: f32,
    pub(crate) font: Option<FontId>,
    pub(crate) show_label: bool,

    pub value: f32,
    pub hovered: bool,
    pub dragging: bool,
    pub just_changed: bool, 
}

impl SliderWidget {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            bounds: Rect { x: 0.0, y: 0.0, w: 200.0, h: 20.0 },
            track_color: [0.3, 0.3, 0.3, 1.0],
            fill_color: [0.2, 0.5, 1.0, 1.0],
            thumb_color: [1.0, 1.0, 1.0, 1.0],
            min: 0.0,
            max: 1.0,
            font: None,
            show_label: false,
            value: 0.0,
            hovered: false,
            dragging: false,
            just_changed: false,
        }
    }

    pub fn position(&mut self, x: f32, y: f32) -> &mut Self {
        self.bounds.x = x;
        self.bounds.y = y;
        self
    }

    pub fn size(&mut self, w: f32, h: f32) -> &mut Self {
        self.bounds.w = w;
        self.bounds.h = h;
        self
    }

    pub fn min(&mut self, min: f32) -> &mut Self {
        self.min = min;
        self
    }

    pub fn max(&mut self, max: f32) -> &mut Self {
        self.max = max;
        self
    }

    pub fn range(&mut self, min: f32, max: f32) -> &mut Self {
        self.min = min;
        self.max = max;
        self
    }

    pub fn value(&mut self, value: f32) -> &mut Self {
        self.value = value.clamp(self.min, self.max);
        self
    }

    pub fn track_color(&mut self, color: [f32; 4]) -> &mut Self {
        self.track_color = color;
        self
    }

    pub fn fill_color(&mut self, color: [f32; 4]) -> &mut Self {
        self.fill_color = color;
        self
    }

    pub fn thumb_color(&mut self, color: [f32; 4]) -> &mut Self {
        self.thumb_color = color;
        self
    }

    pub fn show_label(&mut self, font: FontId) -> &mut Self {
        self.font = Some(font);
        self.show_label = true;
        self
    }

    fn thumb_x(&self) -> f32 {
        let t = (self.value - self.min) / (self.max - self.min);
        let thumb_r = self.bounds.h;
        self.bounds.x + thumb_r + t * (self.bounds.w - thumb_r * 2.0)
    }

    fn value_from_mouse_x(&self, mouse_x: f32) -> f32 {
        let thumb_r = self.bounds.h;
        let track_start = self.bounds.x + thumb_r;
        let track_end = self.bounds.x + self.bounds.w - thumb_r;
        let t = ((mouse_x - track_start) / (track_end - track_start)).clamp(0.0, 1.0);
        self.min + t * (self.max - self.min)
    }
}

impl Widget for SliderWidget {
    fn id(&self) -> usize { self.id }
    fn bounds(&self) -> Rect { self.bounds }

    fn update(&mut self, mouse: &MouseState) {
        let over = self.bounds.contains(mouse.x, mouse.y);
        self.hovered = over;

        if over && mouse.left_just_pressed {
            self.dragging = true;
        }
        if mouse.left_just_released {
            self.dragging = false;
        }

        if self.dragging {
            let new_value = self.value_from_mouse_x(mouse.x);
            self.just_changed = new_value != self.value;
            self.value = new_value;
        } else {
            self.just_changed = false;
        }
    }

    fn render(&mut self, drawer: &mut Drawer) {
        let x = self.bounds.x;
        let y = self.bounds.y;
        let w = self.bounds.w;
        let h = self.bounds.h;
        let thumb_r = h;
        let track_h = h * 0.3;
        let track_y = y + (h - track_h) / 2.0;

        drawer.rounded_rect(
            x + thumb_r, track_y,
            w - thumb_r * 2.0, track_h,
            track_h / 2.0,
            self.track_color, [0.0; 4], 0.0,
        );

        let t = (self.value - self.min) / (self.max - self.min);
        let fill_w = (w - thumb_r * 2.0) * t;
        if fill_w > 0.0 {
            drawer.rounded_rect(
                x + thumb_r, track_y,
                fill_w, track_h,
                track_h / 2.0,
                self.fill_color, [0.0; 4], 0.0,
            );
        }

        let thumb_x = self.thumb_x();
        let thumb_cy = y + h / 2.0;
        let thumb_radius = h * 0.5;
        let thumb_color = if self.dragging {
            darken(self.thumb_color, 0.8)
        } else if self.hovered {
            lighten(self.thumb_color, 1.1)
        } else {
            self.thumb_color
        };
        drawer.circle(thumb_x, thumb_cy, thumb_radius, thumb_color, [0.0; 4], 0.0);

        // optional value label
        if self.show_label {
            if let Some(font_id) = self.font {
                let label = format!("{:.2}", self.value);
                let (text_w, text_h) = drawer.fonts.measure(&label, font_id);
                let label_x = thumb_x - text_w / 2.0;
                let label_y = y - text_h - 4.0;
                drawer.text(&label, font_id, label_x, label_y);
            }
        }
    }

    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}

#[inline]
fn darken(color: [f32; 4], factor: f32) -> [f32; 4] {
    [color[0] * factor, color[1] * factor, color[2] * factor, color[3]]
}

#[inline]
fn lighten(color: [f32; 4], factor: f32) -> [f32; 4] {
    [
        (color[0] * factor).min(1.0),
        (color[1] * factor).min(1.0),
        (color[2] * factor).min(1.0),
        color[3],
    ]
}
