use crate::element::{
    AlignItems, AlignSelf, Element, ElementType, FlexDirection, FlexWrap, JustifyContent, Overflow,
    Position, Size,
};
use taffy::prelude::*;

fn to_dimension(size: &Size) -> Dimension {
    match size {
        Size::Fixed(v) => Dimension::from_length(*v),
        Size::Percent(p) => Dimension::from_percent(*p / 100.0),
        Size::Auto => Dimension::AUTO,
    }
}

fn to_lp(v: f32) -> LengthPercentage {
    LengthPercentage::length(v)
}

fn to_lpa(v: f32) -> LengthPercentageAuto {
    LengthPercentageAuto::length(v)
}

fn to_lpa_auto(v: f32) -> LengthPercentageAuto {
    if v.is_nan() {
        LengthPercentageAuto::auto()
    } else {
        LengthPercentageAuto::length(v)
    }
}

fn to_lpa_size(size: &Size) -> LengthPercentageAuto {
    match size {
        Size::Fixed(v) => LengthPercentageAuto::length(*v),
        Size::Percent(p) => LengthPercentageAuto::percent(*p / 100.0),
        Size::Auto => LengthPercentageAuto::auto(),
    }
}

fn size_2d(w: &Size, h: &Size) -> taffy::geometry::Size<Dimension> {
    taffy::geometry::Size {
        width: to_dimension(w),
        height: to_dimension(h),
    }
}

fn padding_rect(p: &[f32; 4]) -> taffy::geometry::Rect<LengthPercentage> {
    taffy::geometry::Rect {
        top: to_lp(p[0]),
        right: to_lp(p[1]),
        bottom: to_lp(p[2]),
        left: to_lp(p[3]),
    }
}

fn margin_rect(m: &[f32; 4]) -> taffy::geometry::Rect<LengthPercentageAuto> {
    taffy::geometry::Rect {
        top: to_lpa_auto(m[0]),
        right: to_lpa_auto(m[1]),
        bottom: to_lpa_auto(m[2]),
        left: to_lpa_auto(m[3]),
    }
}

fn inset_rect(inset: &[Size; 4]) -> taffy::geometry::Rect<LengthPercentageAuto> {
    taffy::geometry::Rect {
        top: to_lpa_size(&inset[0]),
        right: to_lpa_size(&inset[1]),
        bottom: to_lpa_size(&inset[2]),
        left: to_lpa_size(&inset[3]),
    }
}

fn map_align_items(a: &AlignItems) -> Option<taffy::AlignItems> {
    Some(match a {
        AlignItems::Start => taffy::AlignItems::Start,
        AlignItems::Center => taffy::AlignItems::Center,
        AlignItems::End => taffy::AlignItems::End,
        AlignItems::Stretch => taffy::AlignItems::Stretch,
        AlignItems::Baseline => taffy::AlignItems::Baseline,
    })
}

fn map_align_self(a: &AlignSelf) -> Option<taffy::AlignSelf> {
    match a {
        AlignSelf::Auto => None,
        AlignSelf::Start => Some(taffy::AlignSelf::Start),
        AlignSelf::Center => Some(taffy::AlignSelf::Center),
        AlignSelf::End => Some(taffy::AlignSelf::End),
        AlignSelf::Stretch => Some(taffy::AlignSelf::Stretch),
        AlignSelf::Baseline => Some(taffy::AlignSelf::Baseline),
    }
}

fn map_justify(j: &JustifyContent) -> Option<taffy::JustifyContent> {
    Some(match j {
        JustifyContent::Start => taffy::JustifyContent::Start,
        JustifyContent::Center => taffy::JustifyContent::Center,
        JustifyContent::End => taffy::JustifyContent::End,
        JustifyContent::SpaceBetween => taffy::JustifyContent::SpaceBetween,
        JustifyContent::SpaceAround => taffy::JustifyContent::SpaceAround,
        JustifyContent::SpaceEvenly => taffy::JustifyContent::SpaceEvenly,
    })
}

fn map_flex_wrap(w: &FlexWrap) -> taffy::FlexWrap {
    match w {
        FlexWrap::NoWrap => taffy::FlexWrap::NoWrap,
        FlexWrap::Wrap => taffy::FlexWrap::Wrap,
        FlexWrap::WrapReverse => taffy::FlexWrap::WrapReverse,
    }
}

fn map_flex_direction(el_type: &ElementType, style_dir: &FlexDirection) -> taffy::FlexDirection {
    match el_type {
        ElementType::Row => taffy::FlexDirection::Row,
        ElementType::Col => taffy::FlexDirection::Column,
        ElementType::Rect => match style_dir {
            FlexDirection::Row => taffy::FlexDirection::Row,
            FlexDirection::Col => taffy::FlexDirection::Column,
            FlexDirection::RowReverse => taffy::FlexDirection::RowReverse,
            FlexDirection::ColReverse => taffy::FlexDirection::ColumnReverse,
        },
    }
}

fn map_overflow(o: &Overflow) -> taffy::Overflow {
    match o {
        Overflow::Visible => taffy::Overflow::Visible,
        Overflow::Hidden => taffy::Overflow::Hidden,
        Overflow::Scroll => taffy::Overflow::Scroll,
    }
}

fn map_position(p: &Position) -> taffy::Position {
    match p {
        Position::Relative => taffy::Position::Relative,
        Position::Absolute => taffy::Position::Absolute,
    }
}

fn build_style(el: &Element) -> Style {
    let s = &el.style;
    Style {
        display: Display::Flex,
        position: map_position(&s.position),
        flex_direction: map_flex_direction(&el._type, &s.flex_direction),
        flex_wrap: map_flex_wrap(&s.flex_wrap),
        align_items: map_align_items(&s.align_items),
        align_self: map_align_self(&s.align_self),
        justify_content: map_justify(&s.justify_content),
        flex_grow: s.flex_grow,
        flex_shrink: s.flex_shrink,
        flex_basis: to_dimension(&s.flex_basis),
        size: size_2d(&s.width, &s.height),
        min_size: size_2d(&s.min_w, &s.min_h),
        max_size: size_2d(&s.max_w, &s.max_h),
        aspect_ratio: s.aspect_ratio,
        padding: padding_rect(&s.padding),
        margin: margin_rect(&s.margin),
        inset: inset_rect(&s.inset),
        gap: taffy::geometry::Size {
            width: to_lp(s.col_gap),
            height: to_lp(s.row_gap),
        },
        overflow: taffy::geometry::Point {
            x: map_overflow(&s.overflow_x),
            y: map_overflow(&s.overflow_y),
        },
        ..Style::DEFAULT
    }
}

fn add_node(el: &Element, taffy: &mut TaffyTree<()>) -> NodeId {
    let style = build_style(el);
    if let Some(children) = &el.children {
        let ids: Vec<NodeId> = children.iter().map(|c| add_node(c, taffy)).collect();
        taffy.new_with_children(style, &ids).unwrap()
    } else {
        taffy.new_leaf(style).unwrap()
    }
}

fn write_back(el: &mut Element, taffy: &TaffyTree<()>, node: NodeId, parent_x: f32, parent_y: f32) {
    let layout = taffy.layout(node).unwrap();
    el.style.x = parent_x + layout.location.x;
    el.style.y = parent_y + layout.location.y;
    el.style.w = layout.size.width;
    el.style.h = layout.size.height;

    if let Some(children) = &mut el.children {
        let child_ids = taffy.children(node).unwrap();
        for (child, id) in children.iter_mut().zip(child_ids.iter()) {
            write_back(child, taffy, *id, el.style.x, el.style.y);
        }
    }
}

pub fn layout_tree(el: &mut Element, window_w: f32, window_h: f32) {
    let mut taffy = TaffyTree::new();
    let root = add_node(el, &mut taffy);
    taffy
        .compute_layout(
            root,
            taffy::geometry::Size {
                width: AvailableSpace::Definite(window_w),
                height: AvailableSpace::Definite(window_h),
            },
        )
        .unwrap();
    write_back(el, &taffy, root, 0.0, 0.0);
}
