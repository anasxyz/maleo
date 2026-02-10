use crate::{Ui, widgets::Widget};

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

    pub fn get(&self, id: usize) -> Option<&dyn Widget> {
        self.widgets
            .iter()
            .find(|w| w.id() == id)
            .map(|w| w.as_ref())
    }

    pub fn get_mut(&mut self, id: usize) -> Option<&mut dyn Widget> {
        for widget in self.widgets.iter_mut() {
            if widget.id() == id {
                return Some(widget.as_mut());
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
