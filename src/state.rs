use std::any::{Any, TypeId};
use std::collections::HashMap;

// per-widget state entry — type erased so the store can hold any type
struct StateEntry {
    value: Box<dyn Any>,
    type_id: TypeId,
}

// newtype wrapper for text input callbacks — defined at module level so
// register and call agree on the exact type for downcasting
struct TextCb<M>(Box<dyn Fn(String) -> M>);

// the state store — lives on the runner, persists across frames
pub struct StateStore {
    pub(crate) entries: HashMap<String, StateEntry>,
    text_callbacks: HashMap<String, Box<dyn Any>>,
    input_values: HashMap<String, String>, // current value cache per input id
}

impl StateStore {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            text_callbacks: HashMap::new(),
            input_values: HashMap::new(),
        }
    }

    // get state for a widget by id, returns None if not found or wrong type
    pub fn get<T: Any>(&self, id: &str) -> Option<&T> {
        self.entries.get(id)?.value.downcast_ref::<T>()
    }

    // get mutable state for a widget by id
    pub fn get_mut<T: Any>(&mut self, id: &str) -> Option<&mut T> {
        self.entries.get_mut(id)?.value.downcast_mut::<T>()
    }

    // get state or insert default if missing
    pub fn get_or_default<T: Any + Default>(&mut self, id: &str) -> &T {
        if !self.entries.contains_key(id) {
            self.insert(id, T::default());
        }
        self.get::<T>(id).unwrap()
    }

    // get mutable state or insert default if missing
    pub fn get_or_default_mut<T: Any + Default>(&mut self, id: &str) -> &mut T {
        if !self.entries.contains_key(id) {
            self.insert(id, T::default());
        }
        self.get_mut::<T>(id).unwrap()
    }

    // insert or replace state for a widget
    pub fn insert<T: Any>(&mut self, id: &str, value: T) {
        self.entries.insert(
            id.to_string(),
            StateEntry {
                type_id: TypeId::of::<T>(),
                value: Box::new(value),
            },
        );
    }

    // register a text input's on_change callback — called each frame during draw
    pub(crate) fn register_text_callback<M: Clone + 'static>(
        &mut self,
        id: &str,
        callback: Box<dyn Fn(String) -> M>,
    ) {
        self.text_callbacks
            .insert(id.to_string(), Box::new(TextCb(callback)));
    }

    pub(crate) fn call_text_callback<M: Clone + 'static>(
        &self,
        id: &str,
        value: String,
    ) -> Option<M> {
        let cb = self.text_callbacks.get(id)?;
        let cb = cb.downcast_ref::<TextCb<M>>()?;
        Some((cb.0)(value))
    }

    pub(crate) fn set_input_value(&mut self, id: &str, value: &str) {
        self.input_values.insert(id.to_string(), value.to_string());
    }

    pub(crate) fn get_input_value(&self, id: &str) -> String {
        self.input_values.get(id).cloned().unwrap_or_default()
    }

    pub(crate) fn clear_text_callbacks(&mut self) {
        // nothing — callbacks and values are overwritten each frame during draw
        // clearing them would break key events that arrive between frames
    }

    // remove state for a widget
    pub fn remove(&mut self, id: &str) {
        self.entries.remove(id);
    }
}

impl StateStore {
    // scan all entries to find a focused TextInputState — used by runner to know where to send keys
    pub fn find_focused_text_input(&self) -> Option<String> {
        use crate::draw::TextInputState;
        for (id, entry) in &self.entries {
            if entry.type_id == std::any::TypeId::of::<TextInputState>() {
                if let Some(s) = entry.value.downcast_ref::<TextInputState>() {
                    if s.focused {
                        return Some(id.clone());
                    }
                }
            }
        }
        None
    }
}
