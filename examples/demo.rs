use bento::*;

struct MyApp;

impl App for MyApp {
    fn new() -> Self {
        MyApp
    }

    fn view(&mut self) -> Element {
        col(vec![
            rect().w(100.0).h(200.0).fill(Color::rgb(110, 110, 10)),
            row(vec![
                rect().w(100.0).h(200.0).fill(Color::rgb(110, 10, 10)),
                rect().w(100.0).h(200.0).fill(Color::rgb(110, 10, 120)),
            ]),
            row(vec![
                rect().w(100.0).h(200.0).fill(Color::rgb(10, 110, 10)),
                rect().w(100.0).h(200.0).fill(Color::rgb(10, 10, 110)),
            ]),
        ])
    }
}

fn main() {
    MyApp::run(WindowSettings::default());
}
