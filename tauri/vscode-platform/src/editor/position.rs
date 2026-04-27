//! Position — 1-based (line, column) coordinate in a text document.
//! Mirrors the TypeScript `IPosition` interface from Monaco.

use serde::{Deserialize, Serialize};

/// A position in a text document expressed as 1-based line and column numbers.
///
/// Mirrors Monaco's `IPosition { lineNumber: number; column: number }`.
/// Both components are 1-based; (1,1) is the very first character.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Position {
    /// 1-based line number.
    pub line: u32,
    /// 1-based column (UTF-16 code-unit index, matching Monaco convention).
    pub column: u32,
}

impl Position {
    pub fn new(line: u32, column: u32) -> Self {
        Position { line, column }
    }

    /// Returns true if this position is before `other` in document order.
    pub fn is_before(&self, other: &Position) -> bool {
        self < other
    }

    /// Returns true if this position is before or equal to `other`.
    pub fn is_before_or_equal(&self, other: &Position) -> bool {
        self <= other
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.line
            .cmp(&other.line)
            .then_with(|| self.column.cmp(&other.column))
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.line, self.column)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_new_stores_fields() {
        let p = Position::new(3, 7);
        assert_eq!(p.line, 3);
        assert_eq!(p.column, 7);
    }

    #[test]
    fn position_ordering_by_line_then_column() {
        let a = Position::new(1, 10);
        let b = Position::new(2, 1);
        assert!(a < b);
        let c = Position::new(2, 5);
        let d = Position::new(2, 10);
        assert!(c < d);
    }

    #[test]
    fn position_equal() {
        let a = Position::new(5, 3);
        let b = Position::new(5, 3);
        assert_eq!(a, b);
    }

    #[test]
    fn position_is_before() {
        let a = Position::new(1, 1);
        let b = Position::new(1, 2);
        assert!(a.is_before(&b));
        assert!(!b.is_before(&a));
    }

    #[test]
    fn position_is_before_or_equal_equal() {
        let a = Position::new(3, 3);
        assert!(a.is_before_or_equal(&a));
    }

    #[test]
    fn position_sort() {
        let mut positions = vec![
            Position::new(3, 1),
            Position::new(1, 5),
            Position::new(2, 2),
        ];
        positions.sort();
        assert_eq!(positions[0], Position::new(1, 5));
        assert_eq!(positions[1], Position::new(2, 2));
        assert_eq!(positions[2], Position::new(3, 1));
    }
}
