use std::ops::{Deref, DerefMut};
use crate::{Ui, widgets::{ButtonWidget, Widget, WidgetHandle}};

pub struct WidgetMut<'a, T: Widget> {
    widget: &'a mut T,
    dirty: &'a mut bool,
}

impl<'a, T: Widget> Deref for WidgetMut<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.widget
    }
}

impl<'a, T: Widget> DerefMut for WidgetMut<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        *self.dirty = true;
        self.widget
    }
}

pub struct WidgetManager {
    widgets: Vec<Box<dyn Widget>>,
    next_id: usize,
    dirty: bool,
}

impl WidgetManager {
    pub fn new() -> Self {
        Self {
            widgets: Vec::new(),
            next_id: 0,
            dirty: true, // dirty on first frame so initial state renders
        }
    }

    fn alloc_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn button(&mut self, text: &str) -> WidgetHandle<ButtonWidget> {
        let id = self.alloc_id();
        self.widgets.push(Box::new(ButtonWidget::new(id, text)));
        WidgetHandle::new(id)
    }

    pub fn get_mut<T: Widget + 'static>(&mut self, handle: WidgetHandle<T>) -> WidgetMut<T> {
        for widget in self.widgets.iter_mut() {
            if widget.id() == handle.id {
                let widget = widget.as_any_mut().downcast_mut::<T>()
                    .expect("widget type mismatch");
                return WidgetMut { widget, dirty: &mut self.dirty };
            }
        }
        panic!("widget with id {} not found", handle.id);
    }

    pub(crate) fn take_dirty(&mut self) -> bool {
        let d = self.dirty;
        self.dirty = false;
        d
    }

    pub(crate) fn render_all(&self, ui: &mut Ui) {
        for widget in &self.widgets {
            widget.render(ui);
        }
    }
}
