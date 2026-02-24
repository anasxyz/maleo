use taffy::prelude::*;

use crate::draw::{DrawCtx, check_interactions, is_outside, with_opacity};
use crate::events::{Event, Key};
use crate::layout::{align_to_self, margin_to_rect_lpa, val_to_dimension};
use crate::state::StateStore;
use crate::{Align, Color, Edges, Fonts, Interactions, Layout, Margin, Style, TextAlign, Val};

// ─── persisted state ──────────────────────────────────────────────────────────

#[derive(Default)]
pub struct TextEditorState {
    pub focused: bool,
    pub cursor: usize,
    pub scroll_offset: f32,
    pub selection_anchor: Option<usize>,
    pub dragging: bool,
    pub cached_value: String,
}

pub(crate) struct TextEditorCallback<M>(pub Box<dyn Fn(String) -> M>);

// ─── widget ───────────────────────────────────────────────────────────────────

pub struct TextEditor<M: Clone + 'static> {
    pub id: Option<String>,
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

impl<M: Clone + 'static> TextEditor<M> {
    pub fn new() -> Self {
        Self {
            id: None,
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

    pub fn id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    fn require_id(&self) -> &str {
        self.id
            .as_deref()
            .expect("TextEditor requires an id — use .id(\"my_editor\")")
    }

    pub fn draw(&mut self, ctx: &mut DrawCtx<M>) {
        let id = self.require_id().to_string();
        let id = id.as_str();
        let (x, y, w, h) = (self.x, self.y, self.w, self.h);
        if is_outside(x, y, w, h, ctx.clip) {
            return;
        }

        let value_str = self.value.as_deref().unwrap_or("");

        // cache the current value into state so app.rs can read it on key events
        ctx.state
            .get_or_default_mut::<TextEditorState>(id)
            .cached_value = value_str.to_string();

        // register callback for this frame
        if let Some(cb) = self.on_change.take() {
            ctx.state.set_callback(id, TextEditorCallback(cb));
        }

        let hovered =
            ctx.mouse.x >= x && ctx.mouse.x <= x + w && ctx.mouse.y >= y && ctx.mouse.y <= y + h;

        if hovered {
            *ctx.cursor = Some(crate::draw::Cursor::Text);
        }

        update_focus_and_drag(
            ctx.state,
            id,
            hovered,
            ctx.mouse.left_just_pressed,
            ctx.mouse.left_pressed,
        );
        let focused = ctx.state.get_or_default::<TextEditorState>(id).focused;
        // clamp cursor to valid byte boundary
        ctx.state.get_or_default_mut::<TextEditorState>(id).cursor = clamp_to_char_boundary(
            value_str,
            ctx.state.get_or_default::<TextEditorState>(id).cursor,
        );

        let font_id = self
            .font
            .as_deref()
            .and_then(|name| ctx.fonts.resolve(Some(name)))
            .unwrap_or_else(|| ctx.fonts.default_id().unwrap());
        let family = ctx.fonts.get(font_id).family.clone();
        let size = self.font_size.unwrap_or(ctx.fonts.get(font_id).size);
        let line_height = size * 1.4;

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
        let pad_t = if self.layout.padding.top > 0.0 {
            self.layout.padding.top
        } else {
            8.0
        };
        let pad_b = if self.layout.padding.bottom > 0.0 {
            self.layout.padding.bottom
        } else {
            8.0
        };

        let text_area_w = w - pad_l - pad_r;
        let text_area_h = h - pad_t - pad_b;
        let text_clip = Some([x + pad_l, y + pad_t, x + w - pad_r, y + h - pad_b]);
        let sc = ctx.scale_factor;
        let text_origin_x = ((x + pad_l) * sc).floor() / sc;
        let text_origin_y_base = y + pad_t; // before scroll

        let lines: Vec<&str> = value_str.split('\n').collect();

        // update vertical scroll so cursor stays visible
        update_scroll(ctx.state, id, value_str, &lines, line_height, text_area_h);
        let scroll = ctx
            .state
            .get_or_default::<TextEditorState>(id)
            .scroll_offset;
        let scroll_snapped = (scroll * sc).floor() / sc;

        // handle mouse (may mutate cursor/selection)
        handle_mouse(
            ctx,
            id,
            value_str,
            &lines,
            font_id,
            size,
            line_height,
            text_origin_x,
            text_origin_y_base,
            scroll,
            hovered,
        );

        // re-read after mouse handling
        let cursor_pos = ctx.state.get_or_default::<TextEditorState>(id).cursor;
        let selection_anchor = ctx
            .state
            .get_or_default::<TextEditorState>(id)
            .selection_anchor;
        let has_selection = selection_anchor.map_or(false, |a| a != cursor_pos);

        // ── draw background ──────────────────────────────────────────────────
        draw_background(ctx, x, y, w, h, &self.style, focused, hovered);

        // ── draw selection ───────────────────────────────────────────────────
        if focused && has_selection {
            draw_selection(
                ctx,
                x,
                w,
                pad_l,
                pad_r,
                text_origin_x,
                text_origin_y_base,
                scroll_snapped,
                line_height,
                sc,
                text_clip,
                value_str,
                &lines,
                font_id,
                size,
                self.font_weight,
                cursor_pos,
                selection_anchor,
                self.style.opacity,
            );
        }

        // ── draw text / placeholder ──────────────────────────────────────────
        draw_text(
            ctx,
            x,
            w,
            pad_l,
            pad_r,
            text_origin_x,
            text_origin_y_base,
            scroll_snapped,
            line_height,
            sc,
            text_clip,
            family.clone(),
            size,
            self.font_weight,
            value_str,
            &lines,
            &self.placeholder,
            self.placeholder_color,
            self.style.text_color,
            self.style.opacity,
        );

        // ── draw cursor ──────────────────────────────────────────────────────
        if focused && !has_selection {
            draw_cursor(
                ctx,
                x,
                w,
                pad_l,
                pad_r,
                text_origin_x,
                text_origin_y_base,
                scroll_snapped,
                line_height,
                sc,
                text_clip,
                value_str,
                &lines,
                font_id,
                size,
                self.font_weight,
                cursor_pos,
                self.style.text_color,
                self.style.opacity,
            );
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
        let line_height = size * 1.4;
        let pad_v = if self.layout.padding.top > 0.0 {
            self.layout.padding.top + self.layout.padding.bottom
        } else {
            16.0
        };
        // default height: 5 lines
        let default_h = line_height * 5.0 + pad_v;
        taffy
            .new_leaf(taffy::Style {
                size: taffy::geometry::Size {
                    width: match &self.layout.width {
                        Val::Auto => Dimension::Length(300.0),
                        other => val_to_dimension(other),
                    },
                    height: match &self.layout.height {
                        Val::Auto => Dimension::Length(default_h),
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

    // ── layout builder ────────────────────────────────────────────────────────
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

    // ── style builder ─────────────────────────────────────────────────────────
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

    // ── text editor specific ──────────────────────────────────────────────────
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

    // ── interaction builder ───────────────────────────────────────────────────
    pub fn on_click(mut self, msg: M) -> Self {
        self.interactions.on_click = Some(msg);
        self
    }
    pub fn on_hover(mut self, msg: M) -> Self {
        self.interactions.on_hover = Some(msg);
        self
    }
}

// ─── draw helpers ─────────────────────────────────────────────────────────────

fn update_focus_and_drag(
    state: &mut StateStore,
    id: &str,
    hovered: bool,
    just_pressed: bool,
    pressed: bool,
) {
    if just_pressed {
        let s = state.get_or_default_mut::<TextEditorState>(id);
        s.focused = hovered;
        s.selection_anchor = None;
        s.dragging = hovered;
    }
    if !pressed {
        state.get_or_default_mut::<TextEditorState>(id).dragging = false;
    }
}

fn draw_background<M: Clone + 'static>(
    ctx: &mut DrawCtx<M>,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    style: &Style,
    focused: bool,
    hovered: bool,
) {
    let bg = style
        .background
        .unwrap_or(Color::new(0.13, 0.13, 0.17, 1.0));
    let (border_col, border_w) = if let Some(col) = style.border_color {
        (col, style.border_thickness)
    } else {
        let col = if focused {
            Color::new(0.4, 0.5, 0.9, 1.0)
        } else {
            Color::new(0.3, 0.3, 0.35, 1.0)
        };
        (col, 1.5)
    };

    ctx.sr.draw_rounded_rect(
        x,
        y,
        w,
        h,
        style.border_radius,
        with_opacity(bg.to_array(), style.opacity),
        if border_w > 0.0 {
            with_opacity(border_col.to_array(), style.opacity)
        } else {
            [0.0; 4]
        },
        border_w,
    );
}

/// Returns (line_index, byte_offset_within_line) for a given overall byte offset.
fn offset_to_line_col(lines: &[&str], offset: usize) -> (usize, usize) {
    let mut remaining = offset;
    for (i, line) in lines.iter().enumerate() {
        let len = line.len();
        if remaining <= len || i == lines.len() - 1 {
            return (i, remaining.min(len));
        }
        remaining -= len + 1; // +1 for the '\n'
    }
    (
        lines.len().saturating_sub(1),
        lines.last().map(|l| l.len()).unwrap_or(0),
    )
}

/// Convert (line_index, col_byte_offset) back to overall byte offset.
fn line_col_to_offset(lines: &[&str], line: usize, col: usize) -> usize {
    let mut off = 0;
    for (i, l) in lines.iter().enumerate() {
        if i == line {
            return off + col.min(l.len());
        }
        off += l.len() + 1; // +1 for '\n'
    }
    off
}

fn update_scroll(
    state: &mut StateStore,
    id: &str,
    value: &str,
    lines: &[&str],
    line_height: f32,
    text_area_h: f32,
) {
    let cursor_pos = state.get_or_default::<TextEditorState>(id).cursor;
    let (line_idx, _) = offset_to_line_col(lines, cursor_pos);
    let cursor_top = line_idx as f32 * line_height;
    let cursor_bot = cursor_top + line_height;

    let s = state.get_or_default_mut::<TextEditorState>(id);
    // scroll down if cursor goes below visible area
    if cursor_bot - s.scroll_offset > text_area_h {
        s.scroll_offset = cursor_bot - text_area_h;
    }
    // scroll up if cursor goes above visible area
    if cursor_top - s.scroll_offset < 0.0 {
        s.scroll_offset = cursor_top;
    }
    if s.scroll_offset < 0.0 {
        s.scroll_offset = 0.0;
    }
}

#[allow(clippy::too_many_arguments)]
fn handle_mouse<M: Clone + 'static>(
    ctx: &mut DrawCtx<M>,
    id: &str,
    value: &str,
    lines: &[&str],
    font_id: crate::FontId,
    size: f32,
    line_height: f32,
    text_origin_x: f32,
    text_origin_y_base: f32,
    scroll: f32,
    hovered: bool,
) {
    let dragging = ctx.state.get_or_default::<TextEditorState>(id).dragging;

    if ctx.mouse.left_just_pressed && hovered {
        let hit = hit_test(
            ctx.mouse.x,
            ctx.mouse.y,
            lines,
            font_id,
            size,
            line_height,
            text_origin_x,
            text_origin_y_base,
            scroll,
            ctx.fonts,
        );
        let state = ctx.state.get_or_default_mut::<TextEditorState>(id);
        match ctx.mouse.left_click_count {
            2 => {
                // select word
                let ws = word_start(value, hit);
                let we = word_end(value, hit);
                if ws < we {
                    state.selection_anchor = Some(ws);
                    state.cursor = we;
                } else {
                    state.cursor = hit;
                    state.selection_anchor = None;
                }
            }
            3 => {
                // select whole line
                let (li, _) = offset_to_line_col(lines, hit);
                let line_start = line_col_to_offset(lines, li, 0);
                let line_end = line_col_to_offset(lines, li, lines[li].len());
                state.selection_anchor = Some(line_start);
                state.cursor = line_end;
            }
            _ => {
                state.cursor = hit;
                state.selection_anchor = None;
            }
        }
    } else if dragging && ctx.mouse.left_pressed {
        let hit = hit_test(
            ctx.mouse.x,
            ctx.mouse.y,
            lines,
            font_id,
            size,
            line_height,
            text_origin_x,
            text_origin_y_base,
            scroll,
            ctx.fonts,
        );
        let state = ctx.state.get_or_default_mut::<TextEditorState>(id);
        if hit != state.cursor {
            if state.selection_anchor.is_none() {
                state.selection_anchor = Some(state.cursor);
            }
            state.cursor = hit;
        }
    }
}

/// Given mouse position, return the byte offset in the full string.
#[allow(clippy::too_many_arguments)]
fn hit_test(
    mouse_x: f32,
    mouse_y: f32,
    lines: &[&str],
    font_id: crate::FontId,
    size: f32,
    line_height: f32,
    text_origin_x: f32,
    text_origin_y_base: f32,
    scroll: f32,
    fonts: &mut Fonts,
) -> usize {
    // which line?
    let rel_y = (mouse_y - text_origin_y_base + scroll).max(0.0);
    let line_idx = ((rel_y / line_height) as usize).min(lines.len().saturating_sub(1));
    let line = lines[line_idx];

    // which character within the line?
    let click_x = (mouse_x - text_origin_x).max(0.0);
    let col = hit_test_line(line, click_x, fonts, font_id, size, 400);

    line_col_to_offset(lines, line_idx, col)
}

fn hit_test_line(
    line: &str,
    click_x: f32,
    fonts: &mut Fonts,
    font_id: crate::FontId,
    size: f32,
    weight: u16,
) -> usize {
    if line.is_empty() {
        return 0;
    }
    let boundaries: Vec<usize> = line
        .char_indices()
        .map(|(i, _)| i)
        .chain(std::iter::once(line.len()))
        .collect();
    let mut lo = 0usize;
    let mut hi = boundaries.len() - 1;
    while hi - lo > 1 {
        let mid = (lo + hi) / 2;
        let (measured, _) = fonts.measure_sized(&line[..boundaries[mid]], font_id, size, weight);
        if measured <= click_x {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    let (lo_w, _) = fonts.measure_sized(&line[..boundaries[lo]], font_id, size, weight);
    let (hi_w, _) = fonts.measure_sized(&line[..boundaries[hi]], font_id, size, weight);
    if (lo_w - click_x).abs() <= (hi_w - click_x).abs() {
        boundaries[lo]
    } else {
        boundaries[hi]
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_selection<M: Clone + 'static>(
    ctx: &mut DrawCtx<M>,
    x: f32,
    w: f32,
    pad_l: f32,
    pad_r: f32,
    text_origin_x: f32,
    text_origin_y_base: f32,
    scroll_snapped: f32,
    line_height: f32,
    sc: f32,
    clip: Option<[f32; 4]>,
    value: &str,
    lines: &[&str],
    font_id: crate::FontId,
    size: f32,
    weight: u16,
    cursor_pos: usize,
    selection_anchor: Option<usize>,
    opacity: f32,
) {
    let Some(anchor) = selection_anchor else {
        return;
    };
    if anchor == cursor_pos {
        return;
    }

    let sel_start = anchor.min(cursor_pos);
    let sel_end = anchor.max(cursor_pos);

    let (start_line, start_col) = offset_to_line_col(lines, sel_start);
    let (end_line, end_col) = offset_to_line_col(lines, sel_end);

    let sel_col = with_opacity([0.3, 0.5, 0.9, 0.4], opacity);
    let right_edge = x + w - pad_r;
    let left_edge = x + pad_l;

    for li in start_line..=end_line {
        let line = lines[li];
        // Snap y to pixel, compute height as distance to next snapped y
        // so adjacent lines tile perfectly with no gap or overlap.
        let line_y =
            ((text_origin_y_base + li as f32 * line_height - scroll_snapped) * sc).floor() / sc;
        let next_y = ((text_origin_y_base + (li + 1) as f32 * line_height - scroll_snapped) * sc)
            .floor()
            / sc;
        let lh = next_y - line_y;

        let col_start = if li == start_line { start_col } else { 0 };
        let col_end = if li == end_line { end_col } else { line.len() };

        let (sx_rel, _) = ctx
            .fonts
            .measure_sized(&line[..col_start], font_id, size, weight);
        let (ex_rel, _) = ctx
            .fonts
            .measure_sized(&line[..col_end], font_id, size, weight);

        let sx = (text_origin_x + sx_rel).max(left_edge);
        let ex = (text_origin_x + ex_rel).min(right_edge);
        // Empty lines in the middle of a selection get a small fixed-width highlight
        // so they're visibly included in the selection.
        let ex = if ex <= sx && li < end_line {
            sx + 8.0
        } else {
            ex
        };

        // Clamp rect vertically to clip region
        let (ry, rh) = if let Some([_, cy, _, cy2]) = clip {
            let clamped_top = line_y.max(cy);
            let clamped_bot = (line_y + lh).min(cy2);
            (clamped_top, clamped_bot - clamped_top)
        } else {
            (line_y, lh)
        };

        if ex > sx && rh > 0.0 {
            ctx.sr
                .draw_rect(sx, ry, ex - sx, rh, sel_col, [0.0; 4], 0.0);
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_text<M: Clone + 'static>(
    ctx: &mut DrawCtx<M>,
    x: f32,
    w: f32,
    pad_l: f32,
    pad_r: f32,
    text_origin_x: f32,
    text_origin_y_base: f32,
    scroll_snapped: f32,
    line_height: f32,
    sc: f32,
    clip: Option<[f32; 4]>,
    family: String,
    size: f32,
    weight: u16,
    value: &str,
    lines: &[&str],
    placeholder: &str,
    placeholder_color: Option<Color>,
    text_color: Option<Color>,
    opacity: f32,
) {
    if value.is_empty() {
        let col = placeholder_color.unwrap_or(Color::new(0.45, 0.45, 0.5, 1.0));
        let ty = (text_origin_y_base * sc).floor() / sc;
        ctx.tr.draw(
            &mut ctx.fonts.font_system,
            family,
            size,
            weight,
            false,
            TextAlign::Left,
            placeholder,
            text_origin_x,
            ty,
            99999.0,
            clip,
            with_opacity(col.to_array(), opacity).into(),
        );
        return;
    }

    let col = text_color.unwrap_or(Color::new(0.92, 0.92, 0.95, 1.0));
    let color_arr: [f32; 4] = with_opacity(col.to_array(), opacity);

    for (li, line) in lines.iter().enumerate() {
        let ty_raw = text_origin_y_base + li as f32 * line_height - scroll_snapped;
        let ty = (ty_raw * sc).floor() / sc;
        // skip lines completely outside clip
        if let Some([_, cy, _, cy2]) = clip {
            if ty + line_height < cy || ty > cy2 {
                continue;
            }
        }
        ctx.tr.draw(
            &mut ctx.fonts.font_system,
            family.clone(),
            size,
            weight,
            false,
            TextAlign::Left,
            line,
            text_origin_x,
            ty,
            99999.0,
            clip,
            color_arr.into(),
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_cursor<M: Clone + 'static>(
    ctx: &mut DrawCtx<M>,
    x: f32,
    w: f32,
    pad_l: f32,
    pad_r: f32,
    text_origin_x: f32,
    text_origin_y_base: f32,
    scroll_snapped: f32,
    line_height: f32,
    sc: f32,
    clip: Option<[f32; 4]>,
    value: &str,
    lines: &[&str],
    font_id: crate::FontId,
    size: f32,
    weight: u16,
    cursor_pos: usize,
    text_color: Option<Color>,
    opacity: f32,
) {
    let (line_idx, col) = offset_to_line_col(lines, cursor_pos);
    let line = lines[line_idx];
    let (cursor_x_rel, _) = ctx.fonts.measure_sized(&line[..col], font_id, size, weight);

    let ty_raw = text_origin_y_base + line_idx as f32 * line_height - scroll_snapped;
    let cursor_x = ((text_origin_x + cursor_x_rel) * sc).floor() / sc;
    let cursor_y = (ty_raw * sc).floor() / sc;
    let cursor_h = (line_height * sc).floor() / sc;

    // clip check
    let left_edge = x + pad_l;
    let right_edge = x + w - pad_r;
    if cursor_x < left_edge || cursor_x > right_edge {
        return;
    }
    if let Some([_, cy, _, cy2]) = clip {
        if cursor_y + cursor_h < cy || cursor_y > cy2 {
            return;
        }
    }

    let col_val = text_color.unwrap_or(Color::new(0.7, 0.75, 1.0, 1.0));
    ctx.sr.draw_rect(
        cursor_x,
        cursor_y,
        2.0,
        cursor_h,
        with_opacity(col_val.to_array(), opacity),
        [0.0; 4],
        0.0,
    );
}

// ─── text helpers ─────────────────────────────────────────────────────────────

fn clamp_to_char_boundary(s: &str, mut pos: usize) -> usize {
    pos = pos.min(s.len());
    while pos > 0 && !s.is_char_boundary(pos) {
        pos -= 1;
    }
    pos
}

fn word_start(value: &str, pos: usize) -> usize {
    let mut i = pos.min(value.len());
    while i > 0 {
        if let Some((j, c)) = value[..i].char_indices().next_back() {
            if c.is_alphanumeric() || c == '_' {
                i = j;
            } else {
                break;
            }
        } else {
            break;
        }
    }
    i
}

fn word_end(value: &str, pos: usize) -> usize {
    let mut i = pos;
    for (j, c) in value[pos..].char_indices() {
        if c.is_alphanumeric() || c == '_' {
            i = pos + j + c.len_utf8();
        } else {
            break;
        }
    }
    i
}

fn selection_range(cursor: usize, anchor: Option<usize>) -> (usize, usize) {
    match anchor {
        Some(a) => (a.min(cursor), a.max(cursor)),
        None => (cursor, cursor),
    }
}

// ─── state helpers (called from app.rs) ───────────────────────────────────────

pub(crate) fn call_callback<M: Clone + 'static>(
    state: &StateStore,
    id: &str,
    value: String,
) -> Option<M> {
    let cb = state.get_callback::<TextEditorCallback<M>>(id)?;
    Some((cb.0)(value))
}

pub(crate) fn find_focused(state: &StateStore) -> Option<String> {
    state.find::<TextEditorState, _>(|s| s.focused)
}

pub fn handle_key(state: &mut StateStore, id: &str, event: &Event, text: &str) -> Option<String> {
    let focused = state.get_or_default::<TextEditorState>(id).focused;
    if !focused {
        return None;
    }

    let mut value = state
        .get_or_default::<TextEditorState>(id)
        .cached_value
        .clone();
    let mut cursor =
        clamp_to_char_boundary(&value, state.get_or_default::<TextEditorState>(id).cursor);
    let mut selection_anchor = state.get_or_default::<TextEditorState>(id).selection_anchor;
    let mut changed = false;
    let has_selection = selection_anchor.map_or(false, |a| a != cursor);

    let lines: Vec<&str> = value.split('\n').collect();

    match event {
        Event::KeyPressed {
            key: Key::Backspace,
            ..
        } => {
            if has_selection {
                let (start, end) = selection_range(cursor, selection_anchor);
                value.drain(start..end);
                cursor = start;
                selection_anchor = None;
                changed = true;
            } else if cursor > 0 {
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
            if has_selection {
                let (start, end) = selection_range(cursor, selection_anchor);
                value.drain(start..end);
                cursor = start;
                selection_anchor = None;
                changed = true;
            } else if cursor < value.len() {
                value.remove(cursor);
                changed = true;
            }
        }
        Event::KeyPressed { key: Key::Left, .. } => {
            if has_selection {
                cursor = selection_range(cursor, selection_anchor).0;
                selection_anchor = None;
            } else if cursor > 0 {
                if let Some((i, _)) = value[..cursor].char_indices().next_back() {
                    cursor = i;
                }
            }
        }
        Event::KeyPressed {
            key: Key::Right, ..
        } => {
            if has_selection {
                cursor = selection_range(cursor, selection_anchor).1;
                selection_anchor = None;
            } else if cursor < value.len() {
                cursor = value[cursor..]
                    .char_indices()
                    .nth(1)
                    .map(|(i, _)| cursor + i)
                    .unwrap_or(value.len());
            }
        }
        Event::KeyPressed { key: Key::Up, .. } => {
            selection_anchor = None;
            let (li, col) = offset_to_line_col(&lines, cursor);
            if li > 0 {
                let prev_line = lines[li - 1];
                let col_clamped = col.min(prev_line.len());
                cursor = line_col_to_offset(&lines, li - 1, col_clamped);
            }
        }
        Event::KeyPressed { key: Key::Down, .. } => {
            selection_anchor = None;
            let (li, col) = offset_to_line_col(&lines, cursor);
            if li < lines.len() - 1 {
                let next_line = lines[li + 1];
                let col_clamped = col.min(next_line.len());
                cursor = line_col_to_offset(&lines, li + 1, col_clamped);
            }
        }
        Event::KeyPressed { key: Key::Home, .. } => {
            let (li, _) = offset_to_line_col(&lines, cursor);
            cursor = line_col_to_offset(&lines, li, 0);
            selection_anchor = None;
        }
        Event::KeyPressed { key: Key::End, .. } => {
            let (li, _) = offset_to_line_col(&lines, cursor);
            cursor = line_col_to_offset(&lines, li, lines[li].len());
            selection_anchor = None;
        }
        Event::KeyPressed { .. } => {
            // Enter inserts newline; other printable text is inserted normally
            let insert = if text == "\r" || text == "\r\n" {
                "\n"
            } else {
                text
            };
            if !insert.is_empty() {
                if has_selection {
                    let (start, end) = selection_range(cursor, selection_anchor);
                    value.drain(start..end);
                    cursor = start;
                    selection_anchor = None;
                }
                value.insert_str(cursor, insert);
                cursor += insert.len();
                changed = true;
            }
        }
        _ => {}
    }

    let s = state.get_or_default_mut::<TextEditorState>(id);
    s.cursor = cursor;
    s.selection_anchor = selection_anchor;
    if changed {
        s.cached_value = value.clone();
        Some(value)
    } else {
        None
    }
}
