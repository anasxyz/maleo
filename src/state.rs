use std::any::{Any, TypeId};
use std::collections::HashMap;

struct StateEntry {
    value: Box<dyn Any>,
    type_id: TypeId,
}

pub struct StateStore {
    entries: HashMap<String, StateEntry>,
}

impl StateStore {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    // get a value by id, returns None if not found or wrong type
    pub fn get<T: Any>(&self, id: &str) -> Option<&T> {
        self.entries.get(id)?.value.downcast_ref::<T>()
    }

    pub fn get_mut<T: Any>(&mut self, id: &str) -> Option<&mut T> {
        self.entries.get_mut(id)?.value.downcast_mut::<T>()
    }

    // get or insert a default value
    pub fn get_or_default<T: Any + Default>(&mut self, id: &str) -> &T {
        if !self.entries.contains_key(id) {
            self.insert(id, T::default());
        }
        self.get::<T>(id).unwrap()
    }

    pub fn get_or_default_mut<T: Any + Default>(&mut self, id: &str) -> &mut T {
        if !self.entries.contains_key(id) {
            self.insert(id, T::default());
        }
        self.get_mut::<T>(id).unwrap()
    }

    // insert or overwrite a value
    pub fn insert<T: Any>(&mut self, id: &str, value: T) {
        self.entries.insert(
            id.to_string(),
            StateEntry {
                type_id: TypeId::of::<T>(),
                value: Box::new(value),
            },
        );
    }

    pub fn remove(&mut self, id: &str) {
        self.entries.remove(id);
    }

    // --- internal helpers used by widgets ---

    // store any typed value under a namespaced key
    pub(crate) fn set_raw<T: Any>(&mut self, id: &str, value: T) {
        let key = format!("__raw__{}", id);
        self.insert(&key, value);
    }

    pub(crate) fn get_raw<T: Any>(&self, id: &str) -> Option<&T> {
        let key = format!("__raw__{}", id);
        self.get::<T>(&key)
    }

    // store a plain string under a namespaced key
    pub(crate) fn set_string(&mut self, id: &str, value: &str) {
        let key = format!("__str__{}", id);
        self.insert(&key, value.to_string());
    }

    pub(crate) fn get_string(&self, id: &str) -> String {
        let key = format!("__str__{}", id);
        self.get::<String>(&key).cloned().unwrap_or_default()
    }

    // scan all entries for a specific type, return the id of the first one
    // where the predicate returns true
    pub(crate) fn find_by_type<T: Any, F: Fn(&T) -> bool>(&self, pred: F) -> Option<String> {
        let target = TypeId::of::<T>();
        for (id, entry) in &self.entries {
            if entry.type_id == target {
                if let Some(val) = entry.value.downcast_ref::<T>() {
                    if pred(val) {
                        return Some(id.clone());
                    }
                }
            }
        }
        None
    }
}
