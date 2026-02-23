use taffy::prelude::*;

use crate::draw::{DrawCtx, is_outside};
use crate::layout::{align_to_self, margin_to_rect_lpa};
use crate::{Align, Color, Fonts, Interactions, Layout, Margin, Style, TextAlign, Val};

pub struct Text<M: Clone + 'static> {
    pub content: String,
    pub color: Color,
    pub font: Option<String>,
    pub font_size: Option<f32>,
    pub font_weight: u16,
    pub italic: bool,
    pub text_align: TextAlign,
    pub layout: Layout,
    pub style: Style,
    pub interactions: Interactions<M>,
    pub(crate) w: f32,
}

impl<M: Clone + 'static> Text<M> {
    pub fn new(content: &str, color: Color) -> Self {
        Self {
            content: content.to_string(),
            color,
            font: None,
            font_size: None,
            font_weight: 400,
            italic: false,
            text_align: TextAlign::Left,
            layout: Layout::default(),
            style: Style::default(),
            interactions: Interactions::default(),
            w: 0.0,
        }
    }

    pub fn draw(&mut self, ctx: &mut DrawCtx<M>) {
        if is_outside(self.layout.x, self.layout.y, self.w, 999.0, ctx.clip) {
            return;
        }
        let font_id = ctx.fonts.resolve(self.font.as_deref()).unwrap();
        let family = ctx.fonts.get(font_id).family.clone();
        let size = self.font_size.unwrap_or(ctx.fonts.get(font_id).size);

        let x2 = self.layout.x + self.w;
        let text_clip = Some(match ctx.clip {
            Some([cx, cy, cx2, cy2]) => [
                self.layout.x.max(cx),
                self.layout.y.max(cy),
                x2.min(cx2),
                (self.layout.y + 9999.0).min(cy2),
            ],
            None => [self.layout.x, self.layout.y, x2, self.layout.y + 9999.0],
        });

        ctx.tr.draw(
            &mut ctx.fonts.font_system,
            family,
            size,
            self.font_weight,
            self.italic,
            self.text_align,
            &self.content,
            self.layout.x,
            self.layout.y,
            99999.0,
            text_clip,
            self.color,
        );
    }

    pub fn layout_node(&self, taffy: &mut TaffyTree<()>, fonts: &mut Fonts) -> NodeId {
        let font_id = fonts.resolve(self.font.as_deref()).unwrap();
        let (w, h) = match self.font_size {
            Some(size) => fonts.measure_sized(&self.content, font_id, size),
            None => fonts.measure(&self.content, font_id),
        };
        taffy
            .new_leaf(taffy::Style {
                size: taffy::geometry::Size {
                    width: Dimension::Length(w),
                    height: Dimension::Length(h),
                },
                min_size: taffy::geometry::Size {
                    width: if self.layout.grow > 0.0 {
                        Dimension::Length(0.0)
                    } else {
                        Dimension::Auto
                    },
                    height: Dimension::Auto,
                },
                margin: margin_to_rect_lpa(&self.layout.margin),
                flex_grow: self.layout.grow,
                flex_shrink: 1.0,
                align_self: self.layout.align_self.and_then(align_to_self),
                ..Default::default()
            })
            .unwrap()
    }

    pub fn apply_layout(&mut self, x: f32, y: f32, w: f32, _h: f32) {
        self.layout.x = x;
        self.layout.y = y;
        self.w = w;
    }

    // layout builder methods
    pub fn width(mut self, v: Val) -> Self {
        self.layout.width = v;
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
    pub fn opacity(mut self, v: f32) -> Self {
        self.style.opacity = v;
        self
    }

    // text-specific builder methods
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
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }
    pub fn text_align(mut self, align: TextAlign) -> Self {
        self.text_align = align;
        self
    }
}
