use rentex::App;
use rentex::widgets::{ButtonWidget, WidgetManager};
use winit::keyboard::KeyCode;

fn main() {
    let app = App::new("RNTX demo", 800, 600);
    let mut widgets = WidgetManager::new();

    widgets.button(0, "Click Me")
        .position(100.0, 100.0)
        .size(200.0, 50.0)
        .color([0.0, 0.0, 1.0, 1.0]);

    let mut counter = 0;

    app.run(widgets, move |widgets, _mouse, input| {
        if input.just_pressed(KeyCode::Space) {
            counter += 1;
            if let Some(btn) = widgets.get_mut::<ButtonWidget>(0) {
                btn.text = format!("Clicked: {}", counter);
            }
        }
    });
}
