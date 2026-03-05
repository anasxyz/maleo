use crate::color::Color;
use std::fmt::{Display, Formatter};

pub const AUTO: f32 = f32::NAN;

// enums

#[derive(Debug, Clone)]
pub enum Size {
    Fixed(f32),
    Percent(f32),
    Auto,
}

impl Default for Size {
    fn default() -> Self {
        Size::Auto
    }
}

impl Size {
    pub fn resolve(&self, parent: f32) -> f32 {
        match self {
            Size::Fixed(v) => *v,
            Size::Percent(p) => parent * p / 100.0,
            Size::Auto => 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ElementType {
    Row,
    Col,
    Rect,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Position {
    Relative,
    Absolute,
}

impl Default for Position {
    fn default() -> Self {
        Position::Relative
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Overflow {
    Visible,
    Hidden,
    Scroll,
}

impl Default for Overflow {
    fn default() -> Self {
        Overflow::Hidden
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AlignItems {
    Start,
    Center,
    End,
    Stretch,
    Baseline,
}

impl Default for AlignItems {
    fn default() -> Self {
        AlignItems::Stretch
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AlignSelf {
    Auto,
    Start,
    Center,
    End,
    Stretch,
    Baseline,
}

impl Default for AlignSelf {
    fn default() -> Self {
        AlignSelf::Auto
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JustifyContent {
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

impl Default for JustifyContent {
    fn default() -> Self {
        JustifyContent::Start
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

impl Default for FlexWrap {
    fn default() -> Self {
        FlexWrap::NoWrap
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FlexDirection {
    Row,
    Col,
    RowReverse,
    ColReverse,
}

impl Default for FlexDirection {
    fn default() -> Self {
        FlexDirection::Row
    }
}

// element style

#[derive(Debug, Clone)]
pub struct ElementStyle {
    // computed by layout
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,

    // sizing
    pub width: Size,
    pub height: Size,
    pub min_w: Size,
    pub min_h: Size,
    pub max_w: Size,
    pub max_h: Size,
    pub aspect_ratio: Option<f32>,

    // spacing
    pub padding: [f32; 4],  // [top, right, bottom, left]
    pub margin: [f32; 4],   // [top, right, bottom, left]

    // gap
    pub row_gap: f32,
    pub col_gap: f32,

    // flex container
    pub flex_direction: FlexDirection,
    pub align_items: AlignItems,
    pub justify_content: JustifyContent,
    pub flex_wrap: FlexWrap,

    // flex child
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub flex_basis: Size,
    pub align_self: AlignSelf,

    // overflow
    pub overflow_x: Overflow,
    pub overflow_y: Overflow,

    // position
    pub position: Position,
    pub inset: [Size; 4],   // [top, right, bottom, left]

    // visual
    pub fill: Color,
    pub border_thickness: f32,
    pub border_color: Option<Color>,
    pub border_radius: Option<f32>,
    pub opacity: f32,
    pub z_index: i32,
    pub visible: bool,
}

impl Default for ElementStyle {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            w: 0.0,
            h: 0.0,
            width: Size::Auto,
            height: Size::Auto,
            min_w: Size::Fixed(0.0),
            min_h: Size::Fixed(0.0),
            max_w: Size::Auto,
            max_h: Size::Auto,
            aspect_ratio: None,
            padding: [0.0; 4],
            margin: [0.0; 4],
            row_gap: 0.0,
            col_gap: 0.0,
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Stretch,
            justify_content: JustifyContent::Start,
            flex_wrap: FlexWrap::NoWrap,
            flex_grow: 0.0,
            flex_shrink: 1.0,
            flex_basis: Size::Auto,
            align_self: AlignSelf::Auto,
            overflow_x: Overflow::Hidden,
            overflow_y: Overflow::Hidden,
            position: Position::Relative,
            inset: [Size::Auto, Size::Auto, Size::Auto, Size::Auto],
            fill: Color::new(0.0, 0.0, 0.0, 1.0),
            border_thickness: 0.0,
            border_color: None,
            border_radius: None,
            opacity: 1.0,
            z_index: 0,
            visible: true,
        }
    }
}

// element

pub struct Element {
    pub id: u32,
    pub _type: ElementType,
    pub style: ElementStyle,
    pub children: Option<Vec<Element>>,
}

impl Display for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self._type)
    }
}

impl Default for Element {
    fn default() -> Self {
        Self {
            id: 0,
            _type: ElementType::Rect,
            style: ElementStyle::default(),
            children: None,
        }
    }
}

impl Element {
    // sizing
    pub fn w(mut self, w: Size) -> Self { self.style.width = w; self }
    pub fn h(mut self, h: Size) -> Self { self.style.height = h; self }
    pub fn min_w(mut self, v: Size) -> Self { self.style.min_w = v; self }
    pub fn min_h(mut self, v: Size) -> Self { self.style.min_h = v; self }
    pub fn max_w(mut self, v: Size) -> Self { self.style.max_w = v; self }
    pub fn max_h(mut self, v: Size) -> Self { self.style.max_h = v; self }
    pub fn aspect_ratio(mut self, v: f32) -> Self { self.style.aspect_ratio = Some(v); self }

    // padding
    pub fn p(mut self, v: [f32; 4]) -> Self { self.style.padding = v; self }
    pub fn pt(mut self, v: f32) -> Self { self.style.padding[0] = v; self }
    pub fn pr(mut self, v: f32) -> Self { self.style.padding[1] = v; self }
    pub fn pb(mut self, v: f32) -> Self { self.style.padding[2] = v; self }
    pub fn pl(mut self, v: f32) -> Self { self.style.padding[3] = v; self }
    pub fn px(mut self, v: f32) -> Self { self.style.padding[1] = v; self.style.padding[3] = v; self }
    pub fn py(mut self, v: f32) -> Self { self.style.padding[0] = v; self.style.padding[2] = v; self }

    // margin
    pub fn m(mut self, v: [f32; 4]) -> Self { self.style.margin = v; self }
    pub fn mt(mut self, v: f32) -> Self { self.style.margin[0] = v; self }
    pub fn mr(mut self, v: f32) -> Self { self.style.margin[1] = v; self }
    pub fn mb(mut self, v: f32) -> Self { self.style.margin[2] = v; self }
    pub fn ml(mut self, v: f32) -> Self { self.style.margin[3] = v; self }
    pub fn mx(mut self, v: f32) -> Self { self.style.margin[1] = v; self.style.margin[3] = v; self }
    pub fn my(mut self, v: f32) -> Self { self.style.margin[0] = v; self.style.margin[2] = v; self }

    // gap
    pub fn gap(mut self, v: f32) -> Self { self.style.row_gap = v; self.style.col_gap = v; self }
    pub fn row_gap(mut self, v: f32) -> Self { self.style.row_gap = v; self }
    pub fn col_gap(mut self, v: f32) -> Self { self.style.col_gap = v; self }

    // flex container
    pub fn flex_direction(mut self, v: FlexDirection) -> Self { self.style.flex_direction = v; self }
    pub fn align_items(mut self, v: AlignItems) -> Self { self.style.align_items = v; self }
    pub fn justify_content(mut self, v: JustifyContent) -> Self { self.style.justify_content = v; self }
    pub fn flex_wrap(mut self, v: FlexWrap) -> Self { self.style.flex_wrap = v; self }

    // flex child
    pub fn grow(mut self, v: f32) -> Self { self.style.flex_grow = v; self }
    pub fn shrink(mut self, v: f32) -> Self { self.style.flex_shrink = v; self }
    pub fn basis(mut self, v: Size) -> Self { self.style.flex_basis = v; self }
    pub fn align_self(mut self, v: AlignSelf) -> Self { self.style.align_self = v; self }

    // overflow
    pub fn overflow(mut self, v: Overflow) -> Self { self.style.overflow_x = v.clone(); self.style.overflow_y = v; self }
    pub fn overflow_x(mut self, v: Overflow) -> Self { self.style.overflow_x = v; self }
    pub fn overflow_y(mut self, v: Overflow) -> Self { self.style.overflow_y = v; self }

    // position
    pub fn absolute(mut self) -> Self { self.style.position = Position::Absolute; self }
    pub fn relative(mut self) -> Self { self.style.position = Position::Relative; self }
    pub fn top(mut self, v: Size) -> Self { self.style.inset[0] = v; self }
    pub fn right(mut self, v: Size) -> Self { self.style.inset[1] = v; self }
    pub fn bottom(mut self, v: Size) -> Self { self.style.inset[2] = v; self }
    pub fn left(mut self, v: Size) -> Self { self.style.inset[3] = v; self }

    // visual
    pub fn bg(mut self, color: Color) -> Self { self.style.fill = color; self }
    pub fn border(mut self, thickness: f32) -> Self { self.style.border_thickness = thickness; self }
    pub fn border_color(mut self, color: Color) -> Self { self.style.border_color = Some(color); self }
    pub fn border_radius(mut self, radius: f32) -> Self { self.style.border_radius = Some(radius); self }
    pub fn opacity(mut self, v: f32) -> Self { self.style.opacity = v; self }
    pub fn z_index(mut self, v: i32) -> Self { self.style.z_index = v; self }
    pub fn hide(mut self) -> Self { self.style.visible = false; self }
    pub fn show(mut self) -> Self { self.style.visible = true; self }
}

// constructors

pub fn rect() -> Element {
    Element { _type: ElementType::Rect, ..Default::default() }
}

pub fn row(children: Vec<Element>) -> Element {
    Element { _type: ElementType::Row, children: Some(children), ..Default::default() }
}

pub fn col(children: Vec<Element>) -> Element {
    Element { _type: ElementType::Col, children: Some(children), ..Default::default() }
}

// debug

pub fn print_tree(el: &Element, prefix: &str, last: bool) {
    let connector = if last { "└── " } else { "├── " };
    println!("{}{}{}", prefix, connector, el);
    if let Some(children) = &el.children {
        let extension = if last { "    " } else { "│   " };
        let new_prefix = format!("{}{}", prefix, extension);
        let count = children.len();
        for (i, child) in children.iter().enumerate() {
            print_tree(child, &new_prefix, i == count - 1);
        }
    }
}

pub fn print_root(el: &Element) {
    println!("{}", el);
    if let Some(children) = &el.children {
        let count = children.len();
        for (i, child) in children.iter().enumerate() {
            print_tree(child, "", i == count - 1);
        }
    }
}
