use crate::Color;
use crate::element::{Element, ElementType, Position};
use crate::render::shape_renderer::RectParams;

pub struct DrawCall {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    params: RectParams,
    z_index: i32,
    opacity: f32,
}

pub fn clip_intersect(a: Option<[f32; 4]>, b: Option<[f32; 4]>) -> Option<[f32; 4]> {
    match (a, b) {
        (Some([ax, ay, ax2, ay2]), Some([bx, by, bx2, by2])) => {
            Some([ax.max(bx), ay.max(by), ax2.min(bx2), ay2.min(by2)])
        }
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    }
}

pub fn collect_draws(
    el: &Element,
    clip: Option<[f32; 4]>,
    parent_z: i32,
    parent_opacity: f32,
    calls: &mut Vec<DrawCall>,
) {
    // skip invisible elements entirely
    if !el.style.visible {
        return;
    }

    let z = parent_z + el.style.z_index;
    let opacity = parent_opacity * el.style.opacity;

    match el._type {
        ElementType::Rect => {
            let mut color = el.style.fill.to_array();
            color[3] *= opacity;

            let mut border_color = el.style.border_color.unwrap_or(Color::BLACK).to_array();
            border_color[3] *= opacity;

            calls.push(DrawCall {
                x: el.style.x,
                y: el.style.y,
                w: el.style.w,
                h: el.style.h,
                params: RectParams {
                    color,
                    radius: el.style.border_radius.unwrap_or(0.0),
                    border_color,
                    border_width: el.style.border_thickness,
                    clip,
                },
                z_index: z,
                opacity,
            });
        }
        ElementType::Row | ElementType::Col => {
            let my_clip = Some([
                el.style.x,
                el.style.y,
                el.style.x + el.style.w,
                el.style.y + el.style.h,
            ]);

            if let Some(children) = &el.children {
                for child in children {
                    // absolutely positioned children escape parent clip
                    let child_clip = if child.style.position == Position::Absolute {
                        clip // only outer clip, not this container's clip
                    } else {
                        clip_intersect(clip, my_clip)
                    };
                    collect_draws(child, child_clip, z, opacity, calls);
                }
            }
        }
    }
}

pub fn draw_tree(el: &Element, draw: &mut crate::render::draw_ctx::DrawContext) {
    let mut calls: Vec<DrawCall> = Vec::new();
    collect_draws(el, None, 0, 1.0, &mut calls);

    // sort by z_index
    // stable so tree order is preserved within same z
    calls.sort_by_key(|c| c.z_index);

    for call in calls {
        draw.draw_rect(call.x, call.y, call.w, call.h, call.params);
    }
}
