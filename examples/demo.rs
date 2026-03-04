use bento::*;

struct MyApp;

impl App for MyApp {
    fn new() -> Self {
        MyApp
    }

    fn view(&mut self) -> Element {
        container(vec![
            rect().w(100.0).h(200.0).fill(Color::rgb(110, 110, 10)),
            rect().w(100.0).h(200.0).fill(Color::rgb(110, 10, 10)),
        ])
        .direction("row".to_string())
    }
}

fn main() {
    MyApp::run(WindowSettings::default());
}
