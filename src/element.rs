use crate::Color;

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

// edges, used for padding, margin, inset, etc

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

// keep padding as an alias for backwards compat and ergonomics
pub type Padding = Edges;

// margin — each side can be a value or auto (auto = absorb remaining space, used for centering)
#[derive(Clone, Copy)]
pub struct Margin {
    pub top: Option<f32>,
    pub right: Option<f32>,
    pub bottom: Option<f32>,
    pub left: Option<f32>,
}

impl Default for Margin {
    fn default() -> Self {
        // all zero by default — None means auto which centers in flexbox
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

// style

#[derive(Clone)]
pub struct Style {
    // resolved position — set by layout, not user
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

    // visuals
    pub background: Option<Color>,
    pub border_radius: f32,
    pub border_color: Option<Color>,
    pub border_thickness: f32,
    pub opacity: f32,
    pub overflow: Overflow,
    pub shadow_color: Color,
    pub shadow_offset_x: f32,
    pub shadow_offset_y: f32,
    pub shadow_blur: f32,
    pub font: Option<String>,
    pub font_size: Option<f32>,
    pub text_color: Option<Color>,
}

impl Default for Style {
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
            background: None,
            border_radius: 0.0,
            border_color: None,
            border_thickness: 0.0,
            opacity: 1.0,
            overflow: Overflow::Visible,
            shadow_color: Color::TRANSPARENT,
            shadow_offset_x: 0.0,
            shadow_offset_y: 0.0,
            shadow_blur: 0.0,
            font: None,
            font_size: None,
            text_color: None,
        }
    }
}

// element

pub enum Element<M: Clone + 'static = ()> {
    Empty,
    Rect {
        color: Color,
        style: Style,
        interactions: Interactions<M>,
        resolved_w: f32,
        resolved_h: f32,
    },
    Text {
        content: String,
        color: Color,
        font: Option<String>,
        font_size: Option<f32>,
        font_weight: u16,
        italic: bool,
        text_align: TextAlign,
        style: Style,
        interactions: Interactions<M>,
        resolved_w: f32,
    },
    Button {
        label: String,
        style: Style,
        interactions: Interactions<M>,
        resolved_x: f32,
        resolved_y: f32,
        resolved_w: f32,
        resolved_h: f32,
    },
    TextInput {
        id: String,
        placeholder: String,
        placeholder_color: Option<Color>,
        font: Option<String>,
        font_size: Option<f32>,
        value: Option<String>,
        style: Style,
        interactions: Interactions<M>,
        on_change: Option<Box<dyn Fn(String) -> M>>,
        resolved_x: f32,
        resolved_y: f32,
        resolved_w: f32,
        resolved_h: f32,
    },
    Row {
        style: Style,
        interactions: Interactions<M>,
        children: Vec<Element<M>>,
        resolved_w: f32,
        resolved_h: f32,
    },
    Column {
        style: Style,
        interactions: Interactions<M>,
        children: Vec<Element<M>>,
        resolved_w: f32,
        resolved_h: f32,
    },
}

// builder impl

impl<M: Clone + 'static> Element<M> {
    fn style_mut(&mut self) -> Option<&mut Style> {
        match self {
            Element::Rect { style, .. } => Some(style),
            Element::Text { style, .. } => Some(style),
            Element::Row { style, .. } => Some(style),
            Element::Column { style, .. } => Some(style),
            Element::Button { style, .. } => Some(style),
            Element::TextInput { style, .. } => Some(style),
            Element::Empty => None,
        }
    }

    fn interactions_mut(&mut self) -> Option<&mut Interactions<M>> {
        match self {
            Element::Rect { interactions, .. } => Some(interactions),
            Element::Text { interactions, .. } => Some(interactions),
            Element::Row { interactions, .. } => Some(interactions),
            Element::Column { interactions, .. } => Some(interactions),
            Element::Button { interactions, .. } => Some(interactions),
            Element::TextInput { interactions, .. } => Some(interactions),
            Element::Empty => None,
        }
    }

    // interactions — work on every element
    pub fn on_click(mut self, msg: M) -> Self {
        if let Some(i) = self.interactions_mut() {
            i.on_click = Some(msg);
        }
        self
    }
    pub fn on_hover(mut self, msg: M) -> Self {
        if let Some(i) = self.interactions_mut() {
            i.on_hover = Some(msg);
        }
        self
    }
    pub fn on_mouse_down(mut self, msg: M) -> Self {
        if let Some(i) = self.interactions_mut() {
            i.on_mouse_down = Some(msg);
        }
        self
    }

    // text input specific
    pub fn on_change(mut self, f: impl Fn(String) -> M + 'static) -> Self {
        if let Element::TextInput {
            ref mut on_change, ..
        } = self
        {
            *on_change = Some(Box::new(f));
        }
        self
    }
    pub fn value(mut self, v: &str) -> Self {
        if let Element::TextInput { ref mut value, .. } = self {
            *value = Some(v.to_string());
        }
        self
    }
    pub fn placeholder(mut self, text: &str) -> Self {
        if let Element::TextInput {
            ref mut placeholder,
            ..
        } = self
        {
            *placeholder = text.to_string();
        }
        self
    }
    pub fn placeholder_color(mut self, color: Color) -> Self {
        if let Element::TextInput {
            ref mut placeholder_color,
            ..
        } = self
        {
            *placeholder_color = Some(color);
        }
        self
    }
    pub fn text_color(mut self, color: Color) -> Self {
        if let Some(s) = self.style_mut() {
            s.text_color = Some(color);
        }
        self
    }

    // sizing
    pub fn width(mut self, v: Val) -> Self {
        if let Some(s) = self.style_mut() {
            s.width = v;
        }
        self
    }
    pub fn height(mut self, v: Val) -> Self {
        if let Some(s) = self.style_mut() {
            s.height = v;
        }
        self
    }
    pub fn min_width(mut self, v: Val) -> Self {
        if let Some(s) = self.style_mut() {
            s.min_width = v;
        }
        self
    }
    pub fn max_width(mut self, v: Val) -> Self {
        if let Some(s) = self.style_mut() {
            s.max_width = v;
        }
        self
    }
    pub fn min_height(mut self, v: Val) -> Self {
        if let Some(s) = self.style_mut() {
            s.min_height = v;
        }
        self
    }
    pub fn max_height(mut self, v: Val) -> Self {
        if let Some(s) = self.style_mut() {
            s.max_height = v;
        }
        self
    }
    pub fn aspect_ratio(mut self, ratio: f32) -> Self {
        if let Some(s) = self.style_mut() {
            s.aspect_ratio = Some(ratio);
        }
        self
    }

    // flex
    pub fn grow(mut self, v: f32) -> Self {
        if let Some(s) = self.style_mut() {
            s.grow = v;
        }
        self
    }
    pub fn shrink(mut self, v: f32) -> Self {
        if let Some(s) = self.style_mut() {
            s.shrink = Some(v);
        }
        self
    }
    pub fn basis(mut self, v: Val) -> Self {
        if let Some(s) = self.style_mut() {
            s.basis = v;
        }
        self
    }
    pub fn wrap(mut self) -> Self {
        if let Some(s) = self.style_mut() {
            s.wrap = true;
        }
        self
    }

    // alignment
    pub fn align_x(mut self, a: Align) -> Self {
        if let Some(s) = self.style_mut() {
            s.align_x = a;
        }
        self
    }
    pub fn align_y(mut self, a: Align) -> Self {
        if let Some(s) = self.style_mut() {
            s.align_y = a;
        }
        self
    }
    pub fn align_self(mut self, a: Align) -> Self {
        if let Some(s) = self.style_mut() {
            s.align_self = Some(a);
        }
        self
    }

    // spacing
    pub fn padding(mut self, e: Edges) -> Self {
        if let Some(s) = self.style_mut() {
            s.padding = e;
        }
        self
    }
    pub fn margin(mut self, e: Margin) -> Self {
        if let Some(s) = self.style_mut() {
            s.margin = e;
        }
        self
    }
    pub fn gap(mut self, v: f32) -> Self {
        if let Some(s) = self.style_mut() {
            s.gap = v;
        }
        self
    }

    // position
    pub fn absolute(mut self) -> Self {
        if let Some(s) = self.style_mut() {
            s.position = Position::Absolute;
        }
        self
    }
    pub fn inset(mut self, e: Edges) -> Self {
        if let Some(s) = self.style_mut() {
            s.inset = e;
        }
        self
    }

    // visuals
    pub fn background(mut self, color: Color) -> Self {
        if let Some(s) = self.style_mut() {
            s.background = Some(color);
        }
        self
    }
    pub fn border_radius(mut self, v: f32) -> Self {
        if let Some(s) = self.style_mut() {
            s.border_radius = v;
        }
        self
    }
    pub fn border(mut self, color: Color, thickness: f32) -> Self {
        if let Some(s) = self.style_mut() {
            s.border_color = Some(color);
            s.border_thickness = thickness;
        }
        self
    }
    pub fn opacity(mut self, v: f32) -> Self {
        if let Some(s) = self.style_mut() {
            s.opacity = v;
        }
        self
    }
    pub fn shadow(mut self, color: Color, offset_x: f32, offset_y: f32, blur: f32) -> Self {
        if let Some(s) = self.style_mut() {
            s.shadow_color = color;
            s.shadow_offset_x = offset_x;
            s.shadow_offset_y = offset_y;
            s.shadow_blur = blur;
        }
        self
    }
    pub fn overflow_hidden(mut self) -> Self {
        if let Some(s) = self.style_mut() {
            s.overflow = Overflow::Hidden;
        }
        self
    }
    pub fn overflow_scroll(mut self) -> Self {
        if let Some(s) = self.style_mut() {
            s.overflow = Overflow::Scroll;
        }
        self
    }

    // font — works on any element
    pub fn font(mut self, name: &str) -> Self {
        match &mut self {
            Element::Text { font, .. } => {
                *font = Some(name.to_string());
            }
            Element::TextInput { font, .. } => {
                *font = Some(name.to_string());
            }
            other => {
                if let Some(s) = other.style_mut() {
                    s.font = Some(name.to_string());
                }
            }
        }
        self
    }
    pub fn font_size(mut self, size: f32) -> Self {
        match &mut self {
            Element::Text { font_size, .. } => {
                *font_size = Some(size);
            }
            Element::TextInput { font_size, .. } => {
                *font_size = Some(size);
            }
            other => {
                if let Some(s) = other.style_mut() {
                    s.font_size = Some(size);
                }
            }
        }
        self
    }
    pub fn font_weight(mut self, weight: u16) -> Self {
        if let Element::Text {
            ref mut font_weight,
            ..
        } = self
        {
            *font_weight = weight;
        }
        self
    }
    pub fn italic(mut self) -> Self {
        if let Element::Text { ref mut italic, .. } = self {
            *italic = true;
        }
        self
    }
    pub fn text_align(mut self, align: TextAlign) -> Self {
        if let Element::Text {
            ref mut text_align, ..
        } = self
        {
            *text_align = align;
        }
        self
    }
}

pub fn empty<M: Clone + 'static>() -> Element<M> {
    Element::Empty
}

pub fn rect<M: Clone + 'static>(color: Color) -> Element<M> {
    Element::Rect {
        color,
        style: Style::default(),
        interactions: Interactions::default(),
        resolved_w: 0.0,
        resolved_h: 0.0,
    }
}

pub fn text<M: Clone + 'static>(content: &str, color: Color) -> Element<M> {
    Element::Text {
        content: content.to_string(),
        color,
        font: None,
        font_size: None,
        font_weight: 400,
        italic: false,
        text_align: TextAlign::Left,
        style: Style::default(),
        interactions: Interactions::default(),
        resolved_w: 0.0,
    }
}

pub fn button<M: Clone + 'static>(label: &str) -> Element<M> {
    Element::Button {
        label: label.to_string(),
        style: Style::default(),
        interactions: Interactions::default(),
        resolved_x: 0.0,
        resolved_y: 0.0,
        resolved_w: 0.0,
        resolved_h: 0.0,
    }
}

pub fn text_input<M: Clone + 'static>(id: &str) -> Element<M> {
    Element::TextInput {
        id: id.to_string(),
        placeholder: String::new(),
        placeholder_color: None,
        font: None,
        font_size: None,
        value: None,
        style: Style::default(),
        interactions: Interactions::default(),
        on_change: None,
        resolved_x: 0.0,
        resolved_y: 0.0,
        resolved_w: 0.0,
        resolved_h: 0.0,
    }
}

pub fn row<M: Clone + 'static>(children: Vec<Element<M>>) -> Element<M> {
    Element::Row {
        style: Style::default(),
        interactions: Interactions::default(),
        children,
        resolved_w: 0.0,
        resolved_h: 0.0,
    }
}

pub fn column<M: Clone + 'static>(children: Vec<Element<M>>) -> Element<M> {
    Element::Column {
        style: Style::default(),
        interactions: Interactions::default(),
        children,
        resolved_w: 0.0,
        resolved_h: 0.0,
    }
}

pub fn exit() {
    std::process::exit(0);
}
