//! vscode-platform — port of src/vs/platform/ cross-cutting services.
//!
//! Placeholder. Concrete service implementations follow in tasks #3, #5, #6.

pub mod instantiation;

pub use instantiation::ServiceRegistry;

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
