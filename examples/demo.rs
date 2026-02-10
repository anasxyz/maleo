use rentex::App;
use rentex::Ui;
use rentex::widgets::WidgetManager;
use rentex::widgets::{ButtonWidget, Rect, Widget};
use winit::keyboard::KeyCode;

fn main() {
    let app = App::new("Demo", 800, 600);
    
    let mut widgets = WidgetManager::new();
    widgets.add(ButtonWidget::new(0, "Click Me").position(100.0, 100.0).size(200.0, 50.0).color([0.0, 0.0, 1.0, 1.0]));
    
    let mut counter = 0;
    
    app.run(widgets, move |widgets, mouse, input| {
        if let Some(btn) = widgets.get_button_mut(0) {
            if input.just_pressed(KeyCode::Space) {
                counter += 1;
            }
            btn.text(&format!("Clicked: {}", counter));
        }
    });
}
