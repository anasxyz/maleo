use bento::prelude::*;

struct MyApp;

impl App for MyApp {
    fn new() -> Self {
        MyApp
    }

    fn view(&mut self) -> Element {
        container(vec![
            container(vec![rect()]),
            rect(),
        ])
    }
}

fn main() {
    MyApp::run(WindowSettings::default());
}
