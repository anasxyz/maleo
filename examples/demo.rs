use rentex::{App, Ctx, Fonts, RentexApp};
use rentex::widgets::{ButtonWidget, SliderWidget, WidgetHandle};

// ── Menu Screen ──────────────────────────────────────────────────────────────

struct MenuScreen {
    play_btn: Option<WidgetHandle<ButtonWidget>>,
    settings_btn: Option<WidgetHandle<ButtonWidget>>,
}

impl MenuScreen {
    fn new() -> Self {
        Self { play_btn: None, settings_btn: None }
    }

    fn setup(&mut self, ctx: &mut Ctx, font: rentex::FontId) {
        let play = ctx.widgets.button("Play");
        ctx.widgets.get_mut(play)
            .position(300.0, 200.0)
            .font(font)
            .auto_size()
            .color([0.2, 0.7, 0.2, 1.0]);
        self.play_btn = Some(play);

        let settings = ctx.widgets.button("Settings");
        ctx.widgets.get_mut(settings)
            .position(300.0, 260.0)
            .font(font)
            .auto_size()
            .color([0.2, 0.2, 0.7, 1.0]);
        self.settings_btn = Some(settings);
    }

    /// Returns which screen to switch to, if any.
    fn update(&mut self, ctx: &mut Ctx) -> Option<Screen> {
        if ctx.widgets.get(self.play_btn.unwrap()).just_clicked {
            return Some(Screen::Menu); // placeholder — would go to game
        }
        if ctx.widgets.get(self.settings_btn.unwrap()).just_clicked {
            return Some(Screen::Settings);
        }
        None
    }
}

// ── Settings Screen ───────────────────────────────────────────────────────────

struct SettingsScreen {
    volume_slider: Option<WidgetHandle<SliderWidget>>,
    back_btn: Option<WidgetHandle<ButtonWidget>>,
}

impl SettingsScreen {
    fn new() -> Self {
        Self { volume_slider: None, back_btn: None }
    }

    fn setup(&mut self, ctx: &mut Ctx, font: rentex::FontId) {
        let slider = ctx.widgets.slider();
        ctx.widgets.get_mut(slider)
            .position(400.0, 220.0)
            .size(300.0, 20.0)
            .range(0.0, 100.0)
            .value(75.0)
            .fill_color([0.8, 0.5, 0.1, 1.0])
            .show_label(font);
        self.volume_slider = Some(slider);

        let back = ctx.widgets.button("Back");
        ctx.widgets.get_mut(back)
            .position(300.0, 300.0)
            .font(font)
            .auto_size()
            .color([0.6, 0.2, 0.2, 1.0]);
        self.back_btn = Some(back);
    }

    fn update(&mut self, ctx: &mut Ctx) -> Option<Screen> {
        if ctx.widgets.get(self.back_btn.unwrap()).just_clicked {
            return Some(Screen::Menu);
        }
        None
    }

    fn volume(&self, ctx: &Ctx) -> f32 {
        ctx.widgets.get(self.volume_slider.unwrap()).value
    }
}

// ── App ───────────────────────────────────────────────────────────────────────

#[derive(PartialEq)]
enum Screen {
    Menu,
    Settings,
}

struct MyApp {
    current_screen: Screen,
    menu: MenuScreen,
    settings: SettingsScreen,
    volume: f32,
}

impl RentexApp for MyApp {
    fn setup(&mut self, ctx: &mut Ctx) {
        let font = ctx.fonts.add("JetBrainsMono Nerd Font", 16.0);
        self.menu.setup(ctx, font);
        self.settings.setup(ctx, font);
    }

    fn update(&mut self, ctx: &mut Ctx) {
        let next = match self.current_screen {
            Screen::Menu     => self.menu.update(ctx),
            Screen::Settings => self.settings.update(ctx),
        };

        if let Some(screen) = next {
            self.current_screen = screen;
        }

        // can access any screen's widgets from anywhere
        self.volume = self.settings.volume(ctx);
    }
}

fn main() {
    App::new("RNTX demo", 800, 600).run(Fonts::new(), MyApp {
        current_screen: Screen::Menu,
        menu: MenuScreen::new(),
        settings: SettingsScreen::new(),
        volume: 75.0,
    });
}
