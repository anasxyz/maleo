use crate::state::StateStore;
use crate::widgets::text_input as ti;
use crate::{
    Color, Element, Fonts, Interactions, Overflow, ShadowRenderer, ShapeRenderer, TextAlign,
    TextRenderer,
};

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
    left_just_released: bool,
) -> Vec<M> {
    let mut actions = Vec::new();
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
        left_just_released,
        None,
        &mut actions,
    );
    actions
}

// check interactions for any element and push actions
// uses a state key derived from position to track previous hover state
fn check_interactions<M: Clone + 'static>(
    interactions: &Interactions<M>,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    mouse_x: f32,
    mouse_y: f32,
    left_just_pressed: bool,
    left_just_released: bool,
    state: &mut StateStore,
    actions: &mut Vec<M>,
) {
    let hovered = mouse_x >= x && mouse_x <= x + w && mouse_y >= y && mouse_y <= y + h;

    if hovered {
        if let Some(a) = &interactions.on_hover {
            actions.push(a.clone());
        }
        if left_just_pressed {
            if let Some(a) = &interactions.on_mouse_down {
                actions.push(a.clone());
            }
        }
        if left_just_released {
            if let Some(a) = &interactions.on_click {
                actions.push(a.clone());
            }
        }
    }
}

fn draw_clipped<M: Clone + 'static>(
    el: &mut Element<M>,
    sr: &mut ShapeRenderer,
    shadow: &mut ShadowRenderer,
    tr: &mut TextRenderer,
    fonts: &mut Fonts,
    state: &mut StateStore,
    mouse_x: f32,
    mouse_y: f32,
    left_just_pressed: bool,
    left_just_released: bool,
    clip: Option<[f32; 4]>,
    actions: &mut Vec<M>,
) {
    match el {
        Element::Empty => {}

        Element::Rect {
            color,
            style,
            interactions,
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
            check_interactions(
                interactions,
                style.x,
                style.y,
                *resolved_w,
                *resolved_h,
                mouse_x,
                mouse_y,
                left_just_pressed,
                left_just_released,
                state,
                actions,
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
            interactions,
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
            // text doesn't have a resolved size stored — skip interactions for now
            let _ = interactions;
        }

        Element::Button {
            label,
            style,
            interactions,
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
            let pressed = hovered && left_just_pressed;

            let bg = if pressed {
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
            let family = fonts.get(font_id).family.clone();
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

            check_interactions(
                interactions,
                *resolved_x,
                *resolved_y,
                *resolved_w,
                *resolved_h,
                mouse_x,
                mouse_y,
                left_just_pressed,
                left_just_released,
                state,
                actions,
            );
        }

        Element::TextInput {
            id,
            placeholder,
            placeholder_color,
            text_color,
            font,
            font_size,
            value,
            style,
            interactions,
            on_change,
            resolved_x,
            resolved_y,
            resolved_w,
            resolved_h,
        } => {
            if is_outside(*resolved_x, *resolved_y, *resolved_w, *resolved_h, clip) {
                return;
            }

            let value_str = value.as_deref().unwrap_or("");
            ti::cache_value(state, id, value_str);
            if let Some(cb) = on_change.take() {
                ti::register_callback(state, id, cb);
            }

            let x = *resolved_x;
            let y = *resolved_y;
            let w = *resolved_w;
            let h = *resolved_h;

            let hovered = mouse_x >= x && mouse_x <= x + w && mouse_y >= y && mouse_y <= y + h;
            if left_just_pressed {
                state.get_or_default_mut::<ti::TextInputState>(id).focused = hovered;
            }
            {
                let s = state.get_or_default_mut::<ti::TextInputState>(id);
                s.cursor = s.cursor.min(value_str.len());
            }
            let focused = state.get_or_default::<ti::TextInputState>(id).focused;

            let default_bg = if focused {
                Color::new(0.18, 0.18, 0.22, 1.0)
            } else if hovered {
                Color::new(0.16, 0.16, 0.20, 1.0)
            } else {
                Color::new(0.13, 0.13, 0.17, 1.0)
            };
            let bg = style.background.unwrap_or(default_bg);
            let default_border = if focused {
                Color::new(0.4, 0.5, 0.9, 1.0)
            } else {
                Color::new(0.3, 0.3, 0.35, 1.0)
            };
            let border_col = style.border_color.unwrap_or(default_border);
            let border_w = if style.border_thickness > 0.0 {
                style.border_thickness
            } else {
                1.5
            };
            let radius = if style.border_radius > 0.0 {
                style.border_radius
            } else {
                4.0
            };
            sr.draw_rounded_rect(
                x,
                y,
                w,
                h,
                radius,
                with_opacity(bg.to_array(), style.opacity),
                with_opacity(border_col.to_array(), style.opacity),
                border_w,
            );

            let font_id = font
                .as_deref()
                .and_then(|name| fonts.resolve(Some(name)))
                .unwrap_or_else(|| fonts.default_id().unwrap());
            let family = fonts.get(font_id).family.clone();
            let size = font_size.unwrap_or(fonts.get(font_id).size);
            let pad_l = if style.padding.left > 0.0 {
                style.padding.left
            } else {
                8.0
            };
            let pad_r = if style.padding.right > 0.0 {
                style.padding.right
            } else {
                8.0
            };
            let (_, th) = fonts.measure_sized("M", font_id, size);
            let ty = if style.padding.top > 0.0 {
                y + style.padding.top
            } else {
                y + (h - th) / 2.0
            };

            if value_str.is_empty() {
                let col = placeholder_color.unwrap_or(Color::new(0.45, 0.45, 0.5, 1.0));
                tr.draw(
                    &mut fonts.font_system,
                    family,
                    size,
                    400,
                    false,
                    TextAlign::Left,
                    placeholder,
                    x + pad_l,
                    ty,
                    w - pad_l - pad_r,
                    clip,
                    with_opacity(col.to_array(), style.opacity).into(),
                );
            } else {
                let col = text_color.unwrap_or(Color::new(0.92, 0.92, 0.95, 1.0));
                tr.draw(
                    &mut fonts.font_system,
                    family,
                    size,
                    400,
                    false,
                    TextAlign::Left,
                    value_str,
                    x + pad_l,
                    ty,
                    w - pad_l - pad_r,
                    clip,
                    with_opacity(col.to_array(), style.opacity).into(),
                );
            }

            if focused {
                let cursor = state.get_or_default::<ti::TextInputState>(id).cursor;
                let (cursor_x, _) = fonts.measure_sized(&value_str[..cursor], font_id, size);
                let cursor_col = text_color.unwrap_or(Color::new(0.7, 0.75, 1.0, 1.0));
                sr.draw_rect(
                    x + pad_l + cursor_x,
                    ty,
                    1.5,
                    th,
                    with_opacity(cursor_col.to_array(), style.opacity),
                    [0.0; 4],
                    0.0,
                );
            }

            check_interactions(
                interactions,
                x,
                y,
                w,
                h,
                mouse_x,
                mouse_y,
                left_just_pressed,
                left_just_released,
                state,
                actions,
            );
        }

        Element::Row {
            style,
            interactions,
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
            check_interactions(
                interactions,
                style.x,
                style.y,
                *resolved_w,
                *resolved_h,
                mouse_x,
                mouse_y,
                left_just_pressed,
                left_just_released,
                state,
                actions,
            );
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
                    left_just_released,
                    child_clip,
                    actions,
                );
            }
        }

        Element::Column {
            style,
            interactions,
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
            check_interactions(
                interactions,
                style.x,
                style.y,
                *resolved_w,
                *resolved_h,
                mouse_x,
                mouse_y,
                left_just_pressed,
                left_just_released,
                state,
                actions,
            );
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
                    left_just_released,
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
