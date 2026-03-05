use crate::element::{Element, ElementType};

pub fn layout_tree(el: &mut Element) {
    match el._type {
        ElementType::Row => {
            if let Some(children) = &mut el.children {
                let pad_left = el.style.padding[3];
                let pad_right = el.style.padding[1];
                let pad_top = el.style.padding[0];
                let pad_bottom = el.style.padding[2];
                let gap = el.style.gap;
                let count = children.len();

                let available = el.style.w - pad_left - pad_right;

                // first pass: calculate total used space and count auto margins
                let total_used: f32 = children
                    .iter()
                    .map(|c| {
                        let ml = if c.style.margin[3].is_nan() {
                            0.0
                        } else {
                            c.style.margin[3]
                        };
                        let mr = if c.style.margin[1].is_nan() {
                            0.0
                        } else {
                            c.style.margin[1]
                        };
                        c.style.w + ml + mr
                    })
                    .sum::<f32>()
                    + gap * (count.saturating_sub(1)) as f32;

                let leftover = (available - total_used).max(0.0);

                let auto_count: u32 = children
                    .iter()
                    .map(|c| c.style.margin[3].is_nan() as u32 + c.style.margin[1].is_nan() as u32)
                    .sum();

                let auto_size = if auto_count > 0 {
                    leftover / auto_count as f32
                } else {
                    0.0
                };

                let mut cursor_x = el.style.x + pad_left;
                let mut row_height: f32 = 0.0;

                for (i, child) in children.iter_mut().enumerate() {
                    let margin_left = if child.style.margin[3].is_nan() {
                        auto_size
                    } else {
                        child.style.margin[3]
                    };
                    let margin_right = if child.style.margin[1].is_nan() {
                        auto_size
                    } else {
                        child.style.margin[1]
                    };
                    let margin_top = if child.style.margin[0].is_nan() {
                        0.0
                    } else {
                        child.style.margin[0]
                    };
                    let margin_bottom = if child.style.margin[2].is_nan() {
                        0.0
                    } else {
                        child.style.margin[2]
                    };

                    cursor_x += margin_left;
                    child.style.x = cursor_x;
                    child.style.y = el.style.y + pad_top + margin_top;
                    layout_tree(child);
                    cursor_x += child.style.w + margin_right;
                    if i < count - 1 {
                        cursor_x += gap;
                    }
                    row_height = row_height.max(child.style.h + margin_top + margin_bottom);
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
                let gap = el.style.gap;
                let count = children.len();

                let available = el.style.h - pad_top - pad_bottom;

                // first pass: calculate total used space and count auto margins
                let total_used: f32 = children
                    .iter()
                    .map(|c| {
                        let mt = if c.style.margin[0].is_nan() {
                            0.0
                        } else {
                            c.style.margin[0]
                        };
                        let mb = if c.style.margin[2].is_nan() {
                            0.0
                        } else {
                            c.style.margin[2]
                        };
                        c.style.h + mt + mb
                    })
                    .sum::<f32>()
                    + gap * (count.saturating_sub(1)) as f32;

                let leftover = (available - total_used).max(0.0);

                let auto_count: u32 = children
                    .iter()
                    .map(|c| c.style.margin[0].is_nan() as u32 + c.style.margin[2].is_nan() as u32)
                    .sum();

                let auto_size = if auto_count > 0 {
                    leftover / auto_count as f32
                } else {
                    0.0
                };

                let mut cursor_y = el.style.y + pad_top;
                let mut col_width: f32 = 0.0;

                for (i, child) in children.iter_mut().enumerate() {
                    let margin_top = if child.style.margin[0].is_nan() {
                        auto_size
                    } else {
                        child.style.margin[0]
                    };
                    let margin_bottom = if child.style.margin[2].is_nan() {
                        auto_size
                    } else {
                        child.style.margin[2]
                    };
                    let margin_left = if child.style.margin[3].is_nan() {
                        0.0
                    } else {
                        child.style.margin[3]
                    };
                    let margin_right = if child.style.margin[1].is_nan() {
                        0.0
                    } else {
                        child.style.margin[1]
                    };

                    cursor_y += margin_top;
                    child.style.x = el.style.x + pad_left + margin_left;
                    child.style.y = cursor_y;
                    layout_tree(child);
                    cursor_y += child.style.h + margin_bottom;
                    if i < count - 1 {
                        cursor_y += gap;
                    }
                    col_width = col_width.max(child.style.w + margin_left + margin_right);
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
