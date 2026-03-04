use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ElementType {
    Rect,
    Row,
    Col,
}

pub struct Element {
    pub id: u32,
    pub _type: ElementType,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub children: Option<Vec<Element>>,
}

impl Display for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self._type)
    }
}

pub fn rect() -> Element {
    Element {
        id: 0,
        _type: ElementType::Rect,
        x: 0.0,
        y: 0.0,
        w: 100.0,
        h: 100.0,
        children: None,
    }
}

pub fn row(el: Vec<Element>) -> Element {
    Element {
        id: 0,
        _type: ElementType::Row,
        x: 0.0,
        y: 0.0,
        w: 100.0,
        h: 100.0,
        children: Some(el),
    }
}

pub fn col(el: Vec<Element>) -> Element {
    Element {
        id: 0,
        _type: ElementType::Col,
        x: 0.0,
        y: 0.0,
        w: 100.0,
        h: 100.0,
        children: Some(el),
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
