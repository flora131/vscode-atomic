//! Event<T> / Emitter<T> — port of `src/vs/base/common/event.ts`.
//!
//! Each `Emitter<T>` holds a listener registry. `fire` invokes all live listeners.
//! `add_listener` returns a `ListenerHandle` whose `Drop` impl removes the listener.

use std::sync::{Arc, Mutex};

pub type Listener<T> = Box<dyn Fn(&T) + Send + Sync + 'static>;

struct Registry<T> {
    next_id: u64,
    entries: Vec<(u64, Listener<T>)>,
}

impl<T> Registry<T> {
    fn new() -> Self {
        Self { next_id: 0, entries: Vec::new() }
    }
    fn add(&mut self, l: Listener<T>) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.entries.push((id, l));
        id
    }
    fn remove(&mut self, id: u64) {
        self.entries.retain(|(i, _)| *i != id);
    }
}

/// Thread-safe event emitter. Cloneable — clones share the same listener set.
pub struct Emitter<T> {
    inner: Arc<Mutex<Registry<T>>>,
}

impl<T> Clone for Emitter<T> {
    fn clone(&self) -> Self {
        Self { inner: Arc::clone(&self.inner) }
    }
}

impl<T> Default for Emitter<T> {
    fn default() -> Self {
        Self { inner: Arc::new(Mutex::new(Registry::new())) }
    }
}

impl<T> Emitter<T> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a listener. Returns a handle; drop it to unsubscribe.
    pub fn add_listener(
        &self,
        f: impl Fn(&T) + Send + Sync + 'static,
    ) -> ListenerHandle<T> {
        let id = self.inner.lock().unwrap().add(Box::new(f));
        ListenerHandle { registry: Arc::clone(&self.inner), id }
    }

    /// Legacy helper kept for backward compat with existing tests.
    pub fn on(&self, f: impl Fn(&T) + Send + 'static) -> usize
    where
        T: 'static,
    {
        // We need Sync here; wrap in a mutex-guarded adapter.
        let f = Arc::new(Mutex::new(f));
        let id = self.inner.lock().unwrap().add(Box::new(move |v| {
            f.lock().unwrap()(v);
        }));
        id as usize
    }

    /// Fire event — calls all registered listeners in registration order.
    /// Holds the lock during calls (listeners must not re-lock this emitter).
    pub fn fire(&self, value: &T) {
        let guard = self.inner.lock().unwrap();
        for (_, l) in guard.entries.iter() {
            l(value);
        }
    }
}

/// Dropping this handle removes the associated listener from the emitter.
pub struct ListenerHandle<T> {
    registry: Arc<Mutex<Registry<T>>>,
    id: u64,
}

impl<T> Drop for ListenerHandle<T> {
    fn drop(&mut self) {
        self.registry.lock().unwrap().remove(self.id);
    }
}

// ─────────── backward-compat alias ───────────────────────────────────────────
pub type EventEmitter<T> = Emitter<T>;

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    // Test 1 — fire reaches all listeners in order.
    #[test]
    fn fire_reaches_all_listeners() {
        let emitter: Emitter<i32> = Emitter::new();
        let results = Arc::new(Mutex::new(vec![]));

        let r1 = results.clone();
        let _h1 = emitter.add_listener(move |v| r1.lock().unwrap().push(*v));
        let r2 = results.clone();
        let _h2 = emitter.add_listener(move |v| r2.lock().unwrap().push(*v * 10));

        emitter.fire(&3);
        assert_eq!(*results.lock().unwrap(), vec![3, 30]);
    }

    // Test 2 — dropping handle removes listener.
    #[test]
    fn listener_removed_on_handle_drop() {
        let emitter: Emitter<u8> = Emitter::new();
        let count = Arc::new(Mutex::new(0u32));
        let c = count.clone();
        let handle = emitter.add_listener(move |_| *c.lock().unwrap() += 1);

        emitter.fire(&1);
        assert_eq!(*count.lock().unwrap(), 1);

        drop(handle); // unsubscribe
        emitter.fire(&2);
        // count must not change
        assert_eq!(*count.lock().unwrap(), 1);
    }

    // Test 3 — multiple independent emitters don't cross-fire.
    #[test]
    fn independent_emitters_no_cross_fire() {
        let e1: Emitter<&str> = Emitter::new();
        let e2: Emitter<&str> = Emitter::new();
        let log = Arc::new(Mutex::new(Vec::<String>::new()));

        let l1 = log.clone();
        let _h1 = e1.add_listener(move |v| l1.lock().unwrap().push(format!("e1:{v}")));
        let l2 = log.clone();
        let _h2 = e2.add_listener(move |v| l2.lock().unwrap().push(format!("e2:{v}")));

        e1.fire(&"A");
        e2.fire(&"B");
        assert_eq!(*log.lock().unwrap(), vec!["e1:A", "e2:B"]);
    }

    // Backward-compat: existing EventEmitter<T>::on / fire API still works.
    #[test]
    fn backward_compat_on_fire() {
        let emitter: EventEmitter<i32> = EventEmitter::new();
        let results = Arc::new(Mutex::new(vec![]));
        let r = results.clone();
        emitter.on(move |v| r.lock().unwrap().push(*v));
        emitter.fire(&42);
        emitter.fire(&7);
        assert_eq!(*results.lock().unwrap(), vec![42, 7]);
    }
}
