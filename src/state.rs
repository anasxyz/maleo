use std::any::{Any, TypeId};
use std::collections::HashMap;

// per-widget state entry — type erased so the store can hold any type
struct StateEntry {
    value: Box<dyn Any>,
    type_id: TypeId,
}

// the state store — lives on the runner, persists across frames
pub struct StateStore {
    entries: HashMap<String, StateEntry>,
}

impl StateStore {
    pub fn new() -> Self {
        Self { entries: HashMap::new() }
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
        self.entries.insert(id.to_string(), StateEntry {
            type_id: TypeId::of::<T>(),
            value: Box::new(value),
        });
    }

    // remove state for a widget — useful when a widget is removed from the tree
    pub fn remove(&mut self, id: &str) {
        self.entries.remove(id);
    }
}
