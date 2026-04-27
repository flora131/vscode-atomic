//! Piece-tree / rope buffer placeholder.
//! Uses `ropey` crate for efficient UTF-8 rope storage.
//! Full piece-tree (as in src/vs/editor/common/model/pieceTreeTextBuffer/) deferred.

use ropey::Rope;

/// Rope-backed text buffer wrapping the `ropey` crate.
/// Acts as the in-memory backing store for TextModel content.
pub struct RopeBuffer {
    rope: Rope,
}

impl RopeBuffer {
    /// Create empty buffer.
    pub fn new() -> Self {
        RopeBuffer { rope: Rope::new() }
    }

    /// Create buffer from initial text.
    pub fn from_str(text: &str) -> Self {
        RopeBuffer { rope: Rope::from_str(text) }
    }

    /// Insert `text` at byte offset `char_idx` (char index, not byte offset).
    pub fn insert(&mut self, char_idx: usize, text: &str) {
        self.rope.insert(char_idx, text);
    }

    /// Remove chars in `[start, end)` (char indices).
    pub fn remove(&mut self, start: usize, end: usize) {
        self.rope.remove(start..end);
    }

    /// Total character count.
    pub fn len_chars(&self) -> usize {
        self.rope.len_chars()
    }

    /// Number of lines (rope counts trailing newline as extra line).
    pub fn len_lines(&self) -> usize {
        self.rope.len_lines()
    }

    /// Collect full content as a String.
    pub fn to_string(&self) -> String {
        self.rope.to_string()
    }
}

impl Default for RopeBuffer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buffer_new_is_empty() {
        let buf = RopeBuffer::new();
        assert_eq!(buf.len_chars(), 0);
    }

    #[test]
    fn buffer_from_str_preserves_content() {
        let buf = RopeBuffer::from_str("hello world");
        assert_eq!(buf.to_string(), "hello world");
        assert_eq!(buf.len_chars(), 11);
    }

    #[test]
    fn buffer_insert_at_start() {
        let mut buf = RopeBuffer::from_str("world");
        buf.insert(0, "hello ");
        assert_eq!(buf.to_string(), "hello world");
    }

    #[test]
    fn buffer_insert_at_end() {
        let mut buf = RopeBuffer::from_str("hello");
        buf.insert(5, " world");
        assert_eq!(buf.to_string(), "hello world");
    }

    #[test]
    fn buffer_remove_range() {
        let mut buf = RopeBuffer::from_str("hello world");
        buf.remove(5, 11);
        assert_eq!(buf.to_string(), "hello");
    }

    #[test]
    fn buffer_multiline_line_count() {
        let buf = RopeBuffer::from_str("line1\nline2\nline3");
        // ropey counts lines including the trailing one after last \n
        assert!(buf.len_lines() >= 3);
    }
}
