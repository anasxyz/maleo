use taffy::prelude::*;

use crate::{Align, Element, Fonts, Overflow, Position, Val};

pub fn do_layout<M: Clone + 'static>(
    element: &mut Element<M>,
    width: f32,
    height: f32,
    fonts: &mut Fonts,
) {
    let mut taffy: TaffyTree<()> = TaffyTree::new();
    let root = build_taffy_node(&mut taffy, element, fonts);
    taffy
        .compute_layout(
            root,
            taffy::geometry::Size {
                width: AvailableSpace::Definite(width),
                height: AvailableSpace::Definite(height),
            },
        )
        .unwrap();
    apply_layout(&taffy, element, root, 0.0, 0.0);
}

pub fn build_taffy_node_pub<M: Clone + 'static>(
    taffy: &mut TaffyTree<()>,
    element: &Element<M>,
    fonts: &mut Fonts,
) -> NodeId {
    build_taffy_node(taffy, element, fonts)
}

fn build_taffy_node<M: Clone + 'static>(
    taffy: &mut TaffyTree<()>,
    element: &Element<M>,
    fonts: &mut Fonts,
) -> NodeId {
    match element {
        Element::Empty => taffy.new_leaf(taffy::Style::default()).unwrap(),
        Element::Rect(r) => r.layout_node(taffy, fonts),
        Element::Text(t) => t.layout_node(taffy, fonts),
        Element::Button(b) => b.layout_node(taffy, fonts),
        Element::TextInput(t) => t.layout_node(taffy, fonts),
        Element::TextEditor(t) => t.layout_node(taffy, fonts),
        Element::Row(r) => r.layout_node(taffy, fonts),
        Element::Column(c) => c.layout_node(taffy, fonts),
    }
}

fn apply_layout<M: Clone + 'static>(
    taffy: &TaffyTree<()>,
    element: &mut Element<M>,
    node: NodeId,
    parent_x: f32,
    parent_y: f32,
) {
    let layout = taffy.layout(node).unwrap();
    let x = parent_x + layout.location.x;
    let y = parent_y + layout.location.y;
    let w = layout.size.width;
    let h = layout.size.height;

    match element {
        Element::Empty => {}
        Element::Rect(r) => r.apply_layout(x, y, w, h),
        Element::Text(t) => t.apply_layout(x, y, w, h),
        Element::Button(b) => b.apply_layout(x, y, w, h),
        Element::TextInput(t) => t.apply_layout(x, y, w, h),
        Element::TextEditor(t) => t.apply_layout(x, y, w, h),
        Element::Row(r) => {
            r.apply_layout(x, y, w, h);
            let child_nodes = taffy.children(node).unwrap();
            for (child, child_node) in r.children.iter_mut().zip(child_nodes.iter()) {
                apply_layout(taffy, child, *child_node, x, y);
            }
        }
        Element::Column(c) => {
            c.apply_layout(x, y, w, h);
            let child_nodes = taffy.children(node).unwrap();
            for (child, child_node) in c.children.iter_mut().zip(child_nodes.iter()) {
                apply_layout(taffy, child, *child_node, x, y);
            }
        }
    }
}

// shared layout helpers used by widget structs

pub fn val_to_dimension(v: &Val) -> Dimension {
    match v {
        Val::Auto => Dimension::Auto,
        Val::Px(v) => Dimension::Length(*v),
        Val::Percent(p) => Dimension::Percent(*p / 100.0),
    }
}

pub fn edges_to_rect_lp(e: &crate::Edges) -> Rect<LengthPercentage> {
    Rect {
        left: LengthPercentage::Length(e.left),
        right: LengthPercentage::Length(e.right),
        top: LengthPercentage::Length(e.top),
        bottom: LengthPercentage::Length(e.bottom),
    }
}

pub fn margin_to_rect_lpa(m: &crate::Margin) -> Rect<LengthPercentageAuto> {
    fn side(v: Option<f32>) -> LengthPercentageAuto {
        match v {
            Some(v) => LengthPercentageAuto::Length(v),
            None => LengthPercentageAuto::Auto,
        }
    }
    Rect {
        left: side(m.left),
        right: side(m.right),
        top: side(m.top),
        bottom: side(m.bottom),
    }
}

pub fn align_to_justify(a: Align) -> Option<JustifyContent> {
    Some(match a {
        Align::Start => JustifyContent::FlexStart,
        Align::Center => JustifyContent::Center,
        Align::End => JustifyContent::FlexEnd,
        Align::SpaceBetween => JustifyContent::SpaceBetween,
        Align::SpaceAround => JustifyContent::SpaceAround,
        Align::SpaceEvenly => JustifyContent::SpaceEvenly,
    })
}

pub fn align_to_items(a: Align) -> Option<AlignItems> {
    Some(match a {
        Align::Start => AlignItems::FlexStart,
        Align::Center => AlignItems::Center,
        Align::End => AlignItems::FlexEnd,
        _ => AlignItems::Stretch,
    })
}

pub fn align_to_self(a: Align) -> Option<AlignSelf> {
    Some(match a {
        Align::Start => AlignSelf::FlexStart,
        Align::Center => AlignSelf::Center,
        Align::End => AlignSelf::FlexEnd,
        _ => return None,
    })
}

pub fn overflow_to_taffy(o: Overflow) -> taffy::geometry::Point<taffy::style::Overflow> {
    let v = match o {
        Overflow::Visible => taffy::style::Overflow::Visible,
        Overflow::Hidden => taffy::style::Overflow::Hidden,
        Overflow::Scroll => taffy::style::Overflow::Scroll,
    };
    taffy::geometry::Point { x: v, y: v }
}

pub fn style_to_taffy(style: &crate::Layout, flex_direction: FlexDirection) -> taffy::Style {
    taffy::Style {
        display: Display::Flex,
        flex_direction,
        flex_wrap: if style.wrap {
            FlexWrap::Wrap
        } else {
            FlexWrap::NoWrap
        },
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
        margin: margin_to_rect_lpa(&style.margin),
        gap: taffy::geometry::Size {
            width: LengthPercentage::Length(style.gap),
            height: LengthPercentage::Length(style.gap),
        },
        align_self: style.align_self.and_then(align_to_self),
        overflow: overflow_to_taffy(style.overflow),
        ..Default::default()
    }
}
