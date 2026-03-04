use bento::prelude::*;

struct MyApp;

impl App for MyApp {
    fn new() -> Self {
        MyApp
    }

    fn view(&mut self) -> Element {
        row(vec![
            col(vec![rect()]),
            rect(),
        ])
    }
}

fn main() {
    MyApp::run(WindowSettings::default());
}
