//! Undo stack — stores inverse edit operations for undo/redo.
//! Real implementation deferred; this provides the push/pop/clear surface.

use crate::editor::edit::SingleEditOperation;

/// An entry in the undo stack: a batch of inverse operations that reverse
/// a single `apply_edits` call.
#[derive(Debug, Clone)]
pub struct UndoEntry {
    /// Version id of the model *after* the edit was applied.
    pub version_id: u64,
    /// The inverse operations that, if applied, would undo this edit.
    pub inverse_edits: Vec<SingleEditOperation>,
}

/// A simple stack of undo entries.
///
/// Full undo/redo history semantics (as in `src/vs/editor/common/model/editStack.ts`)
/// are deferred; this stub provides push / pop / clear.
#[derive(Debug, Default)]
pub struct UndoStack {
    entries: Vec<UndoEntry>,
}

impl UndoStack {
    pub fn new() -> Self {
        UndoStack { entries: Vec::new() }
    }

    /// Push an undo entry onto the stack.
    pub fn push(&mut self, entry: UndoEntry) {
        self.entries.push(entry);
    }

    /// Pop the most recent undo entry. Returns `None` if the stack is empty.
    pub fn pop(&mut self) -> Option<UndoEntry> {
        self.entries.pop()
    }

    /// Discard all undo history.
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Number of entries currently on the stack.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Peek at the top entry without removing it.
    pub fn peek(&self) -> Option<&UndoEntry> {
        self.entries.last()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::{
        edit::SingleEditOperation,
        position::Position,
        range::Range,
    };

    fn make_entry(version_id: u64) -> UndoEntry {
        let p = Position::new(1, 1);
        UndoEntry {
            version_id,
            inverse_edits: vec![SingleEditOperation::new(Range::new(p, p), "")],
        }
    }

    #[test]
    fn undo_stack_starts_empty() {
        let stack = UndoStack::new();
        assert!(stack.is_empty());
        assert_eq!(stack.len(), 0);
    }

    #[test]
    fn undo_stack_push_increments_len() {
        let mut stack = UndoStack::new();
        stack.push(make_entry(1));
        assert_eq!(stack.len(), 1);
        stack.push(make_entry(2));
        assert_eq!(stack.len(), 2);
    }

    #[test]
    fn undo_stack_pop_returns_last_pushed() {
        let mut stack = UndoStack::new();
        stack.push(make_entry(1));
        stack.push(make_entry(2));
        let top = stack.pop().unwrap();
        assert_eq!(top.version_id, 2);
        assert_eq!(stack.len(), 1);
    }

    #[test]
    fn undo_stack_pop_empty_returns_none() {
        let mut stack = UndoStack::new();
        assert!(stack.pop().is_none());
    }

    #[test]
    fn undo_stack_clear_empties() {
        let mut stack = UndoStack::new();
        stack.push(make_entry(1));
        stack.push(make_entry(2));
        stack.clear();
        assert!(stack.is_empty());
    }

    #[test]
    fn undo_stack_peek_does_not_remove() {
        let mut stack = UndoStack::new();
        stack.push(make_entry(42));
        let peeked = stack.peek().unwrap();
        assert_eq!(peeked.version_id, 42);
        assert_eq!(stack.len(), 1);
    }
}
