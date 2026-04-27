//! Registry extension-point system mirroring src/vs/platform/registry/common/platform.ts.
//!
//! Tests written first (TDD RED). Implementation below.

use std::any::Any;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex, RwLock};
use once_cell::sync::Lazy;

// ── ServiceAccessor stub ─────────────────────────────────────────────────────
// Full impl lives in instantiation.rs (another worker). Keep a minimal trait
// so WorkbenchContribution compiles standalone.
pub trait ServiceAccessorLike: Send + Sync {}

/// Placeholder forwarded to WorkbenchContribution::start.
pub struct ServiceAccessor;
impl ServiceAccessorLike for ServiceAccessor {}

// ── RegistryKey ──────────────────────────────────────────────────────────────

/// Typed extension-point key. `T` is the value type stored under this key.
/// PhantomData<fn() -> T> is covariant and Send+Sync regardless of T.
pub struct RegistryKey<T: 'static + Send + Sync>(&'static str, PhantomData<fn() -> T>);

impl<T: 'static + Send + Sync> RegistryKey<T> {
    pub const fn new(id: &'static str) -> Self {
        RegistryKey(id, PhantomData)
    }

    pub fn id(&self) -> &'static str {
        self.0
    }
}

// Safety: RegistryKey<T> holds only a &'static str and PhantomData<fn()->T>.
// fn()->T is always Send+Sync.
unsafe impl<T: 'static + Send + Sync> Send for RegistryKey<T> {}
unsafe impl<T: 'static + Send + Sync> Sync for RegistryKey<T> {}

// ── Global registry ───────────────────────────────────────────────────────────

static GLOBAL_REGISTRY: Lazy<RwLock<HashMap<&'static str, Box<dyn Any + Send + Sync>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

// ── Registry API ─────────────────────────────────────────────────────────────

pub struct Registry;

impl Registry {
    /// Register a value under a typed key.
    ///
    /// # Panics
    /// Panics if a value is already registered under `key`.
    pub fn register<T: 'static + Send + Sync>(key: &RegistryKey<T>, value: T) {
        let mut map = GLOBAL_REGISTRY.write().unwrap_or_else(|e| e.into_inner());
        if map.contains_key(key.0) {
            panic!("Registry: key '{}' already registered", key.0);
        }
        map.insert(key.0, Box::new(Arc::new(value)));
    }

    /// Retrieve a value by typed key.
    ///
    /// Returns `None` if not registered.
    pub fn as_<T: 'static + Send + Sync>(key: &RegistryKey<T>) -> Option<Arc<T>> {
        let map = GLOBAL_REGISTRY.read().unwrap_or_else(|e| e.into_inner());
        map.get(key.0)
            .and_then(|boxed| boxed.downcast_ref::<Arc<T>>())
            .cloned()
    }
}

// ── WorkbenchPhase ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WorkbenchPhase {
    BlockStartup = 0,
    AfterRestored = 1,
    Eventually = 2,
}

// ── WorkbenchContribution ─────────────────────────────────────────────────────

pub trait WorkbenchContribution: Send + Sync {
    fn start(&self, accessor: &ServiceAccessor);
}

// ── WorkbenchContributionRegistry ────────────────────────────────────────────

struct Entry {
    #[allow(dead_code)]
    id: &'static str,
    phase: WorkbenchPhase,
    contribution: Box<dyn WorkbenchContribution>,
}

/// Registry for workbench contributions ordered by phase.
pub struct WorkbenchContributionRegistry {
    entries: Mutex<Vec<Entry>>,
}

impl WorkbenchContributionRegistry {
    pub fn new() -> Self {
        WorkbenchContributionRegistry {
            entries: Mutex::new(Vec::new()),
        }
    }

    /// Register a contribution to run in the given phase.
    pub fn register(
        &self,
        id: &'static str,
        contribution: Box<dyn WorkbenchContribution>,
        phase: WorkbenchPhase,
    ) {
        self.entries.lock().unwrap().push(Entry { id, phase, contribution });
    }

    /// Run all contributions registered for `phase`.
    pub fn run_phase(&self, phase: WorkbenchPhase, accessor: &ServiceAccessor) {
        let entries = self.entries.lock().unwrap();
        let matching: Vec<&Entry> = entries.iter().filter(|e| e.phase == phase).collect();
        // Registration order preserved via Vec; no secondary sort needed.
        for entry in &matching {
            entry.contribution.start(accessor);
        }
    }
}

impl Default for WorkbenchContributionRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ── Public function mirroring registerWorkbenchContribution2 ─────────────────

/// Module-level global for the workbench contribution registry.
pub static WORKBENCH_CONTRIBUTIONS: Lazy<WorkbenchContributionRegistry> =
    Lazy::new(WorkbenchContributionRegistry::new);

/// Register a workbench contribution (mirrors registerWorkbenchContribution2).
pub fn register_workbench_contribution(
    id: &'static str,
    contribution: Box<dyn WorkbenchContribution>,
    phase: WorkbenchPhase,
) {
    WORKBENCH_CONTRIBUTIONS.register(id, contribution, phase);
}

// ── Sub-module: keys ──────────────────────────────────────────────────────────
pub mod keys;

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    // Each test uses unique key IDs to avoid global state collisions.

    // ── Typed key constants used only in tests ────────────────────────────────
    struct FakeEditorFactory {
        pub name: &'static str,
    }

    static TEST_KEY_A: RegistryKey<FakeEditorFactory> = RegistryKey::new("test::editor_factory_a");
    static TEST_KEY_B: RegistryKey<FakeEditorFactory> = RegistryKey::new("test::editor_factory_b");
    static TEST_KEY_COLLISION: RegistryKey<FakeEditorFactory> =
        RegistryKey::new("test::collision_key");

    // ── Test 1: register + retrieve preserves identity ────────────────────────
    #[test]
    fn register_and_retrieve_typed_value_preserves_identity() {
        Registry::register(&TEST_KEY_A, FakeEditorFactory { name: "monaco" });
        let retrieved = Registry::as_(&TEST_KEY_A).expect("must be present");
        assert_eq!(retrieved.name, "monaco");
    }

    // ── Test 2: unregistered key returns None ─────────────────────────────────
    #[test]
    fn retrieve_unregistered_key_returns_none() {
        let result = Registry::as_(&TEST_KEY_B);
        assert!(result.is_none());
    }

    // ── Test 3: collision panics ──────────────────────────────────────────────
    #[test]
    #[should_panic(expected = "already registered")]
    fn duplicate_registration_panics() {
        Registry::register(&TEST_KEY_COLLISION, FakeEditorFactory { name: "first" });
        Registry::register(&TEST_KEY_COLLISION, FakeEditorFactory { name: "second" });
    }

    // ── Test 4: WorkbenchContribution phase ordering ──────────────────────────
    #[test]
    fn workbench_contributions_execute_in_phase_order() {
        let log: Arc<Mutex<Vec<&'static str>>> = Arc::new(Mutex::new(Vec::new()));

        struct LogContrib {
            label: &'static str,
            log: Arc<Mutex<Vec<&'static str>>>,
        }

        impl WorkbenchContribution for LogContrib {
            fn start(&self, _accessor: &ServiceAccessor) {
                self.log.lock().unwrap().push(self.label);
            }
        }

        let registry = WorkbenchContributionRegistry::new();
        let accessor = ServiceAccessor;

        registry.register(
            "contrib-eventually",
            Box::new(LogContrib { label: "eventually", log: log.clone() }),
            WorkbenchPhase::Eventually,
        );
        registry.register(
            "contrib-block",
            Box::new(LogContrib { label: "block", log: log.clone() }),
            WorkbenchPhase::BlockStartup,
        );
        registry.register(
            "contrib-after",
            Box::new(LogContrib { label: "after", log: log.clone() }),
            WorkbenchPhase::AfterRestored,
        );

        // Run phases in order; each phase only runs its own contributions.
        registry.run_phase(WorkbenchPhase::BlockStartup, &accessor);
        registry.run_phase(WorkbenchPhase::AfterRestored, &accessor);
        registry.run_phase(WorkbenchPhase::Eventually, &accessor);

        let result = log.lock().unwrap().clone();
        assert_eq!(result, vec!["block", "after", "eventually"]);
    }

    // ── Test 5: run_phase only runs matching phase ────────────────────────────
    #[test]
    fn run_phase_only_runs_matching_phase() {
        let log: Arc<Mutex<Vec<&'static str>>> = Arc::new(Mutex::new(Vec::new()));

        struct LogContrib {
            label: &'static str,
            log: Arc<Mutex<Vec<&'static str>>>,
        }
        impl WorkbenchContribution for LogContrib {
            fn start(&self, _accessor: &ServiceAccessor) {
                self.log.lock().unwrap().push(self.label);
            }
        }

        let registry = WorkbenchContributionRegistry::new();
        let accessor = ServiceAccessor;

        registry.register(
            "only-block",
            Box::new(LogContrib { label: "block", log: log.clone() }),
            WorkbenchPhase::BlockStartup,
        );
        registry.register(
            "only-after",
            Box::new(LogContrib { label: "after", log: log.clone() }),
            WorkbenchPhase::AfterRestored,
        );

        // Only run BlockStartup phase.
        registry.run_phase(WorkbenchPhase::BlockStartup, &accessor);

        let result = log.lock().unwrap().clone();
        assert_eq!(result, vec!["block"]);
        // "after" was NOT executed.
    }
}
