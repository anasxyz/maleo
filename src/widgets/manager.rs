use crate::{Drawer, MouseState, widgets::{ButtonWidget, SliderWidget, Widget, WidgetHandle}};

pub struct WidgetManager {
    widgets: Vec<Box<dyn Widget>>,
    next_id: usize,
    dirty: bool,
}

impl WidgetManager {
    pub fn new() -> Self {
        Self { widgets: Vec::new(), next_id: 0, dirty: true }
    }

    fn alloc_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn button(&mut self, text: &str) -> WidgetHandle<ButtonWidget> {
        let id = self.alloc_id();
        self.widgets.push(Box::new(ButtonWidget::new(id, text)));
        self.dirty = true;
        WidgetHandle::new(id)
    }

    pub fn slider(&mut self) -> WidgetHandle<SliderWidget> {
        let id = self.alloc_id();
        self.widgets.push(Box::new(SliderWidget::new(id)));
        self.dirty = true;
        WidgetHandle::new(id)
    }

    pub fn get<T: Widget + 'static>(&self, handle: WidgetHandle<T>) -> &T {
        for widget in self.widgets.iter() {
            if widget.id() == handle.id {
                return widget.as_any().downcast_ref::<T>()
                    .expect("widget type mismatch");
            }
        }
        panic!("widget with id {} not found", handle.id);
    }

    pub fn get_mut<T: Widget + 'static>(&mut self, handle: WidgetHandle<T>) -> &mut T {
        self.dirty = true;
        for widget in self.widgets.iter_mut() {
            if widget.id() == handle.id {
                return widget.as_any_mut().downcast_mut::<T>()
                    .expect("widget type mismatch");
            }
        }
        panic!("widget with id {} not found", handle.id);
    }

    pub(crate) fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    pub(crate) fn update_all(&mut self, mouse: &MouseState) -> bool {
        let mut changed = false;
        for widget in self.widgets.iter_mut() {
            let was_hovered = widget.bounds().contains(
                mouse.x - mouse.dx,
                mouse.y - mouse.dy,
            );
            widget.update(mouse);
            let is_hovered = widget.bounds().contains(mouse.x, mouse.y);

            if was_hovered != is_hovered
                || mouse.left_just_pressed
                || mouse.left_just_released
                || mouse.right_just_pressed
                || mouse.right_just_released
                || mouse.left_pressed // covers dragging
            {
                changed = true;
            }
        }
        changed
    }

    pub(crate) fn take_dirty(&mut self) -> bool {
        let d = self.dirty;
        self.dirty = false;
        d
    }

    pub(crate) fn render_all(&mut self, drawer: &mut Drawer) {
        for widget in self.widgets.iter_mut() {
            widget.render(drawer);
        }
    }
}
