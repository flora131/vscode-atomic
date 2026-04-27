//! Edit operations — mirrors Monaco's `ISingleEditOperation` and EOL enum.

use serde::{Deserialize, Serialize};

use super::range::Range;

/// End-of-line sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EndOfLineSequence {
    /// Unix-style newline `\n`.
    LF,
    /// Windows-style newline `\r\n`.
    CRLF,
}

impl EndOfLineSequence {
    /// Return the string representation of the EOL sequence.
    pub fn as_str(self) -> &'static str {
        match self {
            EndOfLineSequence::LF => "\n",
            EndOfLineSequence::CRLF => "\r\n",
        }
    }
}

impl Default for EndOfLineSequence {
    fn default() -> Self {
        EndOfLineSequence::LF
    }
}

/// A single edit operation: replace `range` with `text`.
///
/// Mirrors Monaco's `ISingleEditOperation`.
/// An empty `text` string is a deletion; an empty `range` (start == end) is an insertion.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SingleEditOperation {
    /// The range of text to replace.
    pub range: Range,
    /// The replacement text. Use `""` for pure deletion.
    pub text: String,
    /// When true, sticky markers that are at the edit boundary move with the edit.
    pub force_move_markers: bool,
}

impl SingleEditOperation {
    pub fn new(range: Range, text: impl Into<String>) -> Self {
        SingleEditOperation {
            range,
            text: text.into(),
            force_move_markers: false,
        }
    }

    pub fn deletion(range: Range) -> Self {
        SingleEditOperation::new(range, "")
    }

    pub fn insertion(at: super::position::Position, text: impl Into<String>) -> Self {
        SingleEditOperation::new(Range::new(at, at), text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::position::Position;

    fn pos(l: u32, c: u32) -> Position {
        Position::new(l, c)
    }
    fn range(sl: u32, sc: u32, el: u32, ec: u32) -> Range {
        Range::new(pos(sl, sc), pos(el, ec))
    }

    #[test]
    fn eol_lf_str() {
        assert_eq!(EndOfLineSequence::LF.as_str(), "\n");
    }

    #[test]
    fn eol_crlf_str() {
        assert_eq!(EndOfLineSequence::CRLF.as_str(), "\r\n");
    }

    #[test]
    fn single_edit_new_stores_fields() {
        let op = SingleEditOperation::new(range(1, 1, 1, 5), "hello");
        assert_eq!(op.text, "hello");
        assert_eq!(op.range.start, pos(1, 1));
        assert!(!op.force_move_markers);
    }

    #[test]
    fn single_edit_deletion_empty_text() {
        let op = SingleEditOperation::deletion(range(2, 1, 2, 10));
        assert_eq!(op.text, "");
    }

    #[test]
    fn single_edit_insertion_collapsed_range() {
        let op = SingleEditOperation::insertion(pos(3, 5), "X");
        assert!(op.range.is_empty());
        assert_eq!(op.text, "X");
    }
}
