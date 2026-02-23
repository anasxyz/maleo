use crate::Color;
use crate::widgets::containers::{Column, Row};
use crate::widgets::{button::Button, rect::Rect, text::Text, text_input::TextInput};

// alignment

#[derive(Clone, Copy, PartialEq, Default)]
pub enum Align {
    #[default]
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

// text align

#[derive(Clone, Copy, PartialEq, Default)]
pub enum TextAlign {
    #[default]
    Left,
    Center,
    Right,
}

#[derive(Clone, Default)]
pub enum Val {
    #[default]
    Auto,
    Px(f32),
    Percent(f32),
}

pub fn px(v: f32) -> Val {
    Val::Px(v)
}
pub fn percent(v: f32) -> Val {
    Val::Percent(v)
}
pub fn auto() -> Val {
    Val::Auto
}

// edges — used for padding, margin, inset, etc

#[derive(Clone, Copy, Default)]
pub struct Edges {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl Edges {
    pub fn all(v: f32) -> Self {
        Self {
            top: v,
            right: v,
            bottom: v,
            left: v,
        }
    }
    pub fn horizontal(v: f32) -> Self {
        Self {
            top: 0.0,
            right: v,
            bottom: 0.0,
            left: v,
        }
    }
    pub fn vertical(v: f32) -> Self {
        Self {
            top: v,
            right: 0.0,
            bottom: v,
            left: 0.0,
        }
    }
    pub fn top(v: f32) -> Self {
        Self {
            top: v,
            right: 0.0,
            bottom: 0.0,
            left: 0.0,
        }
    }
    pub fn bottom(v: f32) -> Self {
        Self {
            top: 0.0,
            right: 0.0,
            bottom: v,
            left: 0.0,
        }
    }
    pub fn left(v: f32) -> Self {
        Self {
            top: 0.0,
            right: 0.0,
            bottom: 0.0,
            left: v,
        }
    }
    pub fn right(v: f32) -> Self {
        Self {
            top: 0.0,
            right: v,
            bottom: 0.0,
            left: 0.0,
        }
    }
}

pub type Padding = Edges;

// margin — None on a side means auto (absorbs remaining space, used for centering)

#[derive(Clone, Copy)]
pub struct Margin {
    pub top: Option<f32>,
    pub right: Option<f32>,
    pub bottom: Option<f32>,
    pub left: Option<f32>,
}

impl Default for Margin {
    fn default() -> Self {
        Self {
            top: Some(0.0),
            right: Some(0.0),
            bottom: Some(0.0),
            left: Some(0.0),
        }
    }
}

impl Margin {
    pub fn all(v: f32) -> Self {
        Self {
            top: Some(v),
            right: Some(v),
            bottom: Some(v),
            left: Some(v),
        }
    }
    pub fn auto() -> Self {
        Self {
            top: None,
            right: None,
            bottom: None,
            left: None,
        }
    }
    pub fn horizontal_auto() -> Self {
        Self {
            top: Some(0.0),
            right: None,
            bottom: Some(0.0),
            left: None,
        }
    }
    pub fn vertical_auto() -> Self {
        Self {
            top: None,
            right: Some(0.0),
            bottom: None,
            left: Some(0.0),
        }
    }
    pub fn horizontal(v: f32) -> Self {
        Self {
            top: Some(0.0),
            right: Some(v),
            bottom: Some(0.0),
            left: Some(v),
        }
    }
    pub fn vertical(v: f32) -> Self {
        Self {
            top: Some(v),
            right: Some(0.0),
            bottom: Some(v),
            left: Some(0.0),
        }
    }
    pub fn top(v: f32) -> Self {
        Self {
            top: Some(v),
            right: Some(0.0),
            bottom: Some(0.0),
            left: Some(0.0),
        }
    }
    pub fn bottom(v: f32) -> Self {
        Self {
            top: Some(0.0),
            right: Some(0.0),
            bottom: Some(v),
            left: Some(0.0),
        }
    }
    pub fn left(v: f32) -> Self {
        Self {
            top: Some(0.0),
            right: Some(0.0),
            bottom: Some(0.0),
            left: Some(v),
        }
    }
    pub fn right(v: f32) -> Self {
        Self {
            top: Some(0.0),
            right: Some(v),
            bottom: Some(0.0),
            left: Some(0.0),
        }
    }
}

// position

#[derive(Clone, Copy, PartialEq, Default)]
pub enum Position {
    #[default]
    Relative,
    Absolute,
}

// overflow

#[derive(Clone, Copy, PartialEq, Default)]
pub enum Overflow {
    #[default]
    Visible,
    Hidden,
    Scroll,
}

// interactions — attached to any element

pub struct Interactions<M> {
    pub on_click: Option<M>,
    pub on_hover: Option<M>, // fires every frame while hovered
    pub on_mouse_down: Option<M>,
}

impl<M> Default for Interactions<M> {
    fn default() -> Self {
        Self {
            on_click: None,
            on_hover: None,
            on_mouse_down: None,
        }
    }
}

// layout — sizing, flex, alignment, spacing, position
// consumed by the layout pass (taffy), not used for rendering

#[derive(Clone)]
pub struct Layout {
    // resolved position — written by layout pass, not set by user
    pub x: f32,
    pub y: f32,

    // sizing
    pub width: Val,
    pub height: Val,
    pub min_width: Val,
    pub max_width: Val,
    pub min_height: Val,
    pub max_height: Val,
    pub aspect_ratio: Option<f32>,

    // flex
    pub grow: f32,
    pub shrink: Option<f32>,
    pub basis: Val,
    pub wrap: bool,

    // alignment (on containers)
    pub align_x: Align,
    pub align_y: Align,

    // alignment (on self, overrides parent align_items)
    pub align_self: Option<Align>,

    // spacing
    pub padding: Padding,
    pub margin: Margin,
    pub gap: f32,

    // position
    pub position: Position,
    pub inset: Edges,

    // overflow
    pub overflow: Overflow,
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: Val::Auto,
            height: Val::Auto,
            min_width: Val::Auto,
            max_width: Val::Auto,
            min_height: Val::Auto,
            max_height: Val::Auto,
            aspect_ratio: None,
            grow: 0.0,
            shrink: None,
            basis: Val::Auto,
            wrap: false,
            align_x: Align::Start,
            align_y: Align::Start,
            align_self: None,
            padding: Padding::default(),
            margin: Margin::default(),
            gap: 0.0,
            position: Position::Relative,
            inset: Edges::default(),
            overflow: Overflow::Visible,
        }
    }
}

// style — purely visual properties used during rendering

#[derive(Clone)]
pub struct Style {
    pub background: Option<Color>,
    pub border_radius: f32,
    pub border_color: Option<Color>,
    pub border_thickness: f32,
    pub opacity: f32,
    pub shadow_color: Color,
    pub shadow_offset_x: f32,
    pub shadow_offset_y: f32,
    pub shadow_blur: f32,
    pub text_color: Option<Color>,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            background: None,
            border_radius: 0.0,
            border_color: None,
            border_thickness: 0.0,
            opacity: 1.0,
            shadow_color: Color::TRANSPARENT,
            shadow_offset_x: 0.0,
            shadow_offset_y: 0.0,
            shadow_blur: 0.0,
            text_color: None,
        }
    }
}

// element — thin enum, each variant wraps a widget struct

pub enum Element<M: Clone + 'static = ()> {
    Empty,
    Rect(Rect<M>),
    Text(Text<M>),
    Button(Button<M>),
    TextInput(TextInput<M>),
    Row(Row<M>),
    Column(Column<M>),
}

// forwarding methods on Element so button("label").on_click(...) etc. keep working

impl<M: Clone + 'static> Element<M> {
    // interactions
    pub fn on_click(self, msg: M) -> Self {
        match self {
            Element::Rect(w) => Element::Rect(w.on_click(msg)),
            Element::Button(w) => Element::Button(w.on_click(msg)),
            Element::TextInput(w) => Element::TextInput(w.on_click(msg)),
            Element::Row(w) => Element::Row(w.on_click(msg)),
            Element::Column(w) => Element::Column(w.on_click(msg)),
            other => other,
        }
    }
    pub fn on_hover(self, msg: M) -> Self {
        match self {
            Element::Rect(w) => Element::Rect(w.on_hover(msg)),
            Element::Button(w) => Element::Button(w.on_hover(msg)),
            Element::TextInput(w) => Element::TextInput(w.on_hover(msg)),
            Element::Row(w) => Element::Row(w.on_hover(msg)),
            Element::Column(w) => Element::Column(w.on_hover(msg)),
            other => other,
        }
    }
    pub fn on_mouse_down(self, msg: M) -> Self {
        match self {
            Element::Rect(w) => Element::Rect(w.on_mouse_down(msg)),
            Element::Button(w) => Element::Button(w.on_mouse_down(msg)),
            Element::Row(w) => Element::Row(w.on_mouse_down(msg)),
            Element::Column(w) => Element::Column(w.on_mouse_down(msg)),
            other => other,
        }
    }

    // layout
    pub fn width(self, v: Val) -> Self {
        match self {
            Element::Rect(w) => Element::Rect(w.width(v)),
            Element::Text(w) => Element::Text(w.width(v)),
            Element::Button(w) => Element::Button(w.width(v)),
            Element::TextInput(w) => Element::TextInput(w.width(v)),
            Element::Row(w) => Element::Row(w.width(v)),
            Element::Column(w) => Element::Column(w.width(v)),
            other => other,
        }
    }
    pub fn height(self, v: Val) -> Self {
        match self {
            Element::Rect(w) => Element::Rect(w.height(v)),
            Element::Button(w) => Element::Button(w.height(v)),
            Element::TextInput(w) => Element::TextInput(w.height(v)),
            Element::Row(w) => Element::Row(w.height(v)),
            Element::Column(w) => Element::Column(w.height(v)),
            other => other,
        }
    }
    pub fn min_width(self, v: Val) -> Self {
        match self {
            Element::Rect(w) => Element::Rect(w.min_width(v)),
            Element::Row(w) => Element::Row(w.min_width(v)),
            Element::Column(w) => Element::Column(w.min_width(v)),
            other => other,
        }
    }
    pub fn max_width(self, v: Val) -> Self {
        match self {
            Element::Rect(w) => Element::Rect(w.max_width(v)),
            Element::Row(w) => Element::Row(w.max_width(v)),
            Element::Column(w) => Element::Column(w.max_width(v)),
            other => other,
        }
    }
    pub fn min_height(self, v: Val) -> Self {
        match self {
            Element::Rect(w) => Element::Rect(w.min_height(v)),
            Element::Row(w) => Element::Row(w.min_height(v)),
            Element::Column(w) => Element::Column(w.min_height(v)),
            other => other,
        }
    }
    pub fn max_height(self, v: Val) -> Self {
        match self {
            Element::Rect(w) => Element::Rect(w.max_height(v)),
            Element::Row(w) => Element::Row(w.max_height(v)),
            Element::Column(w) => Element::Column(w.max_height(v)),
            other => other,
        }
    }
    pub fn grow(self, v: f32) -> Self {
        match self {
            Element::Rect(w) => Element::Rect(w.grow(v)),
            Element::Text(w) => Element::Text(w.grow(v)),
            Element::Button(w) => Element::Button(w.grow(v)),
            Element::TextInput(w) => Element::TextInput(w.grow(v)),
            Element::Row(w) => Element::Row(w.grow(v)),
            Element::Column(w) => Element::Column(w.grow(v)),
            other => other,
        }
    }
    pub fn shrink(self, v: f32) -> Self {
        match self {
            Element::Rect(w) => Element::Rect(w.shrink(v)),
            Element::Row(w) => Element::Row(w.shrink(v)),
            Element::Column(w) => Element::Column(w.shrink(v)),
            other => other,
        }
    }
    pub fn gap(self, v: f32) -> Self {
        match self {
            Element::Row(w) => Element::Row(w.gap(v)),
            Element::Column(w) => Element::Column(w.gap(v)),
            other => other,
        }
    }
    pub fn padding(self, e: Edges) -> Self {
        match self {
            Element::Rect(w) => Element::Rect(w.padding(e)),
            Element::TextInput(w) => Element::TextInput(w.padding(e)),
            Element::Row(w) => Element::Row(w.padding(e)),
            Element::Column(w) => Element::Column(w.padding(e)),
            other => other,
        }
    }
    pub fn margin(self, e: Margin) -> Self {
        match self {
            Element::Rect(w) => Element::Rect(w.margin(e)),
            Element::Text(w) => Element::Text(w.margin(e)),
            Element::Button(w) => Element::Button(w.margin(e)),
            Element::TextInput(w) => Element::TextInput(w.margin(e)),
            Element::Row(w) => Element::Row(w.margin(e)),
            Element::Column(w) => Element::Column(w.margin(e)),
            other => other,
        }
    }
    pub fn align_x(self, a: Align) -> Self {
        match self {
            Element::Row(w) => Element::Row(w.align_x(a)),
            Element::Column(w) => Element::Column(w.align_x(a)),
            other => other,
        }
    }
    pub fn align_y(self, a: Align) -> Self {
        match self {
            Element::Row(w) => Element::Row(w.align_y(a)),
            Element::Column(w) => Element::Column(w.align_y(a)),
            other => other,
        }
    }
    pub fn align_self(self, a: Align) -> Self {
        match self {
            Element::Rect(w) => Element::Rect(w.align_self(a)),
            Element::Text(w) => Element::Text(w.align_self(a)),
            Element::Button(w) => Element::Button(w.align_self(a)),
            Element::TextInput(w) => Element::TextInput(w.align_self(a)),
            Element::Row(w) => Element::Row(w.align_self(a)),
            Element::Column(w) => Element::Column(w.align_self(a)),
            other => other,
        }
    }
    pub fn absolute(self) -> Self {
        match self {
            Element::Rect(w) => Element::Rect(w.absolute()),
            Element::Row(w) => Element::Row(w.absolute()),
            Element::Column(w) => Element::Column(w.absolute()),
            other => other,
        }
    }
    pub fn inset(self, e: Edges) -> Self {
        match self {
            Element::Rect(w) => Element::Rect(w.inset(e)),
            Element::Row(w) => Element::Row(w.inset(e)),
            Element::Column(w) => Element::Column(w.inset(e)),
            other => other,
        }
    }
    pub fn overflow_hidden(self) -> Self {
        match self {
            Element::Rect(w) => Element::Rect(w.overflow_hidden()),
            Element::Row(w) => Element::Row(w.overflow_hidden()),
            Element::Column(w) => Element::Column(w.overflow_hidden()),
            other => other,
        }
    }
    pub fn overflow_scroll(self) -> Self {
        match self {
            Element::Rect(w) => Element::Rect(w.overflow_scroll()),
            Element::Row(w) => Element::Row(w.overflow_scroll()),
            Element::Column(w) => Element::Column(w.overflow_scroll()),
            other => other,
        }
    }
    pub fn wrap(self) -> Self {
        match self {
            Element::Row(w) => Element::Row(w.wrap()),
            Element::Column(w) => Element::Column(w.wrap()),
            other => other,
        }
    }

    // style
    pub fn background(self, color: Color) -> Self {
        match self {
            Element::Rect(w) => Element::Rect(w.background(color)),
            Element::Button(w) => Element::Button(w.background(color)),
            Element::TextInput(w) => Element::TextInput(w.background(color)),
            Element::Row(w) => Element::Row(w.background(color)),
            Element::Column(w) => Element::Column(w.background(color)),
            other => other,
        }
    }
    pub fn border_radius(self, v: f32) -> Self {
        match self {
            Element::Rect(w) => Element::Rect(w.border_radius(v)),
            Element::Button(w) => Element::Button(w.border_radius(v)),
            Element::TextInput(w) => Element::TextInput(w.border_radius(v)),
            Element::Row(w) => Element::Row(w.border_radius(v)),
            Element::Column(w) => Element::Column(w.border_radius(v)),
            other => other,
        }
    }
    pub fn border(self, color: Color, thickness: f32) -> Self {
        match self {
            Element::Rect(w) => Element::Rect(w.border(color, thickness)),
            Element::Button(w) => Element::Button(w.border(color, thickness)),
            Element::TextInput(w) => Element::TextInput(w.border(color, thickness)),
            Element::Row(w) => Element::Row(w.border(color, thickness)),
            Element::Column(w) => Element::Column(w.border(color, thickness)),
            other => other,
        }
    }
    pub fn opacity(self, v: f32) -> Self {
        match self {
            Element::Rect(w) => Element::Rect(w.opacity(v)),
            Element::Text(w) => Element::Text(w.opacity(v)),
            Element::Button(w) => Element::Button(w.opacity(v)),
            Element::TextInput(w) => Element::TextInput(w.opacity(v)),
            Element::Row(w) => Element::Row(w.opacity(v)),
            Element::Column(w) => Element::Column(w.opacity(v)),
            other => other,
        }
    }
    pub fn shadow(self, color: Color, offset_x: f32, offset_y: f32, blur: f32) -> Self {
        match self {
            Element::Rect(w) => Element::Rect(w.shadow(color, offset_x, offset_y, blur)),
            Element::Button(w) => Element::Button(w.shadow(color, offset_x, offset_y, blur)),
            Element::Row(w) => Element::Row(w.shadow(color, offset_x, offset_y, blur)),
            Element::Column(w) => Element::Column(w.shadow(color, offset_x, offset_y, blur)),
            other => other,
        }
    }
    pub fn text_color(self, color: Color) -> Self {
        match self {
            Element::Button(w) => Element::Button(w.text_color(color)),
            Element::TextInput(w) => Element::TextInput(w.text_color(color)),
            other => other,
        }
    }

    // text / font
    pub fn font_size(self, size: f32) -> Self {
        match self {
            Element::Text(w) => Element::Text(w.font_size(size)),
            Element::TextInput(w) => Element::TextInput(w.font_size(size)),
            other => other,
        }
    }
    pub fn font_weight(self, weight: u16) -> Self {
        match self {
            Element::Text(w) => Element::Text(w.font_weight(weight)),
            other => other,
        }
    }
    pub fn font(self, name: &str) -> Self {
        match self {
            Element::Text(w) => Element::Text(w.font(name)),
            Element::TextInput(w) => Element::TextInput(w.font(name)),
            other => other,
        }
    }
    pub fn italic(self) -> Self {
        match self {
            Element::Text(w) => Element::Text(w.italic()),
            other => other,
        }
    }
    pub fn text_align(self, align: TextAlign) -> Self {
        match self {
            Element::Text(w) => Element::Text(w.text_align(align)),
            other => other,
        }
    }

    // text input specific
    pub fn value(self, v: &str) -> Self {
        match self {
            Element::TextInput(w) => Element::TextInput(w.value(v)),
            other => other,
        }
    }
    pub fn placeholder(self, text: &str) -> Self {
        match self {
            Element::TextInput(w) => Element::TextInput(w.placeholder(text)),
            other => other,
        }
    }
    pub fn placeholder_color(self, color: Color) -> Self {
        match self {
            Element::TextInput(w) => Element::TextInput(w.placeholder_color(color)),
            other => other,
        }
    }
    pub fn on_change(self, f: impl Fn(String) -> M + 'static) -> Self {
        match self {
            Element::TextInput(w) => Element::TextInput(w.on_change(f)),
            other => other,
        }
    }
}

// constructor functions — public API, identical to before from user's perspective

pub fn empty<M: Clone + 'static>() -> Element<M> {
    Element::Empty
}

pub fn rect<M: Clone + 'static>(color: Color) -> Element<M> {
    Element::Rect(Rect::new(color))
}

pub fn text<M: Clone + 'static>(content: &str, color: Color) -> Element<M> {
    Element::Text(Text::new(content, color))
}

pub fn button<M: Clone + 'static>(label: &str) -> Element<M> {
    Element::Button(Button::new(label))
}

pub fn text_input<M: Clone + 'static>(id: &str) -> Element<M> {
    Element::TextInput(TextInput::new(id))
}

pub fn row<M: Clone + 'static>(children: Vec<Element<M>>) -> Element<M> {
    Element::Row(Row::new(children))
}

pub fn column<M: Clone + 'static>(children: Vec<Element<M>>) -> Element<M> {
    Element::Column(Column::new(children))
}

pub fn exit() {
    std::process::exit(0);
}
