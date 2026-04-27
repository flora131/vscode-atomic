//! Decoration tracking stub.
//! Mirrors DecorationOptions and the IModelDecorationsChangedEvent surface from
//! src/vs/editor/common/model/textModel.ts and src/vscode-dts/vscode.d.ts.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Opaque decoration identifier (UUID-like string in production).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DecorationId(pub String);

impl DecorationId {
    pub fn new(id: impl Into<String>) -> Self {
        DecorationId(id.into())
    }
}

/// Visual / semantic options attached to a decoration range.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecorationOptions {
    /// CSS class name applied in the renderer.
    pub class_name: String,
    /// Hover message (markdown).
    pub hover_message: Option<String>,
    /// If true, decoration is shown even when range collapses.
    pub is_whole_line: bool,
}

impl DecorationOptions {
    pub fn new(class_name: impl Into<String>) -> Self {
        DecorationOptions {
            class_name: class_name.into(),
            hover_message: None,
            is_whole_line: false,
        }
    }
}

/// Range associated with a decoration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecorationRange {
    pub start_line: u32,
    pub start_col: u32,
    pub end_line: u32,
    pub end_col: u32,
}

/// A registered decoration entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decoration {
    pub id: DecorationId,
    pub range: DecorationRange,
    pub options: DecorationOptions,
}

/// Tracks decorations for a single TextModel.
/// Renderer subscribes to change events over Tauri event bus.
#[derive(Debug, Default)]
pub struct ModelDecorationsTracker {
    decorations: HashMap<DecorationId, Decoration>,
    next_id: u64,
}

impl ModelDecorationsTracker {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a new decoration. Returns its assigned id.
    pub fn register(
        &mut self,
        range: DecorationRange,
        options: DecorationOptions,
    ) -> DecorationId {
        self.next_id += 1;
        let id = DecorationId::new(format!("deco-{}", self.next_id));
        self.decorations.insert(
            id.clone(),
            Decoration {
                id: id.clone(),
                range,
                options,
            },
        );
        id
    }

    /// Remove a decoration by id. Returns true if it existed.
    pub fn remove(&mut self, id: &DecorationId) -> bool {
        self.decorations.remove(id).is_some()
    }

    /// Remove all decorations.
    pub fn clear(&mut self) {
        self.decorations.clear();
    }

    /// Number of currently registered decorations.
    pub fn len(&self) -> usize {
        self.decorations.len()
    }

    pub fn is_empty(&self) -> bool {
        self.decorations.is_empty()
    }

    /// Snapshot of all decoration ids.
    pub fn ids(&self) -> Vec<&DecorationId> {
        self.decorations.keys().collect()
    }

    /// Look up a decoration by id.
    pub fn get(&self, id: &DecorationId) -> Option<&Decoration> {
        self.decorations.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_range(sl: u32, sc: u32, el: u32, ec: u32) -> DecorationRange {
        DecorationRange { start_line: sl, start_col: sc, end_line: el, end_col: ec }
    }

    #[test]
    fn tracker_register_returns_unique_ids() {
        let mut tracker = ModelDecorationsTracker::new();
        let id1 = tracker.register(make_range(1, 1, 1, 5), DecorationOptions::new("warning"));
        let id2 = tracker.register(make_range(2, 1, 2, 10), DecorationOptions::new("error"));
        assert_ne!(id1, id2);
        assert_eq!(tracker.len(), 2);
    }

    #[test]
    fn tracker_get_returns_registered_decoration() {
        let mut tracker = ModelDecorationsTracker::new();
        let range = make_range(5, 1, 5, 20);
        let opts = DecorationOptions::new("highlight");
        let id = tracker.register(range.clone(), opts);
        let deco = tracker.get(&id).expect("decoration should exist");
        assert_eq!(deco.id, id);
        assert_eq!(deco.options.class_name, "highlight");
        assert_eq!(deco.range.start_line, 5);
    }

    #[test]
    fn tracker_remove_decrements_count() {
        let mut tracker = ModelDecorationsTracker::new();
        let id = tracker.register(make_range(1, 1, 1, 1), DecorationOptions::new("x"));
        assert_eq!(tracker.len(), 1);
        let removed = tracker.remove(&id);
        assert!(removed);
        assert_eq!(tracker.len(), 0);
    }

    #[test]
    fn tracker_remove_nonexistent_returns_false() {
        let mut tracker = ModelDecorationsTracker::new();
        let ghost = DecorationId::new("no-such-id");
        assert!(!tracker.remove(&ghost));
    }

    #[test]
    fn tracker_clear_removes_all() {
        let mut tracker = ModelDecorationsTracker::new();
        tracker.register(make_range(1, 1, 1, 1), DecorationOptions::new("a"));
        tracker.register(make_range(2, 1, 2, 1), DecorationOptions::new("b"));
        tracker.register(make_range(3, 1, 3, 1), DecorationOptions::new("c"));
        assert_eq!(tracker.len(), 3);
        tracker.clear();
        assert!(tracker.is_empty());
    }
}
