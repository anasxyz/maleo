use crate::{
    Color, Element, Events, Fonts, Overflow, ShadowRenderer, ShapeRenderer, TextAlign, TextRenderer,
};

pub fn draw(
    element: &mut Element,
    shape_renderer: &mut ShapeRenderer,
    shadow_renderer: &mut ShadowRenderer,
    text_renderer: &mut TextRenderer,
    fonts: &mut Fonts,
    events: &Events,
) {
    draw_clipped(
        element,
        shape_renderer,
        shadow_renderer,
        text_renderer,
        fonts,
        events,
        None,
    );
}

fn draw_clipped(
    element: &mut Element,
    sr: &mut ShapeRenderer,
    shadow: &mut ShadowRenderer,
    tr: &mut TextRenderer,
    fonts: &mut Fonts,
    events: &Events,
    clip: Option<[f32; 4]>,
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
                w
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

            let hovered = events
                .mouse
                .over(*resolved_x, *resolved_y, *resolved_w, *resolved_h);
            let clicked = hovered && events.mouse.left_just_pressed;

            let bg = if clicked {
                style
                    .background
                    .map(|c| {
                        Color::rgb(
                            (c.r + 0.15).min(1.0),
                            (c.g + 0.15).min(1.0),
                            (c.b + 0.15).min(1.0),
                        )
                    })
                    .unwrap_or(Color::rgb(0.5, 0.5, 0.6))
            } else if hovered {
                style
                    .background
                    .map(|c| {
                        Color::rgb(
                            (c.r + 0.08).min(1.0),
                            (c.g + 0.08).min(1.0),
                            (c.b + 0.08).min(1.0),
                        )
                    })
                    .unwrap_or(Color::rgb(0.35, 0.35, 0.45))
            } else {
                style.background.unwrap_or(Color::rgb(0.25, 0.25, 0.35))
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
                Color::rgb(0.92, 0.92, 0.95),
            );

            if clicked {
                if let Some(cb) = on_click {
                    cb();
                }
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
                draw_clipped(child, sr, shadow, tr, fonts, events, child_clip);
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
                draw_clipped(child, sr, shadow, tr, fonts, events, child_clip);
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
