use crate::state::StateStore;
use crate::{
    Color, Element, Fonts, Overflow, ShadowRenderer, ShapeRenderer, TextAlign, TextRenderer,
};

// internal state stored per text input widget — only cursor and focus, not the value
#[derive(Default)]
pub struct TextInputState {
    pub focused: bool,
    pub cursor: usize,
}

pub fn draw<M: Clone + 'static>(
    element: &mut Element<M>,
    shape_renderer: &mut ShapeRenderer,
    shadow_renderer: &mut ShadowRenderer,
    text_renderer: &mut TextRenderer,
    fonts: &mut Fonts,
    state: &mut StateStore,
    mouse_x: f32,
    mouse_y: f32,
    left_just_pressed: bool,
) -> Vec<M> {
    let mut actions: Vec<M> = Vec::new();
    draw_clipped(
        element,
        shape_renderer,
        shadow_renderer,
        text_renderer,
        fonts,
        state,
        mouse_x,
        mouse_y,
        left_just_pressed,
        None,
        &mut actions,
    );
    actions
}

fn draw_clipped<M: Clone + 'static>(
    element: &mut Element<M>,
    sr: &mut ShapeRenderer,
    shadow: &mut ShadowRenderer,
    tr: &mut TextRenderer,
    fonts: &mut Fonts,
    state: &mut StateStore,
    mouse_x: f32,
    mouse_y: f32,
    left_just_pressed: bool,
    clip: Option<[f32; 4]>,
    actions: &mut Vec<M>,
) {
    match element {
        Element::Empty => {}

        Element::Rect {
            color,
            style,
            resolved_w,
            resolved_h,
        } => {
            if is_outside(style.x, style.y, *resolved_w, *resolved_h, clip) {
                return;
            }
            draw_shadow(shadow, style.x, style.y, *resolved_w, *resolved_h, style);
            let border = style.border_color.unwrap_or(Color::TRANSPARENT).to_array();
            draw_shape(
                sr,
                style.x,
                style.y,
                *resolved_w,
                *resolved_h,
                with_opacity(color.to_array(), style.opacity),
                style.border_radius,
                with_opacity(border, style.opacity),
                style.border_thickness,
                clip,
            );
        }

        Element::Text {
            content,
            color,
            font,
            font_size,
            font_weight,
            italic,
            text_align,
            style,
        } => {
            if is_outside(style.x, style.y, 1.0, 1.0, clip) {
                return;
            }
            let font_id = fonts.resolve(font.as_deref()).unwrap();
            let family = fonts.get(font_id).family.clone();
            let size = font_size.unwrap_or(fonts.get(font_id).size);
            let width = if *text_align != TextAlign::Left {
                let (w, _) = match font_size {
                    Some(s) => fonts.measure_sized(content, font_id, *s),
                    None => fonts.measure(content, font_id),
                };
                w + 2.0
            } else {
                f32::MAX
            };
            tr.draw(
                &mut fonts.font_system,
                family,
                size,
                *font_weight,
                *italic,
                *text_align,
                content,
                style.x,
                style.y,
                width,
                clip,
                *color,
            );
        }

        Element::Button {
            label,
            style,
            on_click,
            resolved_x,
            resolved_y,
            resolved_w,
            resolved_h,
        } => {
            if is_outside(*resolved_x, *resolved_y, *resolved_w, *resolved_h, clip) {
                return;
            }

            let hovered = mouse_x >= *resolved_x
                && mouse_x <= *resolved_x + *resolved_w
                && mouse_y >= *resolved_y
                && mouse_y <= *resolved_y + *resolved_h;
            let clicked = hovered && left_just_pressed;

            let bg = if clicked {
                style
                    .background
                    .map(|c| {
                        Color::new(
                            (c.r + 0.15).min(1.0),
                            (c.g + 0.15).min(1.0),
                            (c.b + 0.15).min(1.0),
                            1.0,
                        )
                    })
                    .unwrap_or(Color::new(0.5, 0.5, 0.6, 1.0))
            } else if hovered {
                style
                    .background
                    .map(|c| {
                        Color::new(
                            (c.r + 0.08).min(1.0),
                            (c.g + 0.08).min(1.0),
                            (c.b + 0.08).min(1.0),
                            1.0,
                        )
                    })
                    .unwrap_or(Color::new(0.35, 0.35, 0.45, 1.0))
            } else {
                style
                    .background
                    .unwrap_or(Color::new(0.25, 0.25, 0.35, 1.0))
            };

            draw_shadow(
                shadow,
                *resolved_x,
                *resolved_y,
                *resolved_w,
                *resolved_h,
                style,
            );
            let border = style.border_color.unwrap_or(Color::TRANSPARENT).to_array();
            draw_shape(
                sr,
                *resolved_x,
                *resolved_y,
                *resolved_w,
                *resolved_h,
                with_opacity(bg.to_array(), style.opacity),
                style.border_radius,
                with_opacity(border, style.opacity),
                style.border_thickness,
                clip,
            );

            let font_id = fonts.default_id().unwrap();
            let family = fonts.get(font_id).family.clone(); // clone before font_system borrow
            let size = fonts.get(font_id).size;
            let (tw, th) = fonts.measure(label, font_id);
            let tx = *resolved_x + (*resolved_w - tw) / 2.0;
            let ty = *resolved_y + (*resolved_h - th) / 2.0;
            tr.draw(
                &mut fonts.font_system,
                family,
                size,
                400,
                false,
                TextAlign::Left,
                label,
                tx,
                ty,
                *resolved_w,
                clip,
                Color::new(0.92, 0.92, 0.95, 1.0),
            );

            if clicked {
                if let Some(action) = on_click {
                    actions.push(action.clone());
                }
            }
        }

        Element::TextInput {
            id,
            placeholder,
            value,
            style: _,
            on_change,
            resolved_x,
            resolved_y,
            resolved_w,
            resolved_h,
        } => {
            if is_outside(*resolved_x, *resolved_y, *resolved_w, *resolved_h, clip) {
                return;
            }

            // clamp cursor to current value length in case value shrank externally
            let value_str = value.as_deref().unwrap_or("");
            {
                let s = state.get_or_default_mut::<TextInputState>(id);
                if s.cursor > value_str.len() {
                    s.cursor = value_str.len();
                }
            }

            // cache current value in state so runner can access it on key events
            state.set_input_value(id, value_str);

            // register callback each frame so runner can call it on key events
            if let Some(cb) = on_change.take() {
                state.register_text_callback(id, cb);
            }

            let x = *resolved_x;
            let y = *resolved_y;
            let w = *resolved_w;
            let h = *resolved_h;

            // click to focus
            let hovered = mouse_x >= x && mouse_x <= x + w && mouse_y >= y && mouse_y <= y + h;
            if hovered && left_just_pressed {
                state.get_or_default_mut::<TextInputState>(id).focused = true;
            } else if left_just_pressed && !hovered {
                state.get_or_default_mut::<TextInputState>(id).focused = false;
            }

            let focused = state.get_or_default::<TextInputState>(id).focused;

            // background
            let bg = if focused {
                Color::new(0.18, 0.18, 0.22, 1.0)
            } else if hovered {
                Color::new(0.16, 0.16, 0.20, 1.0)
            } else {
                Color::new(0.13, 0.13, 0.17, 1.0)
            };
            let border = if focused {
                Color::new(0.4, 0.5, 0.9, 1.0)
            } else {
                Color::new(0.3, 0.3, 0.35, 1.0)
            };

            sr.draw_rounded_rect(x, y, w, h, 4.0, bg.to_array(), border.to_array(), 1.5);

            // text or placeholder
            let pad = 8.0;
            let font_id = fonts.default_id().unwrap();
            let family = fonts.get(font_id).family.clone();
            let size = fonts.get(font_id).size;
            let (_, th) = fonts.measure("M", font_id);
            let ty = y + (h - th) / 2.0;

            if value_str.is_empty() {
                tr.draw(
                    &mut fonts.font_system,
                    family.clone(),
                    size,
                    400,
                    false,
                    TextAlign::Left,
                    placeholder,
                    x + pad,
                    ty,
                    w - pad * 2.0,
                    clip,
                    Color::new(0.45, 0.45, 0.5, 1.0),
                );
            } else {
                tr.draw(
                    &mut fonts.font_system,
                    family.clone(),
                    size,
                    400,
                    false,
                    TextAlign::Left,
                    value_str,
                    x + pad,
                    ty,
                    w - pad * 2.0,
                    clip,
                    Color::new(0.92, 0.92, 0.95, 1.0),
                );
            }

            // cursor
            if focused {
                let cursor_pos = state.get_or_default::<TextInputState>(id).cursor;
                let text_before = &value_str[..cursor_pos.min(value_str.len())];
                let (cursor_x, _) = fonts.measure(text_before, font_id);
                let cx = x + pad + cursor_x;
                sr.draw_rect(
                    cx,
                    y + 4.0,
                    1.5,
                    h - 8.0,
                    Color::new(0.7, 0.75, 1.0, 1.0).to_array(),
                    [0.0; 4],
                    0.0,
                );
            }
        }

        Element::Row {
            style,
            children,
            resolved_w,
            resolved_h,
        } => {
            draw_shadow(shadow, style.x, style.y, *resolved_w, *resolved_h, style);
            if let Some(bg) = style.background {
                let border = style.border_color.unwrap_or(Color::TRANSPARENT).to_array();
                draw_shape(
                    sr,
                    style.x,
                    style.y,
                    *resolved_w,
                    *resolved_h,
                    with_opacity(bg.to_array(), style.opacity),
                    style.border_radius,
                    with_opacity(border, style.opacity),
                    style.border_thickness,
                    clip,
                );
            }
            let child_clip = make_child_clip(
                style.x,
                style.y,
                *resolved_w,
                *resolved_h,
                style.overflow,
                clip,
            );
            for child in children {
                draw_clipped(
                    child,
                    sr,
                    shadow,
                    tr,
                    fonts,
                    state,
                    mouse_x,
                    mouse_y,
                    left_just_pressed,
                    child_clip,
                    actions,
                );
            }
        }

        Element::Column {
            style,
            children,
            resolved_w,
            resolved_h,
        } => {
            draw_shadow(shadow, style.x, style.y, *resolved_w, *resolved_h, style);
            if let Some(bg) = style.background {
                let border = style.border_color.unwrap_or(Color::TRANSPARENT).to_array();
                draw_shape(
                    sr,
                    style.x,
                    style.y,
                    *resolved_w,
                    *resolved_h,
                    with_opacity(bg.to_array(), style.opacity),
                    style.border_radius,
                    with_opacity(border, style.opacity),
                    style.border_thickness,
                    clip,
                );
            }
            let child_clip = make_child_clip(
                style.x,
                style.y,
                *resolved_w,
                *resolved_h,
                style.overflow,
                clip,
            );
            for child in children {
                draw_clipped(
                    child,
                    sr,
                    shadow,
                    tr,
                    fonts,
                    state,
                    mouse_x,
                    mouse_y,
                    left_just_pressed,
                    child_clip,
                    actions,
                );
            }
        }
    }
}

fn draw_shape(
    sr: &mut ShapeRenderer,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    color: [f32; 4],
    border_radius: f32,
    border_color: [f32; 4],
    border_thickness: f32,
    clip: Option<[f32; 4]>,
) {
    if border_radius > 0.0 {
        if let Some([cx, cy, cx2, cy2]) = clip {
            if x < cx || y < cy || x + w > cx2 || y + h > cy2 {
                sr.draw_rect_clipped(x, y, w, h, color, [cx, cy, cx2, cy2]);
                return;
            }
        }
        sr.draw_rounded_rect(
            x,
            y,
            w,
            h,
            border_radius,
            color,
            border_color,
            border_thickness,
        );
    } else if let Some([cx, cy, cx2, cy2]) = clip {
        sr.draw_rect_clipped(x, y, w, h, color, [cx, cy, cx2, cy2]);
    } else {
        sr.draw_rect(x, y, w, h, color, border_color, border_thickness);
    }
}

fn draw_shadow(shadow: &mut ShadowRenderer, x: f32, y: f32, w: f32, h: f32, style: &crate::Style) {
    if style.shadow_color.a > 0.0 && style.shadow_blur > 0.0 {
        shadow.draw_shadow(
            x,
            y,
            w,
            h,
            with_opacity(style.shadow_color.to_array(), style.opacity),
            style.border_radius,
            style.shadow_blur,
            style.shadow_offset_x,
            style.shadow_offset_y,
        );
    }
}

fn is_outside(x: f32, y: f32, w: f32, h: f32, clip: Option<[f32; 4]>) -> bool {
    let Some([cx, cy, cx2, cy2]) = clip else {
        return false;
    };
    x + w < cx || y + h < cy || x > cx2 || y > cy2
}

fn make_child_clip(
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    overflow: Overflow,
    parent_clip: Option<[f32; 4]>,
) -> Option<[f32; 4]> {
    if overflow == Overflow::Hidden || overflow == Overflow::Scroll {
        let new_clip = [x, y, x + w, y + h];
        if let Some([px, py, px2, py2]) = parent_clip {
            Some([
                new_clip[0].max(px),
                new_clip[1].max(py),
                new_clip[2].min(px2),
                new_clip[3].min(py2),
            ])
        } else {
            Some(new_clip)
        }
    } else {
        parent_clip
    }
}

fn with_opacity(mut color: [f32; 4], opacity: f32) -> [f32; 4] {
    color[3] *= opacity;
    color
}

// process a key event for a focused input — takes current value, returns new value if changed
// cursor lives in state, value lives in the app
pub fn text_input_key(
    state: &mut StateStore,
    id: &str,
    current_value: &str,
    event: &crate::events::Event,
    text: &str,
) -> Option<String> {
    use crate::events::{Event, Key};

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
            // use the actual text from the OS — handles shift, locale, keyboard layout
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

// scan state to find which text input (if any) is currently focused
pub fn find_focused_input(state: &StateStore) -> Option<String> {
    state.find_focused_text_input()
}
