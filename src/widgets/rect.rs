use taffy::prelude::*;

use crate::draw::{DrawCtx, check_interactions, draw_shadow, draw_shape, is_outside, with_opacity};
use crate::layout::{margin_to_rect_lpa, style_to_taffy};
use crate::{Color, Edges, Fonts, Interactions, Layout, Margin, Overflow, Style, Val};

pub struct Rect<M: Clone + 'static> {
    pub color: Color,
    pub layout: Layout,
    pub style: Style,
    pub interactions: Interactions<M>,
    pub(crate) w: f32,
    pub(crate) h: f32,
}

impl<M: Clone + 'static> Rect<M> {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            layout: Layout::default(),
            style: Style::default(),
            interactions: Interactions::default(),
            w: 0.0,
            h: 0.0,
        }
    }

    pub fn draw(&mut self, ctx: &mut DrawCtx<M>) {
        let (x, y, w, h) = (self.layout.x, self.layout.y, self.w, self.h);
        if is_outside(x, y, w, h, ctx.clip) {
            return;
        }
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
            with_opacity(self.color.to_array(), self.style.opacity),
            self.style.border_radius,
            with_opacity(border, self.style.opacity),
            self.style.border_thickness,
            ctx.clip,
        );
        check_interactions(&self.interactions, x, y, w, h, ctx);
    }

    pub fn layout_node(&self, taffy: &mut TaffyTree<()>, _fonts: &mut Fonts) -> NodeId {
        let mut ts = style_to_taffy(&self.layout, FlexDirection::Row);
        ts.justify_content = None;
        ts.align_items = None;
        taffy.new_leaf(ts).unwrap()
    }

    pub fn apply_layout(&mut self, x: f32, y: f32, w: f32, h: f32) {
        self.layout.x = x;
        self.layout.y = y;
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
    pub fn min_width(mut self, v: Val) -> Self {
        self.layout.min_width = v;
        self
    }
    pub fn max_width(mut self, v: Val) -> Self {
        self.layout.max_width = v;
        self
    }
    pub fn min_height(mut self, v: Val) -> Self {
        self.layout.min_height = v;
        self
    }
    pub fn max_height(mut self, v: Val) -> Self {
        self.layout.max_height = v;
        self
    }
    pub fn grow(mut self, v: f32) -> Self {
        self.layout.grow = v;
        self
    }
    pub fn shrink(mut self, v: f32) -> Self {
        self.layout.shrink = Some(v);
        self
    }
    pub fn padding(mut self, e: Edges) -> Self {
        self.layout.padding = e;
        self
    }
    pub fn margin(mut self, e: Margin) -> Self {
        self.layout.margin = e;
        self
    }
    pub fn overflow_hidden(mut self) -> Self {
        self.layout.overflow = Overflow::Hidden;
        self
    }
    pub fn overflow_scroll(mut self) -> Self {
        self.layout.overflow = Overflow::Scroll;
        self
    }
    pub fn absolute(mut self) -> Self {
        self.layout.position = crate::Position::Absolute;
        self
    }
    pub fn inset(mut self, e: Edges) -> Self {
        self.layout.inset = e;
        self
    }

    // style builder methods
    pub fn background(mut self, color: Color) -> Self {
        self.style.background = Some(color);
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
    pub fn align_self(mut self, a: crate::Align) -> Self {
        self.layout.align_self = Some(a);
        self
    }

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
