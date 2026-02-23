use taffy::prelude::*;

use crate::draw::{DrawCtx, check_interactions, is_outside, with_opacity};
use crate::events::{Event, Key};
use crate::layout::{align_to_self, margin_to_rect_lpa, val_to_dimension};
use crate::state::StateStore;
use crate::{Align, Color, Edges, Fonts, Interactions, Layout, Margin, Style, TextAlign, Val};

// persisted state between frames
#[derive(Default)]
pub struct TextInputState {
    pub focused: bool,
    pub cursor: usize,
    pub scroll_offset: f32,
}

pub(crate) struct TextInputCallback<M>(pub Box<dyn Fn(String) -> M>);

pub struct TextInput<M: Clone + 'static> {
    pub id: String,
    pub placeholder: String,
    pub placeholder_color: Option<Color>,
    pub font: Option<String>,
    pub font_size: Option<f32>,
    pub font_weight: u16,
    pub value: Option<String>,
    pub layout: Layout,
    pub style: Style,
    pub interactions: Interactions<M>,
    pub on_change: Option<Box<dyn Fn(String) -> M>>,
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) w: f32,
    pub(crate) h: f32,
}

impl<M: Clone + 'static> TextInput<M> {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            placeholder: String::new(),
            placeholder_color: None,
            font: None,
            font_size: None,
            font_weight: 400,
            value: None,
            layout: Layout::default(),
            style: Style::default(),
            interactions: Interactions::default(),
            on_change: None,
            x: 0.0,
            y: 0.0,
            w: 0.0,
            h: 0.0,
        }
    }

    pub fn draw(&mut self, ctx: &mut DrawCtx<M>) {
        let (x, y, w, h) = (self.x, self.y, self.w, self.h);
        if is_outside(x, y, w, h, ctx.clip) {
            return;
        }

        let value_str = self.value.as_deref().unwrap_or("");
        cache_value(ctx.state, &self.id, value_str);
        if let Some(cb) = self.on_change.take() {
            register_callback(ctx.state, &self.id, cb);
        }

        let hovered =
            ctx.mouse.x >= x && ctx.mouse.x <= x + w && ctx.mouse.y >= y && ctx.mouse.y <= y + h;

        if ctx.mouse.left_just_pressed {
            ctx.state
                .get_or_default_mut::<TextInputState>(&self.id)
                .focused = hovered;
        }
        {
            let s = ctx.state.get_or_default_mut::<TextInputState>(&self.id);
            s.cursor = s.cursor.min(value_str.len());
        }
        let focused = ctx.state.get_or_default::<TextInputState>(&self.id).focused;

        let default_bg = if focused {
            Color::new(0.18, 0.18, 0.22, 1.0)
        } else if hovered {
            Color::new(0.16, 0.16, 0.20, 1.0)
        } else {
            Color::new(0.13, 0.13, 0.17, 1.0)
        };
        let bg = self.style.background.unwrap_or(default_bg);

        let (border_col, border_w) = if let Some(col) = self.style.border_color {
            (col, self.style.border_thickness)
        } else {
            let default_col = if focused {
                Color::new(0.4, 0.5, 0.9, 1.0)
            } else {
                Color::new(0.3, 0.3, 0.35, 1.0)
            };
            (default_col, 1.5)
        };

        ctx.sr.draw_rounded_rect(
            x,
            y,
            w,
            h,
            self.style.border_radius,
            with_opacity(bg.to_array(), self.style.opacity),
            if border_w > 0.0 {
                with_opacity(border_col.to_array(), self.style.opacity)
            } else {
                [0.0; 4]
            },
            border_w,
        );

        let font_id = self
            .font
            .as_deref()
            .and_then(|name| ctx.fonts.resolve(Some(name)))
            .unwrap_or_else(|| ctx.fonts.default_id().unwrap());
        let family = ctx.fonts.get(font_id).family.clone();
        let size = self.font_size.unwrap_or(ctx.fonts.get(font_id).size);
        let pad_l = if self.layout.padding.left > 0.0 {
            self.layout.padding.left
        } else {
            8.0
        };
        let pad_r = if self.layout.padding.right > 0.0 {
            self.layout.padding.right
        } else {
            8.0
        };
        let (_, th) = ctx
            .fonts
            .measure_sized("M", font_id, size, self.font_weight);
        let ty = if self.layout.padding.top > 0.0 {
            y + self.layout.padding.top
        } else {
            y + (h - th) / 2.0
        };

        let text_area_w = w - pad_l - pad_r;
        let text_clip = Some([x + pad_l, y, x + w - pad_r, y + h]);

        let cursor_pos = ctx.state.get_or_default::<TextInputState>(&self.id).cursor;
        let (cursor_x_abs, _) =
            ctx.fonts
                .measure_sized(&value_str[..cursor_pos], font_id, size, self.font_weight);
        {
            let s = ctx.state.get_or_default_mut::<TextInputState>(&self.id);
            if cursor_x_abs - s.scroll_offset > text_area_w - 2.0 {
                s.scroll_offset = cursor_x_abs - text_area_w + 2.0;
            }
            if cursor_x_abs - s.scroll_offset < 0.0 {
                s.scroll_offset = cursor_x_abs;
            }
            if s.scroll_offset < 0.0 {
                s.scroll_offset = 0.0;
            }
        }
        let scroll = ctx
            .state
            .get_or_default::<TextInputState>(&self.id)
            .scroll_offset;

        if value_str.is_empty() {
            let col = self
                .placeholder_color
                .unwrap_or(Color::new(0.45, 0.45, 0.5, 1.0));
            ctx.tr.draw(
                &mut ctx.fonts.font_system,
                family,
                size,
                self.font_weight,
                false,
                TextAlign::Left,
                &self.placeholder,
                x + pad_l,
                ty,
                99999.0,
                text_clip,
                with_opacity(col.to_array(), self.style.opacity).into(),
            );
        } else {
            let col = self
                .style
                .text_color
                .unwrap_or(Color::new(0.92, 0.92, 0.95, 1.0));
            ctx.tr.draw(
                &mut ctx.fonts.font_system,
                family,
                size,
                self.font_weight,
                false,
                TextAlign::Left,
                value_str,
                x + pad_l - scroll,
                ty,
                99999.0,
                text_clip,
                with_opacity(col.to_array(), self.style.opacity).into(),
            );
        }

        if focused {
            let cursor_col = self
                .style
                .text_color
                .unwrap_or(Color::new(0.7, 0.75, 1.0, 1.0));
            // snap to nearest whole pixel so the cursor is always crisp.
            // floor() rather than round() keeps the cursor just after the
            // character it follows, which matches native text field behaviour
            let cursor_draw_x = (x + pad_l + cursor_x_abs - scroll).floor();
            let cursor_draw_y = ty.floor();
            let cursor_h = th.ceil();
            if cursor_draw_x >= x + pad_l && cursor_draw_x <= x + w - pad_r {
                ctx.sr.draw_rect(
                    cursor_draw_x,
                    cursor_draw_y,
                    1.0,
                    cursor_h,
                    with_opacity(cursor_col.to_array(), self.style.opacity),
                    [0.0; 4],
                    0.0,
                );
            }
        }

        check_interactions(&self.interactions, x, y, w, h, ctx);
    }

    pub fn layout_node(&self, taffy: &mut TaffyTree<()>, fonts: &mut Fonts) -> NodeId {
        let font_id = self
            .font
            .as_deref()
            .and_then(|name| fonts.resolve(Some(name)))
            .unwrap_or_else(|| fonts.default_id().unwrap());
        let size = self.font_size.unwrap_or(fonts.get(font_id).size);
        let (_, th) = fonts.measure_sized("M", font_id, size, 400);
        let pad_v = if self.layout.padding.top > 0.0 {
            self.layout.padding.top + self.layout.padding.bottom
        } else {
            16.0
        };
        let natural_h = th + pad_v;
        taffy
            .new_leaf(taffy::Style {
                size: taffy::geometry::Size {
                    width: match &self.layout.width {
                        Val::Auto => Dimension::Length(200.0),
                        other => val_to_dimension(other),
                    },
                    height: match &self.layout.height {
                        Val::Auto => Dimension::Length(natural_h),
                        other => val_to_dimension(other),
                    },
                },
                margin: margin_to_rect_lpa(&self.layout.margin),
                flex_grow: self.layout.grow,
                flex_shrink: 1.0,
                align_self: self.layout.align_self.and_then(align_to_self),
                ..Default::default()
            })
            .unwrap()
    }

    pub fn apply_layout(&mut self, x: f32, y: f32, w: f32, h: f32) {
        self.x = x;
        self.y = y;
        self.w = w;
        self.h = h;
    }

    // layout builder methods
    pub fn width(mut self, v: Val) -> Self {
        self.layout.width = v;
        self
    }
    pub fn height(mut self, v: Val) -> Self {
        self.layout.height = v;
        self
    }
    pub fn grow(mut self, v: f32) -> Self {
        self.layout.grow = v;
        self
    }
    pub fn margin(mut self, e: Margin) -> Self {
        self.layout.margin = e;
        self
    }
    pub fn padding(mut self, e: Edges) -> Self {
        self.layout.padding = e;
        self
    }
    pub fn align_self(mut self, a: Align) -> Self {
        self.layout.align_self = Some(a);
        self
    }

    // style builder methods
    pub fn background(mut self, color: Color) -> Self {
        self.style.background = Some(color);
        self
    }
    pub fn text_color(mut self, color: Color) -> Self {
        self.style.text_color = Some(color);
        self
    }
    pub fn border_radius(mut self, v: f32) -> Self {
        self.style.border_radius = v;
        self
    }
    pub fn border(mut self, color: Color, thickness: f32) -> Self {
        self.style.border_color = Some(color);
        self.style.border_thickness = thickness;
        self
    }
    pub fn opacity(mut self, v: f32) -> Self {
        self.style.opacity = v;
        self
    }

    // text input specific builder methods
    pub fn value(mut self, v: &str) -> Self {
        self.value = Some(v.to_string());
        self
    }
    pub fn placeholder(mut self, text: &str) -> Self {
        self.placeholder = text.to_string();
        self
    }
    pub fn placeholder_color(mut self, color: Color) -> Self {
        self.placeholder_color = Some(color);
        self
    }
    pub fn font(mut self, name: &str) -> Self {
        self.font = Some(name.to_string());
        self
    }
    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = Some(size);
        self
    }
    pub fn font_weight(mut self, weight: u16) -> Self {
        self.font_weight = weight;
        self
    }
    pub fn on_change(mut self, f: impl Fn(String) -> M + 'static) -> Self {
        self.on_change = Some(Box::new(f));
        self
    }

    // interaction builder methods
    pub fn on_click(mut self, msg: M) -> Self {
        self.interactions.on_click = Some(msg);
        self
    }
    pub fn on_hover(mut self, msg: M) -> Self {
        self.interactions.on_hover = Some(msg);
        self
    }
}

// state helpers used by app.rs for keyboard handling

pub(crate) fn register_callback<M: Clone + 'static>(
    state: &mut StateStore,
    id: &str,
    callback: Box<dyn Fn(String) -> M>,
) {
    state.set_raw(id, TextInputCallback(callback));
}

pub(crate) fn call_callback<M: Clone + 'static>(
    state: &StateStore,
    id: &str,
    value: String,
) -> Option<M> {
    let cb = state.get_raw::<TextInputCallback<M>>(id)?;
    Some((cb.0)(value))
}

pub(crate) fn cache_value(state: &mut StateStore, id: &str, value: &str) {
    state.set_string(id, value);
}

pub(crate) fn get_cached_value(state: &StateStore, id: &str) -> String {
    state.get_string(id)
}

pub(crate) fn find_focused(state: &StateStore) -> Option<String> {
    state.find_by_type::<TextInputState, _>(|s| s.focused)
}

pub fn handle_key(
    state: &mut StateStore,
    id: &str,
    current_value: &str,
    event: &Event,
    text: &str,
) -> Option<String> {
    let focused = state.get_or_default::<TextInputState>(id).focused;
    if !focused {
        return None;
    }

    let mut value = current_value.to_string();
    let mut cursor = state
        .get_or_default::<TextInputState>(id)
        .cursor
        .min(value.len());
    let mut changed = false;

    match event {
        Event::KeyPressed {
            key: Key::Backspace,
            ..
        } => {
            if cursor > 0 {
                if let Some((i, _)) = value[..cursor].char_indices().next_back() {
                    value.remove(i);
                    cursor = i;
                    changed = true;
                }
            }
        }
        Event::KeyPressed {
            key: Key::Delete, ..
        } => {
            if cursor < value.len() {
                value.remove(cursor);
                changed = true;
            }
        }
        Event::KeyPressed { key: Key::Left, .. } => {
            if cursor > 0 {
                if let Some((i, _)) = value[..cursor].char_indices().next_back() {
                    cursor = i;
                }
            }
        }
        Event::KeyPressed {
            key: Key::Right, ..
        } => {
            if cursor < value.len() {
                cursor = value[cursor..]
                    .char_indices()
                    .nth(1)
                    .map(|(i, _)| cursor + i)
                    .unwrap_or(value.len());
            }
        }
        Event::KeyPressed { key: Key::Home, .. } => {
            cursor = 0;
        }
        Event::KeyPressed { key: Key::End, .. } => {
            cursor = value.len();
        }
        Event::KeyPressed { .. } => {
            if !text.is_empty() && text != "\r" && text != "\n" && text != "\r\n" {
                value.insert_str(cursor, text);
                cursor += text.len();
                changed = true;
            }
        }
        _ => {}
    }

    state.get_or_default_mut::<TextInputState>(id).cursor = cursor;
    if changed { Some(value) } else { None }
}
