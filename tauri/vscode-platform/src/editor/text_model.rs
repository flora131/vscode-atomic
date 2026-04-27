//! TextModel — server-side authoritative document state backed by ropey::Rope.
//!
//! Mirrors the core surface of `src/vs/editor/common/model/textModel.ts`.
//! Position/Range use 1-based line and column numbers as in Monaco.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use ropey::Rope;
use serde::{Deserialize, Serialize};
use vscode_base::event::Emitter;

use super::edit::{EndOfLineSequence, SingleEditOperation};
use super::position::Position;
use super::range::Range;

// ─────────────────────────────────────────────────────────────────────────────
// Events
// ─────────────────────────────────────────────────────────────────────────────

/// Describes one change within a `TextModelChangedEvent`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TextModelContentChange {
    /// The range that was replaced.
    pub range: Range,
    /// Character offset of `range.start` in the document (0-based).
    pub range_offset: usize,
    /// Length in characters of the replaced region.
    pub range_length: usize,
    /// The replacement text.
    pub text: String,
}

/// Event fired by `TextModel::on_did_change_content` after each `apply_edits`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextModelChangedEvent {
    /// New version id after the edits.
    pub version_id: u64,
    /// Individual change records (one per edit operation applied).
    pub changes: Vec<TextModelContentChange>,
    /// EOL in use at the time of the event.
    pub eol: EndOfLineSequence,
}

// ─────────────────────────────────────────────────────────────────────────────
// TextModel
// ─────────────────────────────────────────────────────────────────────────────

/// Server-side authoritative text model backed by a `ropey::Rope`.
pub struct TextModel {
    rope: Rope,
    eol: EndOfLineSequence,
    version_id: Arc<AtomicU64>,
    on_did_change: Emitter<TextModelChangedEvent>,
}

impl std::fmt::Debug for TextModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TextModel")
            .field("version_id", &self.version_id.load(Ordering::SeqCst))
            .field("eol", &self.eol)
            .field("len_chars", &self.rope.len_chars())
            .finish()
    }
}

impl TextModel {
    // ── Construction ─────────────────────────────────────────────────────────

    /// Create a new model from the given text with the specified EOL sequence.
    pub fn new(text: &str, eol: EndOfLineSequence) -> Self {
        TextModel {
            rope: Rope::from_str(text),
            eol,
            version_id: Arc::new(AtomicU64::new(1)),
            on_did_change: Emitter::new(),
        }
    }

    // ── Version ───────────────────────────────────────────────────────────────

    /// Current version identifier. Incremented on each successful `apply_edits`.
    pub fn get_version_id(&self) -> u64 {
        self.version_id.load(Ordering::SeqCst)
    }

    // ── EOL ──────────────────────────────────────────────────────────────────

    /// Change the EOL sequence. Does NOT re-encode the stored rope; only affects
    /// subsequent `get_value()` output and events.
    pub fn set_eol(&mut self, eol: EndOfLineSequence) {
        self.eol = eol;
    }

    pub fn get_eol(&self) -> EndOfLineSequence {
        self.eol
    }

    // ── Value retrieval ───────────────────────────────────────────────────────

    /// Return the full document content, normalising newlines to the model EOL.
    pub fn get_value(&self) -> String {
        let raw = self.rope.to_string();
        normalise_eol(&raw, self.eol)
    }

    /// Return the text within `range`. Both endpoints are inclusive on the start
    /// and the end column is exclusive (matching Monaco semantics).
    ///
    /// Clamps silently to document bounds.
    pub fn get_value_in_range(&self, range: Range) -> String {
        let start_offset = self.offset_of(range.start);
        let end_offset = self.offset_of(range.end);
        if start_offset >= end_offset {
            return String::new();
        }
        let slice = self.rope.slice(start_offset..end_offset);
        let raw: String = slice.to_string();
        normalise_eol(&raw, self.eol)
    }

    // ── Line API ──────────────────────────────────────────────────────────────

    /// Number of lines in the document (always ≥ 1).
    ///
    /// A trailing newline does NOT create an extra empty line (mirrors Monaco).
    pub fn get_line_count(&self) -> u32 {
        let n = self.rope.len_lines();
        // ropey counts a trailing '\n' as starting an extra empty line.
        // If the last line the rope reports is empty AND the rope is non-empty,
        // subtract one to match Monaco behaviour.
        let last_is_phantom = n > 1
            && {
                let last = self.rope.line(n - 1);
                last.len_chars() == 0
            };
        if last_is_phantom {
            (n - 1) as u32
        } else {
            n as u32
        }
    }

    /// Return the content of a 1-based line number (without the line terminator).
    ///
    /// Panics if `line` is out of range.
    pub fn get_line_content(&self, line: u32) -> String {
        let idx = (line as usize).checked_sub(1).expect("line is 1-based");
        let rope_line = self.rope.line(idx);
        // Strip the trailing newline if present.
        let s: String = rope_line.to_string();
        s.trim_end_matches(['\n', '\r']).to_owned()
    }

    /// Return the length (char count, excluding EOL) of a 1-based line.
    pub fn get_line_length(&self, line: u32) -> u32 {
        self.get_line_content(line).chars().count() as u32
    }

    // ── Offset / Position conversion ─────────────────────────────────────────

    /// Convert a 1-based `Position` to a 0-based character offset in the rope.
    ///
    /// Clamps column to the line length.
    pub fn get_offset_at(&self, pos: Position) -> usize {
        let line_idx = (pos.line as usize).saturating_sub(1);
        let line_char_start = self.rope.line_to_char(line_idx);
        let col_offset = (pos.column as usize).saturating_sub(1);
        let line_len = self.get_line_length(pos.line) as usize;
        line_char_start + col_offset.min(line_len)
    }

    /// Convert a 0-based character offset to a 1-based `Position`.
    ///
    /// Clamps to document bounds.
    pub fn get_position_at(&self, offset: usize) -> Position {
        let offset = offset.min(self.rope.len_chars());
        let line_idx = self.rope.char_to_line(offset);
        let line_char_start = self.rope.line_to_char(line_idx);
        let col = offset - line_char_start;
        Position::new((line_idx + 1) as u32, (col + 1) as u32)
    }

    // ── Edit application ─────────────────────────────────────────────────────

    /// Apply a batch of edits, returning the inverse operations for undo.
    ///
    /// Edits are sorted descending by start offset before application so that
    /// later edits don't shift the offsets of earlier ones.
    ///
    /// Bumps `version_id` by 1 and fires `on_did_change_content`.
    pub fn apply_edits(
        &mut self,
        edits: Vec<SingleEditOperation>,
    ) -> Vec<SingleEditOperation> {
        // Compute offsets for all edits before any mutation.
        let mut indexed: Vec<(usize, usize, SingleEditOperation)> = edits
            .into_iter()
            .map(|op| {
                let start_offset = self.offset_of(op.range.start);
                let end_offset = self.offset_of(op.range.end);
                (start_offset, end_offset, op)
            })
            .collect();

        // Sort descending by start offset.
        indexed.sort_by(|a, b| b.0.cmp(&a.0));

        let mut inverse_edits = Vec::with_capacity(indexed.len());
        let mut changes = Vec::with_capacity(indexed.len());

        for (start_offset, end_offset, op) in indexed {
            // Capture old text (will be the inverse's replacement text).
            let old_text: String = if start_offset < end_offset {
                self.rope.slice(start_offset..end_offset).to_string()
            } else {
                String::new()
            };

            let range_length = end_offset - start_offset;

            // Apply: remove old, insert new.
            if start_offset < end_offset {
                self.rope.remove(start_offset..end_offset);
            }
            if !op.text.is_empty() {
                self.rope.insert(start_offset, &op.text);
            }

            // Compute the inverse operation's range (after the edit).
            let new_len_chars = op.text.chars().count();
            let inv_end_offset = start_offset + new_len_chars;
            let inv_start_pos = self.get_position_at(start_offset);
            let inv_end_pos = self.get_position_at(inv_end_offset);
            let inv_range = Range::new(inv_start_pos, inv_end_pos);

            inverse_edits.push(SingleEditOperation {
                range: inv_range,
                text: old_text,
                force_move_markers: op.force_move_markers,
            });

            changes.push(TextModelContentChange {
                range: op.range,
                range_offset: start_offset,
                range_length,
                text: op.text,
            });
        }

        // Bump version.
        let new_version = self.version_id.fetch_add(1, Ordering::SeqCst) + 1;

        // Fire event.
        self.on_did_change.fire(&TextModelChangedEvent {
            version_id: new_version,
            changes,
            eol: self.eol,
        });

        inverse_edits
    }

    // ── Event subscription ────────────────────────────────────────────────────

    /// Subscribe to content-change events.
    /// Returns a `ListenerHandle`; drop it to unsubscribe.
    pub fn on_did_change_content(
        &self,
        f: impl Fn(&TextModelChangedEvent) + Send + Sync + 'static,
    ) -> vscode_base::event::ListenerHandle<TextModelChangedEvent> {
        self.on_did_change.add_listener(f)
    }

    // ── Internal helpers ──────────────────────────────────────────────────────

    /// Convert a `Position` to a rope char offset (0-based). Does NOT clamp to
    /// line length — used only internally where we need exact offsets.
    fn offset_of(&self, pos: Position) -> usize {
        let line_idx = (pos.line as usize).saturating_sub(1);
        // Clamp line index to valid range.
        let line_idx = line_idx.min(self.rope.len_lines().saturating_sub(1));
        let line_char_start = self.rope.line_to_char(line_idx);
        let col_offset = (pos.column as usize).saturating_sub(1);
        // Clamp to line length (without newline).
        let line_content_len = self.get_line_length(line_idx as u32 + 1) as usize;
        line_char_start + col_offset.min(line_content_len)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Normalise all newline sequences in `s` to the target EOL.
fn normalise_eol(s: &str, eol: EndOfLineSequence) -> String {
    match eol {
        EndOfLineSequence::LF => {
            // Replace \r\n with \n first, then lone \r with \n.
            s.replace("\r\n", "\n").replace('\r', "\n")
        }
        EndOfLineSequence::CRLF => {
            // Replace \r\n → tmp, \r → \n, then \n → \r\n, then revert tmp.
            let tmp = "\x00CRLF\x00";
            s.replace("\r\n", tmp)
                .replace('\r', "\n")
                .replace('\n', "\r\n")
                .replace(tmp, "\r\n")
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    fn lf() -> EndOfLineSequence {
        EndOfLineSequence::LF
    }
    fn crlf() -> EndOfLineSequence {
        EndOfLineSequence::CRLF
    }
    fn pos(l: u32, c: u32) -> Position {
        Position::new(l, c)
    }
    fn range(sl: u32, sc: u32, el: u32, ec: u32) -> Range {
        Range::new(pos(sl, sc), pos(el, ec))
    }

    // ── Round-trip ───────────────────────────────────────────────────────────

    #[test]
    fn new_get_value_round_trip_lf() {
        let text = "hello\nworld";
        let m = TextModel::new(text, lf());
        assert_eq!(m.get_value(), text);
    }

    #[test]
    fn new_get_value_round_trip_empty() {
        let m = TextModel::new("", lf());
        assert_eq!(m.get_value(), "");
    }

    // ── Line count ───────────────────────────────────────────────────────────

    #[test]
    fn get_line_count_no_trailing_newline() {
        let m = TextModel::new("a\nb\nc", lf());
        assert_eq!(m.get_line_count(), 3);
    }

    #[test]
    fn get_line_count_trailing_newline_does_not_add_line() {
        // Monaco: "a\nb\n" has 2 lines, not 3.
        let m = TextModel::new("a\nb\n", lf());
        assert_eq!(m.get_line_count(), 2);
    }

    #[test]
    fn get_line_count_single_line() {
        let m = TextModel::new("hello", lf());
        assert_eq!(m.get_line_count(), 1);
    }

    #[test]
    fn get_line_count_empty_doc() {
        let m = TextModel::new("", lf());
        assert_eq!(m.get_line_count(), 1);
    }

    // ── Line content ─────────────────────────────────────────────────────────

    #[test]
    fn get_line_content_strips_newline() {
        let m = TextModel::new("hello\nworld", lf());
        assert_eq!(m.get_line_content(1), "hello");
        assert_eq!(m.get_line_content(2), "world");
    }

    #[test]
    fn get_line_length_correct() {
        let m = TextModel::new("hello\nworld", lf());
        assert_eq!(m.get_line_length(1), 5);
        assert_eq!(m.get_line_length(2), 5);
    }

    // ── Offset / Position conversion ─────────────────────────────────────────

    #[test]
    fn get_offset_at_start_of_doc() {
        let m = TextModel::new("hello\nworld", lf());
        assert_eq!(m.get_offset_at(pos(1, 1)), 0);
    }

    #[test]
    fn get_offset_at_mid_line() {
        let m = TextModel::new("hello\nworld", lf());
        // 'w' is char index 6 (0-based): 5 chars + '\n'
        assert_eq!(m.get_offset_at(pos(2, 1)), 6);
    }

    #[test]
    fn get_position_at_zero() {
        let m = TextModel::new("hello\nworld", lf());
        assert_eq!(m.get_position_at(0), pos(1, 1));
    }

    #[test]
    fn get_position_at_second_line() {
        let m = TextModel::new("hello\nworld", lf());
        assert_eq!(m.get_position_at(6), pos(2, 1));
    }

    #[test]
    fn offset_and_position_are_inverses() {
        let m = TextModel::new("first\nsecond\nthird", lf());
        for (l, c) in [(1u32, 1u32), (1, 3), (1, 6), (2, 1), (2, 4), (3, 1), (3, 5)] {
            let p = pos(l, c);
            let off = m.get_offset_at(p);
            let back = m.get_position_at(off);
            assert_eq!(back, p, "round-trip failed for ({l},{c})");
        }
    }

    // ── get_value_in_range ────────────────────────────────────────────────────

    #[test]
    fn get_value_in_range_single_line() {
        let m = TextModel::new("hello world", lf());
        // columns 1..6 → "hello"
        assert_eq!(m.get_value_in_range(range(1, 1, 1, 6)), "hello");
    }

    #[test]
    fn get_value_in_range_multi_line() {
        let m = TextModel::new("hello\nworld", lf());
        // from (1,1) to (2,6) → "hello\nworld"
        assert_eq!(m.get_value_in_range(range(1, 1, 2, 6)), "hello\nworld");
    }

    #[test]
    fn get_value_in_range_empty_range() {
        let m = TextModel::new("hello", lf());
        assert_eq!(m.get_value_in_range(range(1, 3, 1, 3)), "");
    }

    // ── apply_edits ───────────────────────────────────────────────────────────

    #[test]
    fn apply_edits_insert_text() {
        let mut m = TextModel::new("hello", lf());
        let edits = vec![SingleEditOperation::new(range(1, 6, 1, 6), " world")];
        m.apply_edits(edits);
        assert_eq!(m.get_value(), "hello world");
    }

    #[test]
    fn apply_edits_replace_text() {
        let mut m = TextModel::new("hello world", lf());
        let edits = vec![SingleEditOperation::new(range(1, 7, 1, 12), "Rust")];
        m.apply_edits(edits);
        assert_eq!(m.get_value(), "hello Rust");
    }

    #[test]
    fn apply_edits_delete_text() {
        let mut m = TextModel::new("hello world", lf());
        let edits = vec![SingleEditOperation::deletion(range(1, 6, 1, 12))];
        m.apply_edits(edits);
        assert_eq!(m.get_value(), "hello");
    }

    #[test]
    fn apply_edits_produces_correct_inverse() {
        let mut m = TextModel::new("hello world", lf());
        let edits = vec![SingleEditOperation::new(range(1, 7, 1, 12), "Rust")];
        let inverse = m.apply_edits(edits);
        assert_eq!(inverse.len(), 1);
        // Apply inverse to recover original.
        m.apply_edits(inverse);
        assert_eq!(m.get_value(), "hello world");
    }

    #[test]
    fn apply_edits_bumps_version_id() {
        let mut m = TextModel::new("text", lf());
        assert_eq!(m.get_version_id(), 1);
        m.apply_edits(vec![SingleEditOperation::new(range(1, 1, 1, 1), "X")]);
        assert_eq!(m.get_version_id(), 2);
        m.apply_edits(vec![SingleEditOperation::new(range(1, 1, 1, 1), "Y")]);
        assert_eq!(m.get_version_id(), 3);
    }

    #[test]
    fn apply_edits_multiple_sorted_descending() {
        // Two non-overlapping edits: one on line 1, one on line 2.
        // Result should be independent of input order.
        let mut m = TextModel::new("aaa\nbbb", lf());
        let edits = vec![
            SingleEditOperation::new(range(1, 1, 1, 4), "AAA"),
            SingleEditOperation::new(range(2, 1, 2, 4), "BBB"),
        ];
        m.apply_edits(edits);
        assert_eq!(m.get_value(), "AAA\nBBB");
    }

    // ── on_did_change_content ────────────────────────────────────────────────

    #[test]
    fn on_did_change_fires_once_per_apply_edits() {
        let mut m = TextModel::new("hello", lf());
        let call_count = Arc::new(Mutex::new(0u32));
        let c = call_count.clone();
        let _handle = m.on_did_change_content(move |_| *c.lock().unwrap() += 1);

        m.apply_edits(vec![SingleEditOperation::new(range(1, 6, 1, 6), " world")]);
        assert_eq!(*call_count.lock().unwrap(), 1);

        m.apply_edits(vec![SingleEditOperation::new(range(1, 1, 1, 1), "!")]);
        assert_eq!(*call_count.lock().unwrap(), 2);
    }

    #[test]
    fn on_did_change_event_contains_correct_version() {
        let mut m = TextModel::new("hi", lf());
        let seen_version = Arc::new(Mutex::new(0u64));
        let sv = seen_version.clone();
        let _handle = m.on_did_change_content(move |e| *sv.lock().unwrap() = e.version_id);

        m.apply_edits(vec![SingleEditOperation::new(range(1, 3, 1, 3), "!")]);
        assert_eq!(*seen_version.lock().unwrap(), 2);
    }

    // ── CRLF EOL handling ────────────────────────────────────────────────────

    #[test]
    fn crlf_get_value_normalises_output() {
        // Stored as LF internally; get_value normalises to CRLF.
        let m = TextModel::new("line1\nline2", crlf());
        assert_eq!(m.get_value(), "line1\r\nline2");
    }

    #[test]
    fn set_eol_changes_subsequent_get_value() {
        let mut m = TextModel::new("a\nb", lf());
        assert_eq!(m.get_value(), "a\nb");
        m.set_eol(crlf());
        assert_eq!(m.get_value(), "a\r\nb");
    }

    #[test]
    fn crlf_line_count_correct() {
        let m = TextModel::new("a\r\nb\r\nc", crlf());
        assert_eq!(m.get_line_count(), 3);
    }

    // ── normalise_eol helper ─────────────────────────────────────────────────

    #[test]
    fn normalise_eol_lf_strips_cr() {
        assert_eq!(normalise_eol("a\r\nb", EndOfLineSequence::LF), "a\nb");
    }

    #[test]
    fn normalise_eol_crlf_adds_cr() {
        assert_eq!(normalise_eol("a\nb", EndOfLineSequence::CRLF), "a\r\nb");
    }

    #[test]
    fn normalise_eol_crlf_idempotent() {
        assert_eq!(normalise_eol("a\r\nb", EndOfLineSequence::CRLF), "a\r\nb");
    }
}
