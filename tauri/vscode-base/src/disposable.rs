//! Disposable pattern — port of `src/vs/base/common/lifecycle.ts`.
//!
//! `Disposable` trait (Rust Drop analogue), `DisposableStore` owning a Vec of boxed
//! disposables and disposing them in **reverse insertion order** on drop, and
//! `DisposableHandle` for early removal from a store.

use std::sync::{Arc, Mutex};

// ─────────────────────────────────────────────────────────────────────────────
// Core trait
// ─────────────────────────────────────────────────────────────────────────────

/// Analogous to `IDisposable` in TypeScript — an object that cleans up on demand.
pub trait Disposable: Send {
    fn dispose(&mut self);
}

/// Blanket impl: a closure `FnMut() + Send` is disposable.
impl<F: FnMut() + Send> Disposable for F {
    fn dispose(&mut self) {
        self();
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// DisposableStore
// ─────────────────────────────────────────────────────────────────────────────

type SlotVec = Arc<Mutex<Vec<Option<Box<dyn Disposable>>>>>;

/// Owns a collection of disposables; disposes all in reverse order when dropped.
pub struct DisposableStore {
    slots: SlotVec,
    is_disposed: bool,
}

impl DisposableStore {
    pub fn new() -> Self {
        Self {
            slots: Arc::new(Mutex::new(Vec::new())),
            is_disposed: false,
        }
    }

    /// Add a disposable. Returns a `DisposableHandle` that can remove it early.
    pub fn add<D: Disposable + 'static>(&mut self, d: D) -> DisposableHandle {
        if self.is_disposed {
            // Store already disposed — dispose the value immediately (matches TS warn+leak
            // behaviour, but we eagerly dispose instead of leaking).
            let mut d = d;
            d.dispose();
            return DisposableHandle { slots: Arc::new(Mutex::new(vec![])), idx: 0 };
        }
        let mut guard = self.slots.lock().unwrap();
        let idx = guard.len();
        guard.push(Some(Box::new(d)));
        DisposableHandle { slots: Arc::clone(&self.slots), idx }
    }

    /// Dispose all children and mark store as disposed.
    pub fn dispose(&mut self) {
        if self.is_disposed {
            return;
        }
        self.is_disposed = true;
        self.clear();
    }

    /// Dispose all children but keep store usable.
    pub fn clear(&mut self) {
        let mut guard = self.slots.lock().unwrap();
        // Reverse order (last registered → first disposed), matching TS semantics.
        for slot in guard.iter_mut().rev() {
            if let Some(d) = slot.take() {
                let mut d = d;
                d.dispose();
            }
        }
        guard.clear();
    }

    pub fn is_disposed(&self) -> bool {
        self.is_disposed
    }
}

impl Default for DisposableStore {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for DisposableStore {
    fn drop(&mut self) {
        self.dispose();
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// DisposableHandle
// ─────────────────────────────────────────────────────────────────────────────

/// Returned by `DisposableStore::add`; dropping it removes + disposes the item.
pub struct DisposableHandle {
    slots: SlotVec,
    idx: usize,
}

impl DisposableHandle {
    /// Remove and dispose the associated item immediately, before the store drops.
    pub fn dispose(self) {
        // consume self to prevent double-dispose via Drop
        let mut guard = self.slots.lock().unwrap();
        if let Some(slot) = guard.get_mut(self.idx) {
            if let Some(mut d) = slot.take() {
                d.dispose();
            }
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests (RED first — compiled before implementations to confirm failure)
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    // Test 1 — dispose ordering: last-registered disposed first.
    #[test]
    fn dispose_reverse_order() {
        let order = Arc::new(Mutex::new(Vec::<usize>::new()));
        let mut store = DisposableStore::new();

        for i in 0..3usize {
            let o = order.clone();
            store.add(move || { o.lock().unwrap().push(i); });
        }

        drop(store); // triggers dispose

        assert_eq!(*order.lock().unwrap(), vec![2, 1, 0]);
    }

    // Test 2 — early removal via DisposableHandle.
    #[test]
    fn handle_early_dispose() {
        let fired = Arc::new(Mutex::new(false));
        let f = fired.clone();
        let mut store = DisposableStore::new();
        let handle = store.add(move || { *f.lock().unwrap() = true; });

        // Dispose via handle before store drops.
        handle.dispose();
        assert!(*fired.lock().unwrap(), "should fire immediately on handle dispose");

        // Dropping store should not double-fire (slot is None).
        let fired2 = fired.clone();
        drop(store);
        assert!(*fired2.lock().unwrap()); // still true, not panicked
    }

    // Test 3 — clear keeps store usable; dispose marks it done.
    #[test]
    fn clear_then_add_then_dispose() {
        let count = Arc::new(Mutex::new(0u32));
        let mut store = DisposableStore::new();

        let c1 = count.clone();
        store.add(move || { *c1.lock().unwrap() += 1; });
        store.clear(); // disposes the one item, count == 1
        assert_eq!(*count.lock().unwrap(), 1);

        assert!(!store.is_disposed(), "clear does not mark disposed");

        let c2 = count.clone();
        store.add(move || { *c2.lock().unwrap() += 10; });
        store.dispose(); // now count == 11
        assert_eq!(*count.lock().unwrap(), 11);
        assert!(store.is_disposed());
    }
}
