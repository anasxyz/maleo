use crate::state::StateStore;
use crate::{Element, Fonts, ShadowRenderer, ShapeRenderer, TextRenderer};

#[derive(Clone, Copy, PartialEq)]
pub enum Cursor {
    Default,
    Text,
    Pointer,
    Crosshair,
    Move,
    ResizeNS,
    ResizeEW,
    NotAllowed,
    Grab,
    Grabbing,
    Wait,
}

// all mouse state in one place
pub struct MouseState {
    pub x: f32,
    pub y: f32,
    pub left_pressed: bool,
    pub left_just_pressed: bool,
    pub left_just_released: bool,
    pub right_pressed: bool,
    pub right_just_pressed: bool,
    pub middle_pressed: bool,
    pub middle_just_pressed: bool,
    pub left_click_count: u32, // 1 = single, 2 = double, 3 = triple
    pub left_click_x: f32,
    pub left_click_y: f32,
}

// everything a widget needs to draw itself
pub struct DrawCtx<'a, M> {
    pub sr: &'a mut ShapeRenderer,
    pub shadow: &'a mut ShadowRenderer,
    pub tr: &'a mut TextRenderer,
    pub fonts: &'a mut Fonts,
    pub state: &'a mut StateStore,
    pub mouse: &'a MouseState,
    pub clip: Option<[f32; 4]>,
    pub actions: &'a mut Vec<M>,
    pub scale_factor: f32,
    pub cursor: &'a mut Option<Cursor>,
}

pub fn draw<M: Clone + 'static>(
    element: &mut Element<M>,
    sr: &mut ShapeRenderer,
    shadow: &mut ShadowRenderer,
    tr: &mut TextRenderer,
    fonts: &mut Fonts,
    state: &mut StateStore,
    mouse: &MouseState,
    scale_factor: f32,
) -> (Vec<M>, Option<Cursor>) {
    let mut actions = Vec::new();
    let mut cursor = None;
    let mut ctx = DrawCtx {
        sr,
        shadow,
        tr,
        fonts,
        state,
        mouse,
        clip: None,
        actions: &mut actions,
        scale_factor,
        cursor: &mut cursor,
    };
    draw_element(element, &mut ctx);
    (actions, cursor)
}

pub fn draw_element<M: Clone + 'static>(el: &mut Element<M>, ctx: &mut DrawCtx<M>) {
    match el {
        Element::Empty => {}
        Element::Rect(r) => r.draw(ctx),
        Element::Text(t) => t.draw(ctx),
        Element::Button(b) => b.draw(ctx),
        Element::TextInput(t) => t.draw(ctx),
        Element::Row(r) => r.draw(ctx),
        Element::Column(c) => c.draw(ctx),
    }
}

// helpers shared across widgets

pub fn is_outside(x: f32, y: f32, w: f32, h: f32, clip: Option<[f32; 4]>) -> bool {
    let Some([cx, cy, cx2, cy2]) = clip else {
        return false;
    };
    x + w < cx || y + h < cy || x > cx2 || y > cy2
}

pub fn make_child_clip(
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    overflow: crate::Overflow,
    parent_clip: Option<[f32; 4]>,
) -> Option<[f32; 4]> {
    if overflow == crate::Overflow::Hidden || overflow == crate::Overflow::Scroll {
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

pub fn with_opacity(mut color: [f32; 4], opacity: f32) -> [f32; 4] {
    color[3] *= opacity;
    color
}

pub fn draw_shape(
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

pub fn draw_shadow(
    shadow: &mut ShadowRenderer,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    style: &crate::Style,
) {
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

pub fn check_interactions<M: Clone + 'static>(
    interactions: &crate::Interactions<M>,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    ctx: &mut DrawCtx<M>,
) {
    let hovered =
        ctx.mouse.x >= x && ctx.mouse.x <= x + w && ctx.mouse.y >= y && ctx.mouse.y <= y + h;

    if hovered {
        if let Some(a) = &interactions.on_hover {
            ctx.actions.push(a.clone());
        }
        if ctx.mouse.left_just_pressed {
            if let Some(a) = &interactions.on_mouse_down {
                ctx.actions.push(a.clone());
            }
        }
        if ctx.mouse.left_just_released {
            if let Some(a) = &interactions.on_click {
                ctx.actions.push(a.clone());
            }
        }
    }
}
