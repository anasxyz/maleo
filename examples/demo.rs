use rentex::App;
use rentex::Ui;

fn main() {
    let app = App::new("RNTX Demo", 800, 600);
    
    app.run(|ui| {
        ui.text("Hello World", 12.0, 100.0, 100.0);
    });
}
