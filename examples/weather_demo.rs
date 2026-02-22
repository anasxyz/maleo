#![allow(dead_code, unused)]

use bento::*;
use std::time::Duration;

#[derive(Clone)]
enum Action {
    Tick,
    FetchWeather,
    WeatherLoaded(String),
    WeatherFailed,
    ProcessData,
    DataProcessed(u64),
    ShowNotification(String),
    HideNotification,
}

struct MyApp {
    seconds: u64,
    weather: Option<String>,
    weather_loading: bool,
    processing: bool,
    result: Option<u64>,
    notification: Option<String>,
}

impl App for MyApp {
    type Action = Action;

    fn new() -> Self {
        Self {
            seconds: 0,
            weather: None,
            weather_loading: false,
            processing: false,
            result: None,
            notification: None,
        }
    }

    fn start(&mut self) -> Vec<Task<Action>> {
        vec![
            Task::every(Duration::from_secs(1), Action::Tick),
            Task::delay(Duration::from_millis(500), Action::FetchWeather),
        ]
    }

    fn view(&self) -> Element<Action> {
        let weather_label = match &self.weather {
            _ if self.weather_loading => "fetching weather...".into(),
            Some(w) => w.clone(),
            None => "no weather data".into(),
        };

        let result_label = match self.result {
            _ if self.processing => "processing...".into(),
            Some(r) => format!("result: {}", r),
            None => "no result yet".into(),
        };

        column(vec![
            text(&format!("uptime: {}s", self.seconds), Color::WHITE).font_size(32.0),
            text(&weather_label, Color::WHITE),
            button("Refresh Weather").on_click(Action::FetchWeather),
            text(&result_label, Color::WHITE),
            button("Process Data").on_click(Action::ProcessData),
            button("Show Notification")
                .on_click(Action::ShowNotification("hello from bento!".into())),
            match &self.notification {
                Some(msg) => text(msg, Color::GREEN),
                None => empty(),
            },
        ])
        .gap(12.0)
        .padding(Edges::all(32.0))
        .width(Val::Percent(100.0))
        .height(Val::Percent(100.0))
        .align_x(Align::Center)
        .align_y(Align::Center)
    }

    fn update(&mut self, action: Action) -> Vec<Task<Action>> {
        match action {
            Action::Tick => {
                self.seconds += 1;
                vec![]
            }

            Action::FetchWeather => {
                self.weather_loading = true;
                vec![
                    Task::run(async {
                        let result = reqwest::get("https://wttr.in/?format=3").await;
                        match result {
                            Ok(r) => match r.text().await {
                                Ok(text) => Action::WeatherLoaded(text),
                                Err(_) => Action::WeatherFailed,
                            },
                            Err(_) => Action::WeatherFailed,
                        }
                    })
                    .exclusive()
                    .timeout(Duration::from_secs(2), Action::WeatherFailed),
                ]
            }
            Action::WeatherLoaded(data) => {
                self.weather_loading = false;
                self.weather = Some(data);
                vec![]
            }
            Action::WeatherFailed => {
                self.weather_loading = false;
                self.weather = Some("failed to load".into());
                vec![]
            }

            Action::ProcessData => {
                self.processing = true;
                vec![
                    Task::background(|| {
                        std::thread::sleep(Duration::from_secs(2));
                        let result: u64 = (0..1_000_000u64).sum();
                        Action::DataProcessed(result)
                    })
                    .exclusive(),
                ]
            }
            Action::DataProcessed(result) => {
                self.processing = false;
                self.result = Some(result);
                vec![]
            }

            Action::ShowNotification(msg) => {
                self.notification = Some(msg);
                vec![Task::delay(Duration::from_secs(3), Action::HideNotification).exclusive()]
            }
            Action::HideNotification => {
                self.notification = None;
                vec![]
            }
        }
    }

    fn event(&mut self, event: Event) -> Option<Action> {
        match event {
            Event::KeyPressed { key: Key::F5, .. } => Some(Action::FetchWeather),
            Event::KeyPressed {
                key: Key::Space, ..
            } => Some(Action::ProcessData),
            _ => None,
        }
    }
}

fn main() {
    MyApp::run(Settings::default().title("demo"));
}
