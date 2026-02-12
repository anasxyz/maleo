use crate::{Color, FontId, Fonts, InputState, MouseState, ShapeRenderer, TextRenderer};

pub struct Rect {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub color: Color,
    pub outline_color: Color,
    pub outline_thickness: f32,
}

pub struct Text {
    pub id: u32,
    pub text: String,
    pub font_id: FontId,
    pub x: f32,
    pub y: f32,
    pub color: Color,
    pub font_size: f32,
    pub font_family: String,
}

pub enum Widget {
    Rect(Rect),
    Text(Text),
    Container(Box<Container>),
}

pub enum ContainerDirection {
    Horizontal,
    Vertical,
}

pub struct Container {
    pub id: u32,
    pub direction: ContainerDirection,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub gap: f32,
    pub children: Vec<Widget>,
}

/// everything the user needs during setup and update
pub struct Ctx {
    pub fonts: Fonts,
    pub(crate) text_renderer: TextRenderer,
    pub(crate) shape_renderer: ShapeRenderer,

    pub mouse: MouseState,
    pub input: InputState,
    pub exit: bool,

    pub root_widgets: Vec<Widget>,

    dirty: bool,

    pub window_height: f32,
    pub window_width: f32,
}

impl Ctx {
    pub(crate) fn new(
        fonts: Fonts,
        text_renderer: TextRenderer,
        shape_renderer: ShapeRenderer,
    ) -> Self {
        Self {
            text_renderer,
            shape_renderer,
            fonts,

            mouse: MouseState::default(),
            input: InputState::default(),
            exit: false,

            root_widgets: Vec::new(),

            dirty: false,

            window_height: 0.0,
            window_width: 0.0,
        }
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        println!("screen dimensions: {}x{}", width, height);
        self.window_width = width;
        self.window_height = height;
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }

    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    pub(crate) fn take_dirty(&mut self) -> bool {
        let d = self.dirty;
        self.dirty = false;
        d
    }

    pub fn vcontainer(&mut self, x: f32, y: f32, w: f32, h: f32, gap: f32, children: Vec<Widget>) {
        let new_container = Container {
            id: self.root_widgets.len() as u32,
            direction: ContainerDirection::Vertical,
            x,
            y,
            w,
            h,
            gap,
            children,
        };
        self.root_widgets
            .push(Widget::Container(Box::new(new_container)));
        self.mark_dirty();
    }

    pub fn hcontainer(&mut self, x: f32, y: f32, w: f32, h: f32, gap: f32, children: Vec<Widget>) {
        let new_container = Container {
            id: self.root_widgets.len() as u32,
            direction: ContainerDirection::Horizontal,
            x,
            y,
            w,
            h,
            gap,
            children,
        };
        self.root_widgets
            .push(Widget::Container(Box::new(new_container)));
        self.mark_dirty();
    }

    pub fn rect(
        &mut self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        color: Color,
        outline_color: Color,
        outline_thickness: f32,
    ) {
        let new_rect = Rect {
            id: self.root_widgets.len() as u32,
            x,
            y,
            w,
            h,
            color,
            outline_color,
            outline_thickness,
        };
        self.root_widgets.push(Widget::Rect(new_rect));
        self.mark_dirty();
    }

    pub fn text(&mut self, text: &str, font_id: FontId, x: f32, y: f32, color: Color) {
        let entry = self.fonts.get(font_id);
        let family = entry.family.clone();
        let size = entry.size;

        let new_text = Text {
            id: self.root_widgets.len() as u32,
            text: text.to_string(),
            font_id,
            x,
            y,
            color,
            font_size: size,
            font_family: family,
        };
        self.root_widgets.push(Widget::Text(new_text));
        self.mark_dirty();
    }

    pub fn circle(
        &mut self,
        cx: f32,
        cy: f32,
        radius: f32,
        color: [f32; 4],
        outline_color: [f32; 4],
        outline_thickness: f32,
    ) {
        self.shape_renderer
            .circle(cx, cy, radius, color, outline_color, outline_thickness);
    }

    pub fn rounded_rect(
        &mut self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        radius: f32,
        color: [f32; 4],
        outline_color: [f32; 4],
        outline_thickness: f32,
    ) {
        self.shape_renderer.rounded_rect(
            x,
            y,
            w,
            h,
            radius,
            color,
            outline_color,
            outline_thickness,
        );
    }

    pub fn layout(&mut self) {
        for widget in &mut self.root_widgets {
            Self::layout_widget(widget, &mut self.fonts);
        }
    }

    fn layout_widget(widget: &mut Widget, fonts: &mut Fonts) {
        match widget {
            Widget::Container(container) => {
                let mut current_x = container.x;
                let mut current_y = container.y;

                for child in &mut container.children {
                    match child {
                        Widget::Rect(rect) => {
                            rect.x = current_x;
                            rect.y = current_y;

                            match container.direction {
                                ContainerDirection::Vertical => {
                                    current_y += rect.h + container.gap;
                                }
                                ContainerDirection::Horizontal => {
                                    current_x += rect.w + container.gap;
                                }
                            }
                        }
                        Widget::Text(text) => {
                            text.x = current_x;
                            text.y = current_y;

                            let (w, h) = fonts.measure(&text.text, text.font_id);

                            match container.direction {
                                ContainerDirection::Vertical => {
                                    current_y += h + container.gap;
                                }
                                ContainerDirection::Horizontal => {
                                    current_x += w + container.gap;
                                }
                            }
                        }
                        Widget::Container(_) => {
                            if let Widget::Container(nested) = child {
                                nested.x = current_x;
                                nested.y = current_y;
                            }

                            Self::layout_widget(child, fonts);

                            if let Widget::Container(nested) = child {
                                match container.direction {
                                    ContainerDirection::Vertical => {
                                        current_y += nested.h + container.gap; 
                                    }
                                    ContainerDirection::Horizontal => {
                                        current_x += nested.w + container.gap;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn measure_container(container: &Container, fonts: &mut Fonts) -> (f32, f32) {
        let mut width: f32 = 0.0;
        let mut height: f32 = 0.0;

        for child in &container.children {
            match child {
                Widget::Rect(rect) => match container.direction {
                    ContainerDirection::Vertical => {
                        width = width.max(rect.w);
                        height += rect.h + container.gap;
                    }
                    ContainerDirection::Horizontal => {
                        width += rect.w + container.gap;
                        height = height.max(rect.h);
                    }
                },
                Widget::Text(text) => {
                    let (w, h) = fonts.measure(&text.text, text.font_id);
                    match container.direction {
                        ContainerDirection::Vertical => {
                            width = width.max(w);
                            height += h + container.gap;
                        }
                        ContainerDirection::Horizontal => {
                            width += w + container.gap;
                            height = height.max(h);
                        }
                    }
                }
                Widget::Container(nested) => {
                    let (w, h) = Self::measure_container(nested, fonts);
                    match container.direction {
                        ContainerDirection::Vertical => {
                            width = width.max(w);
                            height += h + container.gap;
                        }
                        ContainerDirection::Horizontal => {
                            width += w + container.gap;
                            height = height.max(h);
                        }
                    }
                }
            }
        }

        match container.direction {
            ContainerDirection::Vertical if height > 0.0 => height -= container.gap,
            ContainerDirection::Horizontal if width > 0.0 => width -= container.gap,
            _ => {}
        }

        (width, height)
    }

    pub fn render_widgets(&mut self) {
        for widget in &self.root_widgets {
            Self::render_widget(
                widget,
                &mut self.shape_renderer,
                &mut self.text_renderer,
                &mut self.fonts,
            );
        }
    }

    fn render_widget(
        widget: &Widget,
        shape_renderer: &mut ShapeRenderer,
        text_renderer: &mut TextRenderer,
        fonts: &mut Fonts,
    ) {
        match widget {
            Widget::Rect(rect) => {
                shape_renderer.rect(
                    rect.x,
                    rect.y,
                    rect.w,
                    rect.h,
                    rect.color.to_array(),
                    rect.outline_color.to_array(),
                    rect.outline_thickness,
                );
            }
            Widget::Text(text) => {
                text_renderer.draw(
                    &mut fonts.font_system,
                    text.font_family.clone(),
                    text.font_size,
                    &text.text,
                    text.x,
                    text.y,
                );
            }
            Widget::Container(container) => {
                shape_renderer.rect(
                    container.x,
                    container.y,
                    container.w,
                    container.h,
                    [0.2, 0.2, 0.2, 0.3], 
                    [0.5, 0.5, 0.5, 0.5], 
                    1.0,
                );

                for child in &container.children {
                    Self::render_widget(child, shape_renderer, text_renderer, fonts);
                }
            }
        }
    }
}
