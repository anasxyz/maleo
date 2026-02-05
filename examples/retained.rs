// examples/retained_mode.rs
use rentex::App;

fn main() {
    let app = App::new("Retained Mode Example", 800, 600);

    app.run(|rntx| {
        // Everything goes through rntx.scene
        // Scene stores all commands and replays them
        
        rntx.scene.rect(50.0, 50.0, 200.0, 100.0, [1.0, 0.0, 0.0, 1.0]);
        rntx.scene.circle(400.0, 150.0, 50.0, [0.0, 1.0, 0.0, 1.0]);
        rntx.scene.text("Hello, Retained Mode!", 60.0, 220.0);
        rntx.scene.button(500.0, 500.0, 50.0, 25.0, "Click Me");
    });
}
