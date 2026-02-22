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

    // runs once on launch
    // this just starts the clock and fetches weather
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
            Some(r) => format!("result: {}", r),
            None if self.processing => "processing...".into(),
            None => "no result yet".into(),
        };

        column(vec![
            // clock
            text(&format!("uptime: {}s", self.seconds), Color::WHITE).font_size(32.0),
            // weather
            text(&weather_label, Color::WHITE),
            button("Refresh Weather").on_click(Action::FetchWeather),
            // cpu work
            text(&result_label, Color::WHITE),
            button("Process Data").on_click(Action::ProcessData),
            // notification
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
            // clock tick
            Action::Tick => {
                self.seconds += 1;
                vec![]
            }

            // kick off a network request
            Action::FetchWeather => {
                self.weather_loading = true;
                println!("fetching weather");
                vec![Task::run(async {
                    let result = reqwest::get("https://wttr.in/?format=3").await;
                    match result {
                        Ok(r) => match r.text().await {
                            Ok(text) => Action::WeatherLoaded(text),
                            Err(_) => Action::WeatherFailed,
                        },
                        Err(_) => Action::WeatherFailed,
                    }
                })]
            }
            Action::WeatherLoaded(data) => {
                println!("weather loaded");
                self.weather_loading = false;
                self.weather = Some(data);
                vec![]
            }
            Action::WeatherFailed => {
                println!("weather failed");
                self.weather_loading = false;
                self.weather = Some("failed to load".into());
                vec![]
            }

            // kick off cpu heavy work on background thread
            Action::ProcessData => {
                self.processing = true;
                vec![Task::background(|| {
                    // simulate expensive work
                    std::thread::sleep(Duration::from_secs(2));
                    let result: u64 = (0..1_000_000u64).sum();
                    Action::DataProcessed(result)
                })]
            }
            Action::DataProcessed(result) => {
                self.processing = false;
                self.result = Some(result);
                vec![]
            }

            // show a notification then auto hide after 3 seconds
            Action::ShowNotification(msg) => {
                self.notification = Some(msg);
                vec![Task::delay(
                    Duration::from_secs(3),
                    Action::HideNotification,
                )]
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
