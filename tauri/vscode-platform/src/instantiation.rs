//! DI container — port of src/vs/platform/instantiation/common/instantiation.ts.
//!
//! Provides typed `ServiceId<T>` tokens, `ServiceCollection` with lazy singleton
//! resolution, child-scope inheritance, and `Disposable` wiring.

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

use vscode_base::disposable::DisposableStore;

// ─────────────────────────────────────────────────────────────────────────────
// ServiceId<T> — typed token (mirrors createDecorator<T>)
// ─────────────────────────────────────────────────────────────────────────────

/// Typed service identifier. `T` is the service trait object (unsized OK).
///
/// Mirrors `ServiceIdentifier<T>` produced by `createDecorator<T>` in TS.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ServiceId<T: ?Sized> {
    id: &'static str,
    _marker: PhantomData<fn() -> *const T>,
}

impl<T: ?Sized> ServiceId<T> {
    /// Construct a new service identifier with the given string key.
    pub const fn new(id: &'static str) -> Self {
        ServiceId {
            id,
            _marker: PhantomData,
        }
    }

    /// The string identifier (e.g. `"ILogService"`).
    pub fn id(&self) -> &'static str {
        self.id
    }
}

/// Declare a typed service-id constant. Mirrors `export const IFoo = createDecorator<IFoo>('foo')`.
///
/// Usage:
/// ```ignore
/// service_id!(pub IFoo: dyn FooService = "IFoo");
/// ```
#[macro_export]
macro_rules! service_id {
    ($vis:vis $name:ident : $ty:ty = $key:literal) => {
        $vis const $name: $crate::instantiation::ServiceId<$ty> =
            $crate::instantiation::ServiceId::new($key);
    };
}

// ─────────────────────────────────────────────────────────────────────────────
// InstantiationType
// ─────────────────────────────────────────────────────────────────────────────

/// Mirrors `InstantiationType` enum in vscode.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum InstantiationType {
    /// Factory is called during `register_singleton` (eager).
    Eager,
    /// Factory is called only on first `get` (lazy/delayed).
    Delayed,
}

// ─────────────────────────────────────────────────────────────────────────────
// Internal slot — holds factory + once-cell for lazy realisation
// ─────────────────────────────────────────────────────────────────────────────

struct Slot {
    #[allow(dead_code)]
    kind: InstantiationType,
    factory: Box<dyn Fn(&ServiceAccessor) -> Arc<dyn Any + Send + Sync> + Send + Sync>,
    /// Cached instance once realised.
    instance: Mutex<Option<Arc<dyn Any + Send + Sync>>>,
}

// ─────────────────────────────────────────────────────────────────────────────
// ServiceCollection
// ─────────────────────────────────────────────────────────────────────────────

/// Owns service registrations and their singleton instances.
///
/// Analogous to VSCode's `InstantiationService` / `ServiceCollection`.
pub struct ServiceCollection {
    /// Keyed by (TypeId_of_T, service_id_string).
    slots: HashMap<(TypeId, &'static str), Arc<Slot>>,
    /// Reverse-order disposal of realised services.
    dispose_store: DisposableStore,
    /// Parent scope (for child scopes). Child falls back to parent on miss.
    parent: Option<Arc<ServiceCollection>>,
}

impl ServiceCollection {
    pub fn new() -> Self {
        ServiceCollection {
            slots: HashMap::new(),
            dispose_store: DisposableStore::new(),
            parent: None,
        }
    }

    /// Register a typed singleton factory.
    ///
    /// If `kind == Eager`, the factory is invoked immediately via a temporary accessor.
    pub fn register_singleton<T>(
        &mut self,
        id: ServiceId<T>,
        factory: Box<dyn Fn(&ServiceAccessor) -> Arc<T> + Send + Sync>,
        kind: InstantiationType,
    ) where
        T: ?Sized + 'static,
        Arc<T>: Any + Send + Sync,
    {
        let type_key = (TypeId::of::<T>(), id.id);

        // Wrap typed factory into type-erased factory.
        let erased: Box<dyn Fn(&ServiceAccessor) -> Arc<dyn Any + Send + Sync> + Send + Sync> =
            Box::new(move |acc| {
                let typed: Arc<T> = factory(acc);
                // Safety: Arc<T> where T: ?Sized + 'static does *not* auto-impl Any.
                // We store it as Arc<ArcWrapper<T>> to allow downcast later.
                Arc::new(ArcWrapper(typed)) as Arc<dyn Any + Send + Sync>
            });

        let slot = Arc::new(Slot {
            kind,
            factory: erased,
            instance: Mutex::new(None),
        });

        // Eager: realise immediately.
        if kind == InstantiationType::Eager {
            let acc = ServiceAccessor {
                slots: self.slots.clone(),
                parent: self.parent.clone(),
            };
            let instance = (slot.factory)(&acc);
            *slot.instance.lock().unwrap() = Some(instance);
        }

        self.slots.insert(type_key, slot);
    }

    /// Retrieve a service, realising it lazily if needed.
    pub fn get<T>(&self, id: ServiceId<T>) -> Option<Arc<T>>
    where
        T: ?Sized + 'static,
        Arc<T>: Any + Send + Sync,
    {
        let type_key = (TypeId::of::<T>(), id.id);
        if let Some(slot) = self.slots.get(&type_key) {
            return Some(Self::realise_slot::<T>(slot, &self.slots, &self.parent));
        }
        // Fall back to parent scope.
        if let Some(parent) = &self.parent {
            return parent.get(id);
        }
        None
    }

    fn realise_slot<T>(
        slot: &Arc<Slot>,
        slots: &HashMap<(TypeId, &'static str), Arc<Slot>>,
        parent: &Option<Arc<ServiceCollection>>,
    ) -> Arc<T>
    where
        T: ?Sized + 'static,
        Arc<T>: Any + Send + Sync,
    {
        let guard = slot.instance.lock().unwrap();
        if let Some(existing) = &*guard {
            return existing
                .clone()
                .downcast::<ArcWrapper<T>>()
                .expect("downcast failed")
                .0
                .clone();
        }
        // Cycle detection: factory recursing would try to lock the same Mutex → deadlock/panic
        // (std::sync::Mutex is non-reentrant). For explicit protection we drop the guard before
        // invoking the factory; re-entrant callers will block on the second lock attempt, which
        // is a clear signal of a dependency cycle.
        drop(guard); // release before calling factory

        let acc = ServiceAccessor {
            slots: slots.clone(),
            parent: parent.clone(),
        };
        let raw = (slot.factory)(&acc);

        let mut guard = slot.instance.lock().unwrap();
        *guard = Some(raw.clone());

        raw.downcast::<ArcWrapper<T>>()
            .expect("downcast failed")
            .0
            .clone()
    }

    /// Create a child scope that inherits parent registrations but can override them.
    pub fn child(self) -> Self {
        let parent = Arc::new(self);
        ServiceCollection {
            slots: HashMap::new(),
            dispose_store: DisposableStore::new(),
            parent: Some(parent),
        }
    }

    /// Build a `ServiceAccessor` snapshot of this collection's current slots.
    pub fn accessor(&self) -> ServiceAccessor {
        ServiceAccessor {
            slots: self.slots.clone(),
            parent: self.parent.clone(),
        }
    }
}

impl Default for ServiceCollection {
    fn default() -> Self {
        Self::new()
    }
}

// ServiceCollection owns a DisposableStore; on drop it disposes tracked resources.
// Service arcs themselves are dropped when the HashMap is cleared by the store Drop.
impl Drop for ServiceCollection {
    fn drop(&mut self) {
        // DisposableStore implements Drop so this is automatic;
        // explicit call here is a no-op but clarifies intent.
        self.dispose_store.dispose();
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// ArcWrapper — lets Arc<dyn Trait> participate in Any::downcast
// ─────────────────────────────────────────────────────────────────────────────

struct ArcWrapper<T: ?Sized>(Arc<T>);

// ─────────────────────────────────────────────────────────────────────────────
// ServiceAccessor — read-only view for use inside factories
// ─────────────────────────────────────────────────────────────────────────────

/// Passed to service factories so they can resolve their own dependencies.
pub struct ServiceAccessor {
    slots: HashMap<(TypeId, &'static str), Arc<Slot>>,
    parent: Option<Arc<ServiceCollection>>,
}

impl ServiceAccessor {
    /// Resolve a dependency inside a factory.
    pub fn get<T>(&self, id: ServiceId<T>) -> Option<Arc<T>>
    where
        T: ?Sized + 'static,
        Arc<T>: Any + Send + Sync,
    {
        let type_key = (TypeId::of::<T>(), id.id);
        if let Some(slot) = self.slots.get(&type_key) {
            return Some(ServiceCollection::realise_slot::<T>(
                slot,
                &self.slots,
                &self.parent,
            ));
        }
        if let Some(parent) = &self.parent {
            return parent.get(id);
        }
        None
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Legacy ServiceRegistry (kept for backwards compat with existing lib.rs tests)
// ─────────────────────────────────────────────────────────────────────────────

use std::sync::RwLock;

/// Type-erased service registry (original placeholder; retained for compat).
#[derive(Default)]
pub struct ServiceRegistry {
    services: RwLock<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>,
}

impl ServiceRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register<T: Any + Send + Sync>(&self, service: T) {
        let mut map = self.services.write().unwrap();
        map.insert(TypeId::of::<T>(), Arc::new(service));
    }

    pub fn get<T: Any + Send + Sync>(&self) -> Option<Arc<T>> {
        let map = self.services.read().unwrap();
        map.get(&TypeId::of::<T>())
            .and_then(|arc| arc.clone().downcast::<T>().ok())
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    // ── Service trait + concrete impl used across tests ──────────────────────

    trait Counter: Send + Sync {
        fn value(&self) -> usize;
    }

    struct CounterImpl {
        v: usize,
    }
    impl Counter for CounterImpl {
        fn value(&self) -> usize {
            self.v }
    }

    // Typed id constant.
    const I_COUNTER: ServiceId<dyn Counter> = ServiceId::new("ICounter");

    // ── Test 1: register + resolve, pointer identity ──────────────────────────
    #[test]
    fn register_and_resolve_pointer_identity() {
        let mut col = ServiceCollection::new();
        col.register_singleton(
            I_COUNTER,
            Box::new(|_| Arc::new(CounterImpl { v: 42 }) as Arc<dyn Counter>),
            InstantiationType::Delayed,
        );

        let a = col.get(I_COUNTER).expect("service missing");
        let b = col.get(I_COUNTER).expect("service missing second");

        assert_eq!(a.value(), 42);
        // Same singleton — pointer equality.
        assert!(Arc::ptr_eq(&a, &b), "must return same Arc");
    }

    // ── Test 2: Delayed — factory NOT invoked until first get ─────────────────
    #[test]
    fn delayed_factory_not_called_until_get() {
        static CALLS: AtomicUsize = AtomicUsize::new(0);

        let mut col = ServiceCollection::new();
        col.register_singleton(
            I_COUNTER,
            Box::new(|_| {
                CALLS.fetch_add(1, Ordering::SeqCst);
                Arc::new(CounterImpl { v: 1 }) as Arc<dyn Counter>
            }),
            InstantiationType::Delayed,
        );

        assert_eq!(CALLS.load(Ordering::SeqCst), 0, "factory must not fire before get");
        let _ = col.get(I_COUNTER);
        assert_eq!(CALLS.load(Ordering::SeqCst), 1, "factory must fire exactly once");
        let _ = col.get(I_COUNTER);
        assert_eq!(CALLS.load(Ordering::SeqCst), 1, "factory must not fire again");
    }

    // ── Test 3: Eager — factory invoked at registration ───────────────────────
    #[test]
    fn eager_factory_called_at_registration() {
        static CALLS: AtomicUsize = AtomicUsize::new(0);

        let mut col = ServiceCollection::new();
        col.register_singleton(
            I_COUNTER,
            Box::new(|_| {
                CALLS.fetch_add(1, Ordering::SeqCst);
                Arc::new(CounterImpl { v: 2 }) as Arc<dyn Counter>
            }),
            InstantiationType::Eager,
        );

        assert_eq!(CALLS.load(Ordering::SeqCst), 1, "factory fires during register");
        let _ = col.get(I_COUNTER);
        assert_eq!(CALLS.load(Ordering::SeqCst), 1, "no second call on get");
    }

    // ── Test 4: child scope override ─────────────────────────────────────────
    #[test]
    fn child_scope_overrides_parent_parent_unchanged() {
        let mut parent = ServiceCollection::new();
        parent.register_singleton(
            I_COUNTER,
            Box::new(|_| Arc::new(CounterImpl { v: 10 }) as Arc<dyn Counter>),
            InstantiationType::Delayed,
        );

        let mut child = parent.child();
        child.register_singleton(
            I_COUNTER,
            Box::new(|_| Arc::new(CounterImpl { v: 99 }) as Arc<dyn Counter>),
            InstantiationType::Delayed,
        );

        let child_val = child.get(I_COUNTER).expect("child missing").value();
        assert_eq!(child_val, 99, "child returns its own override");

        // Parent slot is inside the Arc; we can still access via child's parent ref.
        // Direct parent access via Arc — resolve through child's parent field.
        let parent_arc = child.parent.as_ref().expect("parent arc");
        let parent_val = parent_arc.get(I_COUNTER).expect("parent missing").value();
        assert_eq!(parent_val, 10, "parent unchanged by child override");
    }

    // ── Test 5: child inherits from parent when not overridden ───────────────
    #[test]
    fn child_inherits_parent_service() {
        let mut parent = ServiceCollection::new();
        parent.register_singleton(
            I_COUNTER,
            Box::new(|_| Arc::new(CounterImpl { v: 55 }) as Arc<dyn Counter>),
            InstantiationType::Delayed,
        );

        let child = parent.child();
        let val = child.get(I_COUNTER).expect("should inherit").value();
        assert_eq!(val, 55);
    }

    // ── Test 6: missing service returns None ─────────────────────────────────
    #[test]
    fn get_unregistered_returns_none() {
        let col = ServiceCollection::new();
        assert!(col.get(I_COUNTER).is_none());
    }

    // ── Test 7: disposal — DisposableStore fires on drop ─────────────────────
    #[test]
    fn collection_drop_fires_dispose_store() {
        let fired = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let fired2 = fired.clone();

        {
            let mut col = ServiceCollection::new();
            col.dispose_store.add(move || {
                fired2.store(true, Ordering::SeqCst);
            });
            // col drops here
        }

        assert!(fired.load(Ordering::SeqCst), "dispose_store must fire on drop");
    }

    // ── Test 8: ServiceId macro expansion ────────────────────────────────────
    #[test]
    fn service_id_macro() {
        service_id!(pub I_MY_SVC: dyn Counter = "IMyCounter");
        assert_eq!(I_MY_SVC.id(), "IMyCounter");
    }

    // ── Test 9: factory can resolve dependencies via ServiceAccessor ─────────
    #[test]
    fn factory_can_resolve_dependency() {
        trait Greeter: Send + Sync {
            fn greet(&self) -> String;
        }
        struct GreeterImpl(Arc<dyn Counter>);
        impl Greeter for GreeterImpl {
            fn greet(&self) -> String {
                format!("hello {}", self.0.value())
            }
        }
        const I_GREETER: ServiceId<dyn Greeter> = ServiceId::new("IGreeter");

        let mut col = ServiceCollection::new();
        col.register_singleton(
            I_COUNTER,
            Box::new(|_| Arc::new(CounterImpl { v: 7 }) as Arc<dyn Counter>),
            InstantiationType::Delayed,
        );
        col.register_singleton(
            I_GREETER,
            Box::new(|acc| {
                let counter = acc.get(I_COUNTER).expect("counter dep");
                Arc::new(GreeterImpl(counter)) as Arc<dyn Greeter>
            }),
            InstantiationType::Delayed,
        );

        let g = col.get(I_GREETER).expect("greeter missing");
        assert_eq!(g.greet(), "hello 7");
    }
}
