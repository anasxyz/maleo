use crate::events::{Event, Key};
use crate::state::StateStore;

// the part of text input state that persists between frames
// only cursor and focus — the actual value lives in the app
#[derive(Default)]
pub struct TextInputState {
    pub focused: bool,
    pub cursor: usize,
    pub scroll_offset: f32, // pixels scrolled left to keep cursor visible
}

// wraps the on_change closure so it can be stored as Box<dyn Any>
// defined here at module level so register and retrieve agree on the type
pub(crate) struct TextInputCallback<M>(pub Box<dyn Fn(String) -> M>);

// store the on_change closure in state so the runner can call it on key events
pub(crate) fn register_callback<M: Clone + 'static>(
    state: &mut StateStore,
    id: &str,
    callback: Box<dyn Fn(String) -> M>,
) {
    state.set_raw(id, TextInputCallback(callback));
}

// call the stored on_change closure with a new value, returns the resulting action
pub(crate) fn call_callback<M: Clone + 'static>(
    state: &StateStore,
    id: &str,
    value: String,
) -> Option<M> {
    let cb = state.get_raw::<TextInputCallback<M>>(id)?;
    Some((cb.0)(value))
}

// store the current value so the runner can read it during key events
// (the value lives in the app, but the runner needs it between frames)
pub(crate) fn cache_value(state: &mut StateStore, id: &str, value: &str) {
    state.set_string(id, value);
}

// read the cached value
pub(crate) fn get_cached_value(state: &StateStore, id: &str) -> String {
    state.get_string(id)
}

// scan state to find which text input is currently focused
pub(crate) fn find_focused(state: &StateStore) -> Option<String> {
    state.find_by_type::<TextInputState, _>(|s| s.focused)
}

// process a key event for a focused input
// takes the current value from the app, returns the new value if it changed
pub fn handle_key(
    state: &mut StateStore,
    id: &str,
    current_value: &str,
    event: &Event,
    text: &str, // actual character from OS, already shift/locale aware
) -> Option<String> {
    let focused = state.get_or_default::<TextInputState>(id).focused;
    if !focused {
        return None;
    }

    let mut value = current_value.to_string();
    let mut cursor = state
        .get_or_default::<TextInputState>(id)
        .cursor
        .min(value.len());
    let mut changed = false;

    match event {
        Event::KeyPressed {
            key: Key::Backspace,
            ..
        } => {
            if cursor > 0 {
                if let Some((i, _)) = value[..cursor].char_indices().next_back() {
                    value.remove(i);
                    cursor = i;
                    changed = true;
                }
            }
        }
        Event::KeyPressed {
            key: Key::Delete, ..
        } => {
            if cursor < value.len() {
                value.remove(cursor);
                changed = true;
            }
        }
        Event::KeyPressed { key: Key::Left, .. } => {
            if cursor > 0 {
                if let Some((i, _)) = value[..cursor].char_indices().next_back() {
                    cursor = i;
                }
            }
        }
        Event::KeyPressed {
            key: Key::Right, ..
        } => {
            if cursor < value.len() {
                cursor = value[cursor..]
                    .char_indices()
                    .nth(1)
                    .map(|(i, _)| cursor + i)
                    .unwrap_or(value.len());
            }
        }
        Event::KeyPressed { key: Key::Home, .. } => {
            cursor = 0;
        }
        Event::KeyPressed { key: Key::End, .. } => {
            cursor = value.len();
        }
        Event::KeyPressed { .. } => {
            if !text.is_empty() && text != "\r" && text != "\n" && text != "\r\n" {
                value.insert_str(cursor, text);
                cursor += text.len();
                changed = true;
            }
        }
        _ => {}
    }

    state.get_or_default_mut::<TextInputState>(id).cursor = cursor;

    if changed { Some(value) } else { None }
}
