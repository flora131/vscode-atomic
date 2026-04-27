//! WebviewRegistry — panel lifecycle store.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use super::webview::WebviewPanel;

/// Central registry of live WebviewPanels.
#[derive(Default)]
pub struct WebviewRegistry {
    panels: Mutex<HashMap<Uuid, Arc<WebviewPanel>>>,
}

impl WebviewRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a panel. Returns its id.
    pub fn register_panel(&self, panel: Arc<WebviewPanel>) -> Uuid {
        let id = panel.id;
        self.panels.lock().expect("registry poisoned").insert(id, panel);
        id
    }

    /// Remove a panel by id.
    pub fn unregister(&self, id: Uuid) -> Option<Arc<WebviewPanel>> {
        self.panels.lock().expect("registry poisoned").remove(&id)
    }

    /// Retrieve a panel by id.
    pub fn get(&self, id: Uuid) -> Option<Arc<WebviewPanel>> {
        self.panels.lock().expect("registry poisoned").get(&id).cloned()
    }

    /// Count of active panels.
    pub fn len(&self) -> usize {
        self.panels.lock().expect("registry poisoned").len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::webview::webview::WebviewPanel;

    #[test]
    fn registry_register_get_unregister_round_trip() {
        let registry = WebviewRegistry::new();
        let panel = WebviewPanel::new("test.view", "Test");
        let id = panel.id;

        let registered_id = registry.register_panel(panel);
        assert_eq!(registered_id, id);

        let got = registry.get(id).expect("panel should be found after register");
        assert_eq!(got.id, id);

        let removed = registry.unregister(id).expect("panel should be removable");
        assert_eq!(removed.id, id);

        assert!(registry.get(id).is_none(), "panel should be gone after unregister");
    }

    #[test]
    fn registry_get_nonexistent_returns_none() {
        let registry = WebviewRegistry::new();
        assert!(registry.get(Uuid::new_v4()).is_none());
    }

    #[test]
    fn registry_holds_multiple_panels() {
        let registry = WebviewRegistry::new();
        let p1 = WebviewPanel::new("view.a", "A");
        let p2 = WebviewPanel::new("view.b", "B");
        let id1 = p1.id;
        let id2 = p2.id;

        registry.register_panel(p1);
        registry.register_panel(p2);

        assert_eq!(registry.len(), 2);
        assert!(registry.get(id1).is_some());
        assert!(registry.get(id2).is_some());
    }

    #[test]
    fn registry_unregister_nonexistent_returns_none() {
        let registry = WebviewRegistry::new();
        assert!(registry.unregister(Uuid::new_v4()).is_none());
    }
}
