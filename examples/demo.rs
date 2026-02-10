use rentex::{App, Fonts};
use rentex::widgets::WidgetManager;
use winit::keyboard::KeyCode;

fn main() {
    let app = App::new("RNTX demo", 800, 600);

    let mut fonts = Fonts::new();
    let ui_font = fonts.add("JetBrainsMono Nerd Font", 16.0);

    let mut widgets = WidgetManager::new();
    let btn = widgets.button("Click Me");
    widgets.get_mut(btn)
        .position(100.0, 100.0)
        .font(ui_font)
        .auto_size()
        .color([0.0, 0.0, 1.0, 1.0]);

    let mut counter = 0;

    app.run(fonts, widgets, move |widgets, mouse, input| {
        if input.just_pressed(KeyCode::Space) {
            counter += 1;
            widgets.get_mut(btn).text(format!("Clicked: {}", counter));
        }
    });
}
