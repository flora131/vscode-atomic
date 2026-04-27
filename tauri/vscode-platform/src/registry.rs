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

    /// Returns `true` iff an extension is registered under `id`.
    ///
    /// Mirrors `IRegistry::knows(id)` from platform.ts.
    pub fn knows(id: &str) -> bool {
        let map = GLOBAL_REGISTRY.read().unwrap_or_else(|e| e.into_inner());
        map.contains_key(id)
    }

    /// Return all registered extension-point key ids.
    ///
    /// Order is unspecified (HashMap iteration order).
    pub fn all() -> Vec<&'static str> {
        let map = GLOBAL_REGISTRY.read().unwrap_or_else(|e| e.into_inner());
        map.keys().copied().collect()
    }
}

// ── KnownExtensions ───────────────────────────────────────────────────────────

/// Enum of built-in extension point identifiers.
///
/// Mirrors the `Extensions` namespace constants in
/// src/vs/platform/registry/common/platform.ts.
/// Use `.id()` to get the string key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KnownExtensions {
    /// Editor factory contributions.
    EditorFactories,
    /// Editor pane (panel) contributions.
    EditorPanes,
    /// View container contributions (sidebar / panel groups).
    ViewContainers,
    /// Configuration schema contributions.
    Configuration,
    /// Drag-and-drop handler contributions.
    DragAndDrop,
}

impl KnownExtensions {
    /// String id for this extension point.
    pub fn id(self) -> &'static str {
        match self {
            KnownExtensions::EditorFactories => "vscode.platform.editor.factories",
            KnownExtensions::EditorPanes     => "vscode.platform.editor.panes",
            KnownExtensions::ViewContainers  => "vscode.platform.viewContainers",
            KnownExtensions::Configuration   => "vscode.platform.configuration",
            KnownExtensions::DragAndDrop     => "vscode.platform.dnd",
        }
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

    // ── Test 6: knows returns true after register, false before ──────────────
    #[test]
    fn knows_returns_true_after_register_false_before() {
        static KEY_KNOWS: RegistryKey<FakeEditorFactory> =
            RegistryKey::new("test::knows_check_unique_1");

        assert!(!Registry::knows(KEY_KNOWS.id()), "must be unknown before register");
        Registry::register(&KEY_KNOWS, FakeEditorFactory { name: "knows-test" });
        assert!(Registry::knows(KEY_KNOWS.id()), "must be known after register");
    }

    // ── Test 7: all includes registered key id ────────────────────────────────
    #[test]
    fn all_includes_registered_key_id() {
        static KEY_ALL: RegistryKey<FakeEditorFactory> =
            RegistryKey::new("test::all_check_unique_2");

        let before = Registry::all();
        assert!(!before.contains(&KEY_ALL.id()), "must not be in all() before register");

        Registry::register(&KEY_ALL, FakeEditorFactory { name: "all-test" });
        let after = Registry::all();
        assert!(after.contains(&KEY_ALL.id()), "all() must include newly registered key");
    }

    // ── Test 8: KnownExtensions ids are unique and non-empty ─────────────────
    #[test]
    fn known_extensions_ids_are_unique_and_non_empty() {
        let ids = [
            KnownExtensions::EditorFactories.id(),
            KnownExtensions::EditorPanes.id(),
            KnownExtensions::ViewContainers.id(),
            KnownExtensions::Configuration.id(),
            KnownExtensions::DragAndDrop.id(),
        ];
        for id in &ids {
            assert!(!id.is_empty(), "extension point id must not be empty: {:?}", id);
        }
        let mut seen = std::collections::HashSet::new();
        for id in &ids {
            assert!(seen.insert(*id), "duplicate extension point id: {}", id);
        }
    }

    // ── Test 9: KnownExtensions ids match keys::* static ids ─────────────────
    #[test]
    fn known_extensions_ids_match_key_constants() {
        use crate::registry::keys;
        assert_eq!(KnownExtensions::EditorFactories.id(), keys::EDITOR_FACTORIES.id());
        assert_eq!(KnownExtensions::EditorPanes.id(),     keys::EDITOR_PANES.id());
        assert_eq!(KnownExtensions::ViewContainers.id(),  keys::VIEW_CONTAINERS.id());
        assert_eq!(KnownExtensions::Configuration.id(),   keys::CONFIGURATION.id());
        assert_eq!(KnownExtensions::DragAndDrop.id(),     keys::DRAG_AND_DROP.id());
    }

    // ── Test 10: typed cast roundtrip using arbitrary test type ──────────────
    #[test]
    fn typed_cast_roundtrip_with_known_extension_key_pattern() {
        // Simulate registering + retrieving via a key whose id matches
        // KnownExtensions::EditorFactories (if not already registered).
        // Uses the real EDITOR_FACTORIES static key for the typed downcast.
        use crate::registry::keys::{EditorFactoryContribution, EDITOR_FACTORIES};

        let known_id = KnownExtensions::EditorFactories.id();
        // ids must match
        assert_eq!(known_id, EDITOR_FACTORIES.id());

        // typed downcast: if already registered from another parallel test, skip.
        if !Registry::knows(EDITOR_FACTORIES.id()) {
            Registry::register(&EDITOR_FACTORIES, EditorFactoryContribution);
        }
        // Must be retrievable and downcastable.
        let arc = Registry::as_(&EDITOR_FACTORIES).expect("EDITOR_FACTORIES must exist");
        // Verify knows() agrees.
        assert!(Registry::knows(EDITOR_FACTORIES.id()));
        // Arc points to correct type (trivially confirmed by successful as_<> downcast).
        drop(arc);
    }
}
