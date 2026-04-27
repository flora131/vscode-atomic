//! TextModel — server-side authoritative text model stub.
//! Mirrors the surface of src/vs/editor/common/model/textModel.ts.

use serde::{Deserialize, Serialize};

/// Single text edit operation (range + replacement text).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TextEdit {
    /// 1-based start line.
    pub start_line: u32,
    /// 1-based start column.
    pub start_col: u32,
    /// 1-based end line (inclusive).
    pub end_line: u32,
    /// 1-based end column (inclusive).
    pub end_col: u32,
    /// Replacement text (empty string = deletion).
    pub text: String,
}

/// Event emitted after edits are applied.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelChangedEvent {
    pub model_id: String,
    pub version: u64,
    pub changes: Vec<TextEdit>,
}

/// Server-side authoritative representation of an open document.
#[derive(Debug)]
pub struct TextModel {
    pub id: String,
    pub language_id: String,
    pub version: u64,
    pub line_count: usize,
    /// Raw content lines (0-indexed).
    lines: Vec<String>,
}

impl TextModel {
    /// Create a new TextModel from initial content.
    pub fn new(id: impl Into<String>, language_id: impl Into<String>, content: &str) -> Self {
        let lines: Vec<String> = content.lines().map(|l| l.to_owned()).collect();
        let line_count = lines.len().max(1);
        TextModel {
            id: id.into(),
            language_id: language_id.into(),
            version: 1,
            line_count,
            lines,
        }
    }

    /// Apply a batch of edits, bump version, return change event.
    ///
    /// Edits are applied in the order given. For this stub only single-line
    /// replacements are fully implemented; multi-line semantics are deferred.
    pub fn apply_edits(&mut self, edits: Vec<TextEdit>) -> ModelChangedEvent {
        for edit in &edits {
            let idx = (edit.start_line as usize).saturating_sub(1);
            if idx < self.lines.len() {
                // Stub: replace entire line content with edit.text.
                self.lines[idx] = edit.text.clone();
            } else {
                self.lines.push(edit.text.clone());
                self.line_count = self.lines.len();
            }
        }
        self.version += 1;
        self.line_count = self.lines.len().max(1);
        ModelChangedEvent {
            model_id: self.id.clone(),
            version: self.version,
            changes: edits,
        }
    }

    /// Read a line by 1-based line number. Returns None if out of range.
    pub fn get_line(&self, line: usize) -> Option<&str> {
        self.lines.get(line.saturating_sub(1)).map(|s| s.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn model_new_sets_id_language_and_line_count() {
        let m = TextModel::new("file:///foo.rs", "rust", "fn main() {}\nlet x = 1;");
        assert_eq!(m.id, "file:///foo.rs");
        assert_eq!(m.language_id, "rust");
        assert_eq!(m.line_count, 2);
        assert_eq!(m.version, 1);
    }

    #[test]
    fn model_apply_edits_bumps_version_and_returns_event() {
        let mut m = TextModel::new("file:///a.ts", "typescript", "const x = 1;");
        let edit = TextEdit {
            start_line: 1,
            start_col: 1,
            end_line: 1,
            end_col: 12,
            text: "const x = 42;".to_string(),
        };
        let event = m.apply_edits(vec![edit.clone()]);
        assert_eq!(event.version, 2);
        assert_eq!(event.model_id, "file:///a.ts");
        assert_eq!(event.changes.len(), 1);
        assert_eq!(m.get_line(1), Some("const x = 42;"));
    }

    #[test]
    fn model_apply_multiple_edits_accumulates() {
        let mut m = TextModel::new("id", "plaintext", "line1\nline2\nline3");
        let edits = vec![
            TextEdit { start_line: 1, start_col: 1, end_line: 1, end_col: 5, text: "CHANGED1".into() },
            TextEdit { start_line: 3, start_col: 1, end_line: 3, end_col: 5, text: "CHANGED3".into() },
        ];
        let event = m.apply_edits(edits);
        assert_eq!(event.version, 2);
        assert_eq!(m.get_line(1), Some("CHANGED1"));
        assert_eq!(m.get_line(2), Some("line2"));
        assert_eq!(m.get_line(3), Some("CHANGED3"));
    }

    #[test]
    fn model_get_line_returns_none_out_of_range() {
        let m = TextModel::new("id", "rust", "hello");
        assert!(m.get_line(99).is_none());
    }
}
