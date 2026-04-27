//! Minimal Event<T> / EventEmitter placeholder mirroring vscode.d.ts Event<T>.

use std::sync::{Arc, Mutex};

pub type Listener<T> = Box<dyn Fn(&T) + Send + 'static>;

/// Single-event emitter. Each call to `fire` invokes all registered listeners.
pub struct EventEmitter<T> {
    listeners: Arc<Mutex<Vec<Listener<T>>>>,
}

impl<T> Default for EventEmitter<T> {
    fn default() -> Self {
        Self { listeners: Arc::new(Mutex::new(Vec::new())) }
    }
}

impl<T> EventEmitter<T> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a listener; returns its index (handle for future disposal).
    pub fn on(&self, listener: impl Fn(&T) + Send + 'static) -> usize {
        let mut guard = self.listeners.lock().unwrap();
        guard.push(Box::new(listener));
        guard.len() - 1
    }

    /// Fire the event, calling every registered listener with `value`.
    pub fn fire(&self, value: &T) {
        let guard = self.listeners.lock().unwrap();
        for l in guard.iter() {
            l(value);
        }
    }
}
