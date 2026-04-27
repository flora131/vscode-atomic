//! Minimal DI registry placeholder mirroring the createDecorator / registerSingleton
//! pattern from src/vs/platform/instantiation/common/instantiation.ts.
//!
//! A full implementation is delivered in task #3 (Implementing Rust DI container).

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Type-erased service registry.
#[derive(Default)]
pub struct ServiceRegistry {
    services: RwLock<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>,
}

impl ServiceRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a singleton service.
    pub fn register<T: Any + Send + Sync>(&self, service: T) {
        let mut map = self.services.write().unwrap();
        map.insert(TypeId::of::<T>(), Arc::new(service));
    }

    /// Retrieve a registered service.
    pub fn get<T: Any + Send + Sync>(&self) -> Option<Arc<T>> {
        let map = self.services.read().unwrap();
        map.get(&TypeId::of::<T>())
            .and_then(|arc| arc.clone().downcast::<T>().ok())
    }
}
