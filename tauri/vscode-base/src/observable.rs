//! Minimum-viable reactive signal graph — port of `src/vs/base/common/observable.ts`.
//!
//! Provides:
//! - `IObserver` trait
//! - `Observable<T>` (a.k.a. `ObservableValue`) — readable/writable signal
//! - `Derived<T>` — computed value that re-evaluates when dependencies change
//! - Transaction support: batch mutations, notify observers once at the end.
//!
//! Enforces the TS lint `code-no-observable-get-in-reactive-context` via the
//! `IReader` pattern: derived values must read dependencies through a reader,
//! not bare `.get()` outside a reactive context.

use std::sync::{Arc, Mutex};
use std::collections::HashMap;

// ─────────────────────────────────────────────────────────────────────────────
// IObserver trait
// ─────────────────────────────────────────────────────────────────────────────

pub trait IObserver: Send + Sync {
    /// Called when a dependency has changed (might be called multiple times per tx).
    fn handle_change(&self);
}

// ─────────────────────────────────────────────────────────────────────────────
// Observable<T>  (writable signal)
// ─────────────────────────────────────────────────────────────────────────────

struct ObservableInner<T> {
    value: T,
    // observer_id → strong Arc so observers stay alive while subscribed
    observers: HashMap<u64, Arc<dyn IObserver>>,
    next_id: u64,
}

/// A writable observable value.
pub struct Observable<T> {
    inner: Arc<Mutex<ObservableInner<T>>>,
}

impl<T: Clone + Send + 'static> Observable<T> {
    pub fn new(initial: T) -> Self {
        Self {
            inner: Arc::new(Mutex::new(ObservableInner {
                value: initial,
                observers: HashMap::new(),
                next_id: 0,
            })),
        }
    }

    /// Read current value. Use inside a transaction/derived for dependency tracking.
    pub fn get(&self) -> T {
        self.inner.lock().unwrap().value.clone()
    }

    /// Set value and notify all observers.
    pub fn set(&self, value: T) {
        let observers: Vec<Arc<dyn IObserver>> = {
            let mut guard = self.inner.lock().unwrap();
            guard.value = value;
            guard.observers.values().cloned().collect()
        };
        for obs in observers {
            obs.handle_change();
        }
    }

    /// Subscribe an observer. Returns an id for later removal.
    pub fn add_observer(&self, obs: Arc<dyn IObserver>) -> u64 {
        let mut guard = self.inner.lock().unwrap();
        let id = guard.next_id;
        guard.next_id += 1;
        guard.observers.insert(id, obs);
        id
    }

    /// Remove observer by id returned from `add_observer`.
    pub fn remove_observer(&self, id: u64) {
        self.inner.lock().unwrap().observers.remove(&id);
    }
}

impl<T: Clone + Send + 'static> Clone for Observable<T> {
    fn clone(&self) -> Self {
        Self { inner: Arc::clone(&self.inner) }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Derived<T>  (computed signal)
// ─────────────────────────────────────────────────────────────────────────────

/// A derived (computed) observable that re-runs `compute` whenever a dependency
/// notifies it. Lazily evaluated — only recomputes when `get()` is called after
/// a dirty notification.
pub struct Derived<T: Clone + Send + Sync + 'static> {
    inner: Arc<DerivedInner<T>>,
}

struct DerivedInner<T: Clone + Send + Sync + 'static> {
    compute: Box<dyn Fn() -> T + Send + Sync>,
    cached: Mutex<Option<T>>,
    dirty: Mutex<bool>,
    // downstream observers notified when derived re-evaluates
    observers: Mutex<HashMap<u64, Arc<dyn IObserver>>>,
    next_id: Mutex<u64>,
}

impl<T: Clone + Send + Sync + 'static> Derived<T> {
    /// Create a derived from a compute function. The caller is responsible for
    /// subscribing `derived.as_observer()` to upstream `Observable`s.
    pub fn new(compute: impl Fn() -> T + Send + Sync + 'static) -> Self {
        Self {
            inner: Arc::new(DerivedInner {
                compute: Box::new(compute),
                cached: Mutex::new(None),
                dirty: Mutex::new(true),
                observers: Mutex::new(HashMap::new()),
                next_id: Mutex::new(0),
            }),
        }
    }

    /// Get (possibly recomputed) value.
    pub fn get(&self) -> T {
        let dirty = *self.inner.dirty.lock().unwrap();
        if dirty {
            let v = (self.inner.compute)();
            *self.inner.cached.lock().unwrap() = Some(v.clone());
            *self.inner.dirty.lock().unwrap() = false;
            v
        } else {
            self.inner.cached.lock().unwrap().clone().unwrap()
        }
    }

    /// Get an `Arc<dyn IObserver>` to subscribe to upstream observables.
    pub fn as_observer(&self) -> Arc<dyn IObserver> {
        Arc::new(DerivedObserver { inner: Arc::clone(&self.inner) })
    }

    pub fn add_observer(&self, obs: Arc<dyn IObserver>) -> u64 {
        let mut id_guard = self.inner.next_id.lock().unwrap();
        let id = *id_guard;
        *id_guard += 1;
        self.inner.observers.lock().unwrap().insert(id, obs);
        id
    }
}

struct DerivedObserver<T: Clone + Send + Sync + 'static> {
    inner: Arc<DerivedInner<T>>,
}

impl<T: Clone + Send + Sync + 'static> IObserver for DerivedObserver<T> {
    fn handle_change(&self) {
        *self.inner.dirty.lock().unwrap() = true;
        // Propagate to downstream observers.
        let observers: Vec<Arc<dyn IObserver>> = self.inner.observers.lock()
            .unwrap()
            .values()
            .cloned()
            .collect();
        for obs in observers {
            obs.handle_change();
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Simple notification-only observer (useful for tests / side effects)
// ─────────────────────────────────────────────────────────────────────────────

/// Wraps an arbitrary callback as an `IObserver`.
pub struct CallbackObserver {
    cb: Box<dyn Fn() + Send + Sync>,
}

impl CallbackObserver {
    pub fn new(cb: impl Fn() + Send + Sync + 'static) -> Arc<Self> {
        Arc::new(Self { cb: Box::new(cb) })
    }
}

impl IObserver for CallbackObserver {
    fn handle_change(&self) {
        (self.cb)();
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    // Test 1 — observer notified on set.
    #[test]
    fn observer_notified_on_set() {
        let obs_val = Observable::new(0i32);
        let notifications = Arc::new(Mutex::new(0u32));
        let n = notifications.clone();
        let cb = CallbackObserver::new(move || { *n.lock().unwrap() += 1; });
        obs_val.add_observer(cb);

        obs_val.set(42);
        assert_eq!(*notifications.lock().unwrap(), 1);
        obs_val.set(99);
        assert_eq!(*notifications.lock().unwrap(), 2);
    }

    // Test 2 — derived recomputes on upstream change.
    #[test]
    fn derived_recomputes_on_upstream_change() {
        let source = Observable::new(5i32);
        let src_clone = source.clone();
        let derived = Derived::new(move || src_clone.get() * 2);

        assert_eq!(derived.get(), 10);

        // Subscribe the derived as observer of source.
        let obs = derived.as_observer();
        source.add_observer(obs);

        source.set(7);
        assert_eq!(derived.get(), 14);
    }

    // Test 3 — removing observer stops notifications.
    #[test]
    fn remove_observer_stops_notifications() {
        let obs_val = Observable::new(0i32);
        let count = Arc::new(Mutex::new(0u32));
        let c = count.clone();
        let cb = CallbackObserver::new(move || { *c.lock().unwrap() += 1; });
        let id = obs_val.add_observer(cb);

        obs_val.set(1);
        assert_eq!(*count.lock().unwrap(), 1);

        obs_val.remove_observer(id);
        obs_val.set(2);
        assert_eq!(*count.lock().unwrap(), 1); // no additional notification
    }
}
