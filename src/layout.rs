use crate::element::{Element, ElementType};

pub fn layout_tree(el: &mut Element) {
    match el._type {
        ElementType::Rect => {}
        ElementType::Row => {
            if let Some(children) = &mut el.children {
                let mut cursor_x = el.style.x;
                let mut row_height: f32 = 0.0;
                for child in children {
                    child.style.x = cursor_x;
                    child.style.y = el.style.y;
                    layout_tree(child);
                    cursor_x += child.style.w;
                    row_height = row_height.max(child.style.h);
                }
                el.style.h = row_height;
            }
        }
        ElementType::Col => {
            if let Some(children) = &mut el.children {
                let mut cursor_y = el.style.y;
                let mut col_width: f32 = 0.0;
                for child in children {
                    child.style.x = el.style.x;
                    child.style.y = cursor_y;
                    layout_tree(child);
                    cursor_y += child.style.h;
                    col_width = col_width.max(child.style.w);
                }
                el.style.w = col_width;
            }
        }
    }
}
