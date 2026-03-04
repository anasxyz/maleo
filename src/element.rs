use std::fmt::{Display, Formatter};
use crate::color::Color;

#[derive(Debug)]
pub enum ElementType {
    Container,
    Rect,
}

#[derive(Debug)]
pub struct ElementStyle {
    // positional / layout
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub min_w: f32,
    pub min_h: f32,
    pub max_w: f32,
    pub max_h: f32,
    pub padding: [f32; 4],
    pub margin: [f32; 4],

    // visual
    pub fill: Color,
    pub border_thickness: f32,
    pub border_color: Option<Color>,
    pub border_radius: Option<f32>,
}

impl Default for ElementStyle {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            w: 100.0,
            h: 100.0,
            min_w: 0.0,
            min_h: 0.0,
            max_w: 0.0,
            max_h: 0.0,
            padding: [0.0; 4],
            margin: [0.0; 4],
            fill: Color::new(0.0, 0.0, 0.0, 1.0),
            border_thickness: 0.0,
            border_color: None,
            border_radius: None,
        }
    }
}

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

pub fn rect() -> Element {
    Element {
        _type: ElementType::Rect,
        ..Default::default()
    }
}

pub fn container(el: Vec<Element>) -> Element {
    Element {
        _type: ElementType::Container,
        children: Some(el),
        ..Default::default()
    }
}

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
