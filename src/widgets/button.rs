use taffy::prelude::*;

use crate::draw::{DrawCtx, check_interactions, draw_shadow, draw_shape, is_outside, with_opacity};
use crate::layout::{align_to_self, margin_to_rect_lpa, val_to_dimension};
use crate::{Align, Color, Edges, Fonts, Interactions, Layout, Margin, Style, TextAlign, Val};

pub struct Button<M: Clone + 'static> {
    pub label: String,
    pub layout: Layout,
    pub style: Style,
    pub interactions: Interactions<M>,
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) w: f32,
    pub(crate) h: f32,
}

impl<M: Clone + 'static> Button<M> {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            layout: Layout::default(),
            style: Style::default(),
            interactions: Interactions::default(),
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

        let hovered =
            ctx.mouse.x >= x && ctx.mouse.x <= x + w && ctx.mouse.y >= y && ctx.mouse.y <= y + h;
        let pressed = hovered && ctx.mouse.left_just_pressed;

        if hovered {
            *ctx.cursor = Some(crate::draw::Cursor::Pointer);
        }

        let bg = if pressed {
            self.style
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
            self.style
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
            self.style
                .background
                .unwrap_or(Color::new(0.25, 0.25, 0.35, 1.0))
        };

        draw_shadow(ctx.shadow, x, y, w, h, &self.style);
        let border = self
            .style
            .border_color
            .unwrap_or(Color::TRANSPARENT)
            .to_array();
        draw_shape(
            ctx.sr,
            x,
            y,
            w,
            h,
            with_opacity(bg.to_array(), self.style.opacity),
            self.style.border_radius,
            with_opacity(border, self.style.opacity),
            self.style.border_thickness,
            ctx.clip,
        );

        let font_id = ctx.fonts.default_id().unwrap();
        let family = ctx.fonts.get(font_id).family.clone();
        let size = ctx.fonts.get(font_id).size;
        let (tw, th) = ctx.fonts.measure(&self.label, font_id);
        let tx = x + (w - tw) / 2.0;
        let ty = y + (h - th) / 2.0;
        let label_color = self
            .style
            .text_color
            .unwrap_or(Color::new(0.92, 0.92, 0.95, 1.0));
        ctx.tr.draw(
            &mut ctx.fonts.font_system,
            family,
            size,
            400,
            false,
            TextAlign::Left,
            &self.label,
            tx,
            ty,
            w,
            ctx.clip,
            with_opacity(label_color.to_array(), self.style.opacity).into(),
        );

        check_interactions(&self.interactions, x, y, w, h, ctx);
    }

    pub fn layout_node(&self, taffy: &mut TaffyTree<()>, fonts: &mut Fonts) -> NodeId {
        let font_id = fonts.default_id().unwrap();
        let (tw, th) = fonts.measure(&self.label, font_id);
        let natural_w = tw + 24.0;
        let natural_h = th + 12.0;
        taffy
            .new_leaf(taffy::Style {
                size: taffy::geometry::Size {
                    width: match &self.layout.width {
                        Val::Auto => Dimension::Length(natural_w),
                        other => val_to_dimension(other),
                    },
                    height: match &self.layout.height {
                        Val::Auto => Dimension::Length(natural_h),
                        other => val_to_dimension(other),
                    },
                },
                min_size: taffy::geometry::Size {
                    width: match &self.layout.width {
                        Val::Auto => Dimension::Length(natural_w),
                        other => val_to_dimension(other),
                    },
                    height: Dimension::Auto,
                },
                margin: margin_to_rect_lpa(&self.layout.margin),
                flex_grow: self.layout.grow,
                flex_shrink: 0.0,
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
    pub fn shadow(mut self, color: Color, offset_x: f32, offset_y: f32, blur: f32) -> Self {
        self.style.shadow_color = color;
        self.style.shadow_offset_x = offset_x;
        self.style.shadow_offset_y = offset_y;
        self.style.shadow_blur = blur;
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
    pub fn on_mouse_down(mut self, msg: M) -> Self {
        self.interactions.on_mouse_down = Some(msg);
        self
    }
}
