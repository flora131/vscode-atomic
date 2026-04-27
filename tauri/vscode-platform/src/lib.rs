//! vscode-platform — port of src/vs/platform/ cross-cutting services.
//!
//! Placeholder. Concrete service implementations follow in tasks #3, #5, #6.

// Suppress pre-existing lint issues in modules owned by other workers.
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::inherent_to_string)]
#![allow(clippy::type_complexity)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::doc_overindented_list_items)]

pub mod debug;
pub mod editor;
pub mod files;
pub mod instantiation;
pub mod notebook;
pub mod registry;
pub mod scm;
pub mod storage;
pub mod terminal;
pub mod webview;

pub use instantiation::{
    InstantiationType, ServiceCollection, ServiceId, ServiceRegistry,
    ServiceAccessor as DIServiceAccessor,
};
pub use registry::{
    KnownExtensions, Registry, RegistryKey, ServiceAccessor, WorkbenchContribution,
    WorkbenchContributionRegistry, WorkbenchPhase, WORKBENCH_CONTRIBUTIONS,
};

#[cfg(test)]
mod tests {
    use super::ServiceRegistry;

    struct MyService {
        value: i32,
    }

    #[test]
    fn service_registry_register_and_retrieve() {
        let registry = ServiceRegistry::new();
        registry.register(MyService { value: 99 });
        let svc = registry.get::<MyService>().expect("service not found");
        assert_eq!(svc.value, 99);
    }

    #[test]
    fn service_registry_returns_none_for_unregistered() {
        let registry = ServiceRegistry::new();
        assert!(registry.get::<MyService>().is_none());
    }
}
