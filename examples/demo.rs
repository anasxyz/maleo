use bento::*;

struct MyApp;

impl App for MyApp {
    fn new() -> Self {
        MyApp
    }

    fn view(&mut self) -> Element {
        col(vec![
            rect().w(100.0).h(200.0).bg(rgb(110, 110, 10)),
            row(vec![
                rect().w(100.0).h(200.0).bg(rgb(110, 10, 10)),
                rect().w(100.0).h(200.0).bg(rgb(110, 10, 120)),
            ]),
            row(vec![
                rect()
                    .w(100.0)
                    .h(200.0)
                    .bg(rgb(10, 110, 10))
                    .mt(10.0),
                rect()
                    .w(100.0)
                    .h(100.0)
                    .bg(rgb(10, 10, 110))
                    .border(3.0)
                    .border_radius(10.0)
                    .border_color(rgb(0, 0, 0)),
            ]),
        ])
        .m([10.0, 10.0, 10.0, 10.0])
    }
}

fn main() {
    MyApp::run(WindowSettings::default());
}
