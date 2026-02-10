use crate::{
    Ui,
    widgets::{ButtonWidget, Widget},
};

pub struct WidgetManager {
    widgets: Vec<Box<dyn Widget>>,
}

impl WidgetManager {
    pub fn new() -> Self {
        Self {
            widgets: Vec::new(),
        }
    }

    pub fn add(&mut self, widget: impl Widget + 'static) {
        self.widgets.push(Box::new(widget));
    }

    pub fn button(&mut self, id: usize, text: &str) -> &mut ButtonWidget {
        self.widgets.push(Box::new(ButtonWidget::new(id, text)));
        self.get_mut(id).unwrap()
    }

    pub fn get_mut<T: Widget + 'static>(&mut self, id: usize) -> Option<&mut T> {
        for widget in self.widgets.iter_mut() {
            if widget.id() == id {
                return widget.as_any_mut().downcast_mut::<T>();
            }
        }
        None
    }

    pub(crate) fn render_all(&self, ui: &mut Ui) {
        for widget in &self.widgets {
            widget.render(ui);
        }
    }
}
