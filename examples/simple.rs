// examples/simple.rs
use rentex::App;

fn main() {
    let app = App::new("Simple Example", 800, 600);
    
    app.run(|canvas| {
        // Draw a red rectangle
        canvas.shapes.rect(50.0, 50.0, 200.0, 100.0, [1.0, 0.0, 0.0, 1.0]);
        
        // Draw a green circle
        canvas.shapes.circle(400.0, 150.0, 50.0, [0.0, 1.0, 0.0, 1.0]);
        
        // Draw a blue rounded button
        canvas.shapes.rounded_rect(50.0, 200.0, 200.0, 60.0, 8.0, [0.2, 0.4, 0.8, 1.0]);
        
        // Draw text (coming soon)
        // canvas.text.draw("Click me!", 100.0, 215.0);
    });
}
