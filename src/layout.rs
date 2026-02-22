use taffy::prelude::*;

use crate::{Align, Element, Fonts, Font, Overflow, Position, Val};

pub fn do_layout(element: &mut Element, width: f32, height: f32, fonts: &mut Fonts) {
    let mut taffy: TaffyTree<()> = TaffyTree::new();
    let root = build_taffy_node(&mut taffy, element, fonts);
    taffy.compute_layout(
        root,
        taffy::geometry::Size {
            width: AvailableSpace::Definite(width),
            height: AvailableSpace::Definite(height),
        },
    ).unwrap();
    apply_layout(&taffy, element, root, 0.0, 0.0);
}

fn build_taffy_node(taffy: &mut TaffyTree<()>, element: &Element, fonts: &mut Fonts) -> NodeId {
    match element {
        Element::Empty => taffy.new_leaf(taffy::Style::default()).unwrap(),

        Element::Text { content, font, style, .. } => {
            let font_id = match font {
                Font::Name(name) => fonts.get_by_name(name).or_else(|| fonts.default()),
                Font::Default => fonts.default(),
            };
            let (w, h) = fonts.measure(content, font_id.unwrap());
            taffy.new_leaf(taffy::Style {
                size: taffy::geometry::Size {
                    width: Dimension::Length(w),
                    height: Dimension::Length(h),
                },
                margin: edges_to_rect_lpa(&style.margin),
                flex_grow: style.grow,
                flex_shrink: 1.0,
                align_self: style.align_self.and_then(align_to_self),
                ..Default::default()
            }).unwrap()
        }

        Element::Rect { style, .. } => {
            let mut ts = style_to_taffy(style, FlexDirection::Row);
            ts.justify_content = None;
            ts.align_items = None;
            taffy.new_leaf(ts).unwrap()
        }

        Element::Button { label, style, .. } => {
            let font_id = fonts.default().unwrap();
            let (tw, th) = fonts.measure(label, font_id);
            let natural_w = tw + 24.0;
            let natural_h = th + 12.0;
            taffy.new_leaf(taffy::Style {
                size: taffy::geometry::Size {
                    width: match &style.width {
                        Val::Auto => Dimension::Length(natural_w),
                        other => val_to_dimension(other),
                    },
                    height: match &style.height {
                        Val::Auto => Dimension::Length(natural_h),
                        other => val_to_dimension(other),
                    },
                },
                margin: edges_to_rect_lpa(&style.margin),
                flex_grow: style.grow,
                flex_shrink: 1.0,
                align_self: style.align_self.and_then(align_to_self),
                ..Default::default()
            }).unwrap()
        }

        Element::Row { style, children, .. } => {
            let child_nodes: Vec<NodeId> = children
                .iter()
                .map(|c| build_taffy_node(taffy, c, fonts))
                .collect();
            let mut ts = style_to_taffy(style, FlexDirection::Row);
            ts.justify_content = align_to_justify(style.align_x);
            ts.align_items = align_to_items(style.align_y);
            taffy.new_with_children(ts, &child_nodes).unwrap()
        }

        Element::Column { style, children, .. } => {
            let child_nodes: Vec<NodeId> = children
                .iter()
                .map(|c| build_taffy_node(taffy, c, fonts))
                .collect();
            let mut ts = style_to_taffy(style, FlexDirection::Column);
            ts.justify_content = align_to_justify(style.align_y);
            ts.align_items = align_to_items(style.align_x);
            taffy.new_with_children(ts, &child_nodes).unwrap()
        }
    }
}

fn apply_layout(taffy: &TaffyTree<()>, element: &mut Element, node: NodeId, parent_x: f32, parent_y: f32) {
    let layout = taffy.layout(node).unwrap();
    let x = parent_x + layout.location.x;
    let y = parent_y + layout.location.y;
    let w = layout.size.width;
    let h = layout.size.height;

    match element {
        Element::Empty => {}
        Element::Text { style, .. } => {
            style.x = x;
            style.y = y;
        }
        Element::Rect { style, resolved_w, resolved_h, .. } => {
            style.x = x;
            style.y = y;
            *resolved_w = w;
            *resolved_h = h;
        }
        Element::Button { resolved_x, resolved_y, resolved_w, resolved_h, .. } => {
            *resolved_x = x;
            *resolved_y = y;
            *resolved_w = w;
            *resolved_h = h;
        }
        Element::Row { style, children, resolved_w, resolved_h } => {
            style.x = x;
            style.y = y;
            *resolved_w = w;
            *resolved_h = h;
            let child_nodes = taffy.children(node).unwrap();
            for (child, child_node) in children.iter_mut().zip(child_nodes.iter()) {
                apply_layout(taffy, child, *child_node, x, y);
            }
        }
        Element::Column { style, children, resolved_w, resolved_h } => {
            style.x = x;
            style.y = y;
            *resolved_w = w;
            *resolved_h = h;
            let child_nodes = taffy.children(node).unwrap();
            for (child, child_node) in children.iter_mut().zip(child_nodes.iter()) {
                apply_layout(taffy, child, *child_node, x, y);
            }
        }
    }
}

// conversion helpers

fn val_to_dimension(v: &Val) -> Dimension {
    match v {
        Val::Auto => Dimension::Auto,
        Val::Px(v) => Dimension::Length(*v),
        Val::Percent(p) => Dimension::Percent(*p / 100.0),
    }
}

fn val_to_lpa(v: &Val) -> LengthPercentageAuto {
    match v {
        Val::Auto => LengthPercentageAuto::Auto,
        Val::Px(v) => LengthPercentageAuto::Length(*v),
        Val::Percent(p) => LengthPercentageAuto::Percent(*p / 100.0),
    }
}

fn edges_to_rect_lp(e: &crate::Edges) -> Rect<LengthPercentage> {
    Rect {
        left: LengthPercentage::Length(e.left),
        right: LengthPercentage::Length(e.right),
        top: LengthPercentage::Length(e.top),
        bottom: LengthPercentage::Length(e.bottom),
    }
}

fn edges_to_rect_lpa(e: &crate::Edges) -> Rect<LengthPercentageAuto> {
    Rect {
        left: LengthPercentageAuto::Length(e.left),
        right: LengthPercentageAuto::Length(e.right),
        top: LengthPercentageAuto::Length(e.top),
        bottom: LengthPercentageAuto::Length(e.bottom),
    }
}

fn align_to_justify(a: Align) -> Option<JustifyContent> {
    Some(match a {
        Align::Start => JustifyContent::FlexStart,
        Align::Center => JustifyContent::Center,
        Align::End => JustifyContent::FlexEnd,
        Align::SpaceBetween => JustifyContent::SpaceBetween,
        Align::SpaceAround => JustifyContent::SpaceAround,
        Align::SpaceEvenly => JustifyContent::SpaceEvenly,
    })
}

fn align_to_items(a: Align) -> Option<AlignItems> {
    Some(match a {
        Align::Start => AlignItems::FlexStart,
        Align::Center => AlignItems::Center,
        Align::End => AlignItems::FlexEnd,
        _ => AlignItems::Stretch,
    })
}

fn align_to_self(a: Align) -> Option<AlignSelf> {
    Some(match a {
        Align::Start => AlignSelf::FlexStart,
        Align::Center => AlignSelf::Center,
        Align::End => AlignSelf::FlexEnd,
        _ => return None,
    })
}

fn overflow_to_taffy(o: Overflow) -> taffy::geometry::Point<taffy::style::Overflow> {
    let v = match o {
        Overflow::Visible => taffy::style::Overflow::Visible,
        Overflow::Hidden => taffy::style::Overflow::Hidden,
        Overflow::Scroll => taffy::style::Overflow::Scroll,
    };
    taffy::geometry::Point { x: v, y: v }
}

fn style_to_taffy(style: &crate::Style, flex_direction: FlexDirection) -> taffy::Style {
    taffy::Style {
        display: Display::Flex,
        flex_direction,
        flex_wrap: if style.wrap { FlexWrap::Wrap } else { FlexWrap::NoWrap },
        position: match style.position {
            Position::Relative => taffy::style::Position::Relative,
            Position::Absolute => taffy::style::Position::Absolute,
        },
        inset: Rect {
            left: LengthPercentageAuto::Length(style.inset.left),
            right: LengthPercentageAuto::Length(style.inset.right),
            top: LengthPercentageAuto::Length(style.inset.top),
            bottom: LengthPercentageAuto::Length(style.inset.bottom),
        },
        size: taffy::geometry::Size {
            width: val_to_dimension(&style.width),
            height: val_to_dimension(&style.height),
        },
        min_size: taffy::geometry::Size {
            width: val_to_dimension(&style.min_width),
            height: val_to_dimension(&style.min_height),
        },
        max_size: taffy::geometry::Size {
            width: val_to_dimension(&style.max_width),
            height: val_to_dimension(&style.max_height),
        },
        aspect_ratio: style.aspect_ratio,
        flex_grow: style.grow,
        flex_shrink: style.shrink.unwrap_or(1.0),
        flex_basis: val_to_dimension(&style.basis),
        padding: edges_to_rect_lp(&style.padding),
        margin: edges_to_rect_lpa(&style.margin),
        gap: taffy::geometry::Size {
            width: LengthPercentage::Length(style.gap),
            height: LengthPercentage::Length(style.gap),
        },
        align_self: style.align_self.and_then(align_to_self),
        overflow: overflow_to_taffy(style.overflow),
        ..Default::default()
    }
}
