use crate::element::{Element, ElementType};

pub fn layout_tree(el: &mut Element) {
    match el._type {
        ElementType::Row => {
            if let Some(children) = &mut el.children {
                let pad_left = el.style.padding[3];
                let pad_right = el.style.padding[1];
                let pad_top = el.style.padding[0];
                let mut cursor_x = el.style.x + pad_left;
                let mut row_height: f32 = 0.0;
                let gap = el.style.gap;
                let count = children.len();
                for (i, child) in children.iter_mut().enumerate() {
                    let margin_left = child.style.margin[3];
                    let margin_right = child.style.margin[1];
                    cursor_x += margin_left;
                    child.style.x = cursor_x;
                    child.style.y = el.style.y + pad_top + child.style.margin[0];
                    layout_tree(child);
                    cursor_x += child.style.w + margin_right;
                    if i < count - 1 {
                        cursor_x += gap;
                    }
                    row_height = row_height
                        .max(child.style.h + child.style.margin[0] + child.style.margin[2]);
                }
                el.style.h = row_height + el.style.padding[0] + el.style.padding[2];
                el.style.w = cursor_x - el.style.x + pad_right;
            }
        }
        ElementType::Col => {
            if let Some(children) = &mut el.children {
                let pad_top = el.style.padding[0];
                let pad_bottom = el.style.padding[2];
                let pad_left = el.style.padding[3];
                let mut cursor_y = el.style.y + pad_top;
                let mut col_width: f32 = 0.0;
                let gap = el.style.gap;
                let count = children.len();
                for (i, child) in children.iter_mut().enumerate() {
                    let margin_top = child.style.margin[0];
                    let margin_bottom = child.style.margin[2];
                    cursor_y += margin_top;
                    child.style.x = el.style.x + pad_left + child.style.margin[3];
                    child.style.y = cursor_y;
                    layout_tree(child);
                    cursor_y += child.style.h + margin_bottom;
                    if i < count - 1 {
                        cursor_y += gap;
                    }
                    col_width = col_width
                        .max(child.style.w + child.style.margin[1] + child.style.margin[3]);
                }
                el.style.h = cursor_y - el.style.y + pad_bottom;
                el.style.w = col_width + el.style.padding[1] + el.style.padding[3];
            }
        }
        _ => {}
    }
}
