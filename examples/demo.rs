use bento::*;

struct MyApp;

impl App for MyApp {
    fn new() -> Self {
        MyApp
    }

    fn view(&mut self) -> Element {
        row(vec![
            rect()
                .w(Size::Fixed(200.0))
                .h(Size::Fixed(200.0))
                .bg(rgb(200, 50, 50)),
            rect()
                .w(Size::Fixed(200.0))
                .h(Size::Fixed(200.0))
                .bg(rgb(50, 200, 50)),
        ])
    }
}

fn main() {
    MyApp::run(WindowSettings::default());
}
