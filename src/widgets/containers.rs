use taffy::prelude::*;

use crate::draw::{
    DrawCtx, check_interactions, draw_element, draw_shadow, draw_shape, make_child_clip,
    with_opacity,
};
use crate::layout::{align_to_items, align_to_justify, build_taffy_node_pub, style_to_taffy};
use crate::{
    Align, Color, Edges, Element, Fonts, Interactions, Layout, Margin, Overflow, Style, Val,
};

// Row

pub struct Row<M: Clone + 'static> {
    pub layout: Layout,
    pub style: Style,
    pub interactions: Interactions<M>,
    pub children: Vec<Element<M>>,
    pub(crate) w: f32,
    pub(crate) h: f32,
}

impl<M: Clone + 'static> Row<M> {
    pub fn new(children: Vec<Element<M>>) -> Self {
        Self {
            layout: Layout::default(),
            style: Style::default(),
            interactions: Interactions::default(),
            children,
            w: 0.0,
            h: 0.0,
        }
    }

    pub fn draw(&mut self, ctx: &mut DrawCtx<M>) {
        let (x, y, w, h) = (self.layout.x, self.layout.y, self.w, self.h);
        draw_shadow(ctx.shadow, x, y, w, h, &self.style);
        if let Some(bg) = self.style.background {
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
        }
        check_interactions(&self.interactions, x, y, w, h, ctx);
        let child_clip = make_child_clip(x, y, w, h, self.layout.overflow, ctx.clip);
        for child in &mut self.children {
            let mut child_ctx = DrawCtx {
                sr: ctx.sr,
                shadow: ctx.shadow,
                tr: ctx.tr,
                fonts: ctx.fonts,
                state: ctx.state,
                mouse: ctx.mouse,
                clip: child_clip,
                actions: ctx.actions,
                scale_factor: ctx.scale_factor,
            };
            draw_element(child, &mut child_ctx);
        }
    }

    pub fn layout_node(&self, taffy: &mut TaffyTree<()>, fonts: &mut Fonts) -> NodeId {
        let child_nodes: Vec<NodeId> = self
            .children
            .iter()
            .map(|c| build_taffy_node_pub(taffy, c, fonts))
            .collect();
        let mut ts = style_to_taffy(&self.layout, FlexDirection::Row);
        ts.justify_content = align_to_justify(self.layout.align_x);
        ts.align_items = align_to_items(self.layout.align_y);
        taffy.new_with_children(ts, &child_nodes).unwrap()
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
    pub fn wrap(mut self) -> Self {
        self.layout.wrap = true;
        self
    }
    pub fn gap(mut self, v: f32) -> Self {
        self.layout.gap = v;
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
    pub fn align_x(mut self, a: Align) -> Self {
        self.layout.align_x = a;
        self
    }
    pub fn align_y(mut self, a: Align) -> Self {
        self.layout.align_y = a;
        self
    }
    pub fn align_self(mut self, a: Align) -> Self {
        self.layout.align_self = Some(a);
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

// Column

pub struct Column<M: Clone + 'static> {
    pub layout: Layout,
    pub style: Style,
    pub interactions: Interactions<M>,
    pub children: Vec<Element<M>>,
    pub(crate) w: f32,
    pub(crate) h: f32,
}

impl<M: Clone + 'static> Column<M> {
    pub fn new(children: Vec<Element<M>>) -> Self {
        Self {
            layout: Layout::default(),
            style: Style::default(),
            interactions: Interactions::default(),
            children,
            w: 0.0,
            h: 0.0,
        }
    }

    pub fn draw(&mut self, ctx: &mut DrawCtx<M>) {
        let (x, y, w, h) = (self.layout.x, self.layout.y, self.w, self.h);
        draw_shadow(ctx.shadow, x, y, w, h, &self.style);
        if let Some(bg) = self.style.background {
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
        }
        check_interactions(&self.interactions, x, y, w, h, ctx);
        let child_clip = make_child_clip(x, y, w, h, self.layout.overflow, ctx.clip);
        for child in &mut self.children {
            let mut child_ctx = DrawCtx {
                sr: ctx.sr,
                shadow: ctx.shadow,
                tr: ctx.tr,
                fonts: ctx.fonts,
                state: ctx.state,
                mouse: ctx.mouse,
                clip: child_clip,
                actions: ctx.actions,
                scale_factor: ctx.scale_factor,
            };
            draw_element(child, &mut child_ctx);
        }
    }

    pub fn layout_node(&self, taffy: &mut TaffyTree<()>, fonts: &mut Fonts) -> NodeId {
        let child_nodes: Vec<NodeId> = self
            .children
            .iter()
            .map(|c| build_taffy_node_pub(taffy, c, fonts))
            .collect();
        let mut ts = style_to_taffy(&self.layout, FlexDirection::Column);
        ts.justify_content = align_to_justify(self.layout.align_y);
        ts.align_items = align_to_items(self.layout.align_x);
        taffy.new_with_children(ts, &child_nodes).unwrap()
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
    pub fn wrap(mut self) -> Self {
        self.layout.wrap = true;
        self
    }
    pub fn gap(mut self, v: f32) -> Self {
        self.layout.gap = v;
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
    pub fn align_x(mut self, a: Align) -> Self {
        self.layout.align_x = a;
        self
    }
    pub fn align_y(mut self, a: Align) -> Self {
        self.layout.align_y = a;
        self
    }
    pub fn align_self(mut self, a: Align) -> Self {
        self.layout.align_self = Some(a);
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
