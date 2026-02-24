use std::any::{Any, TypeId};
use std::collections::HashMap;

pub struct StateStore {
    // persists across frames — widget state structs
    state: HashMap<(TypeId, String), Box<dyn Any>>,
    // refreshed every frame — callbacks registered during draw
    callbacks: HashMap<(TypeId, String), Box<dyn Any>>,
}

impl StateStore {
    pub fn new() -> Self {
        Self {
            state: HashMap::new(),
            callbacks: HashMap::new(),
        }
    }

    // --- state (persists across frames) ---

    pub fn get<T: Any>(&self, id: &str) -> Option<&T> {
        self.state
            .get(&(TypeId::of::<T>(), id.to_string()))
            .and_then(|v| v.downcast_ref::<T>())
    }

    pub fn get_mut<T: Any>(&mut self, id: &str) -> Option<&mut T> {
        self.state
            .get_mut(&(TypeId::of::<T>(), id.to_string()))
            .and_then(|v| v.downcast_mut::<T>())
    }

    pub fn get_or_default<T: Any + Default>(&mut self, id: &str) -> &T {
        let key = (TypeId::of::<T>(), id.to_string());
        self.state
            .entry(key)
            .or_insert_with(|| Box::new(T::default()))
            .downcast_ref::<T>()
            .unwrap()
    }

    pub fn get_or_default_mut<T: Any + Default>(&mut self, id: &str) -> &mut T {
        let key = (TypeId::of::<T>(), id.to_string());
        self.state
            .entry(key)
            .or_insert_with(|| Box::new(T::default()))
            .downcast_mut::<T>()
            .unwrap()
    }

    pub fn insert<T: Any>(&mut self, id: &str, value: T) {
        self.state
            .insert((TypeId::of::<T>(), id.to_string()), Box::new(value));
    }

    pub fn remove<T: Any>(&mut self, id: &str) {
        self.state.remove(&(TypeId::of::<T>(), id.to_string()));
    }

    // scan state entries of a given type, return the id of the first match
    pub fn find<T: Any, F: Fn(&T) -> bool>(&self, pred: F) -> Option<String> {
        let target = TypeId::of::<T>();
        for ((type_id, id), value) in &self.state {
            if *type_id == target {
                if let Some(val) = value.downcast_ref::<T>() {
                    if pred(val) {
                        return Some(id.clone());
                    }
                }
            }
        }
        None
    }

    // --- callbacks (refreshed every frame during draw) ---

    pub fn set_callback<T: Any>(&mut self, id: &str, cb: T) {
        self.callbacks
            .insert((TypeId::of::<T>(), id.to_string()), Box::new(cb));
    }

    pub fn get_callback<T: Any>(&self, id: &str) -> Option<&T> {
        self.callbacks
            .get(&(TypeId::of::<T>(), id.to_string()))
            .and_then(|v| v.downcast_ref::<T>())
    }
}
