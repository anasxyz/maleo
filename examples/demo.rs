use rentex::App;
use rentex::Ui;
use rentex::widgets::{ButtonWidget, Rect, Widget};

fn main() {
    let app = App::new("RNTX Demo", 800, 600);

    app.run(|ui| {
        let button = ButtonWidget::new(0, "Click Me")
            .position(100.0, 100.0)
            .size(200.0, 50.0)
            .font_size(20.0)
            .color([0.0, 1.0, 0.0, 1.0]);

        button.render(ui);
    });
}
