use crate::element::{Element, ElementType};

pub fn layout_tree(el: &mut Element) {
    match el._type {
        ElementType::Row => {
            if let Some(children) = &mut el.children {
                let pad_left = el.style.padding[3];
                let pad_right = el.style.padding[1];
                let pad_top = el.style.padding[0];
                let pad_bottom = el.style.padding[2];

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

                let natural_w = cursor_x - el.style.x + pad_right;
                let natural_h = row_height + pad_top + pad_bottom;
                el.style.w = natural_w.max(el.style.min_w).min(if el.style.max_w > 0.0 {
                    el.style.max_w
                } else {
                    f32::MAX
                });
                el.style.h = natural_h.max(el.style.min_h).min(if el.style.max_h > 0.0 {
                    el.style.max_h
                } else {
                    f32::MAX
                });
            }
        }
        ElementType::Col => {
            if let Some(children) = &mut el.children {
                let pad_top = el.style.padding[0];
                let pad_bottom = el.style.padding[2];
                let pad_left = el.style.padding[3];
                let pad_right = el.style.padding[1];

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

                let natural_w = col_width + pad_left + pad_right;
                let natural_h = cursor_y - el.style.y + pad_bottom;
                el.style.w = natural_w.max(el.style.min_w).min(if el.style.max_w > 0.0 {
                    el.style.max_w
                } else {
                    f32::MAX
                });
                el.style.h = natural_h.max(el.style.min_h).min(if el.style.max_h > 0.0 {
                    el.style.max_h
                } else {
                    f32::MAX
                });
            }
        }
        _ => {}
    }
}
