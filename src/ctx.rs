use crate::{Fonts, InputState, MouseState};
use crate::widgets::WidgetManager;

/// everything the user needs during setup and update
pub struct Ctx {
    pub widgets: WidgetManager,
    pub fonts: Fonts,
    pub mouse: MouseState,
    pub input: InputState,
}

impl Ctx {
    pub(crate) fn new(fonts: Fonts) -> Self {
        Self {
            widgets: WidgetManager::new(),
            fonts,
            mouse: MouseState::default(),
            input: InputState::default(),
        }
    }
}
