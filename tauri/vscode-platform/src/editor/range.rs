//! Range — a half-open [start, end) span between two Positions.
//! Mirrors Monaco's `IRange` interface.

use serde::{Deserialize, Serialize};

use super::position::Position;

/// A range in a text document between two positions.
///
/// The range is considered [start, end) in document order;
/// callers must ensure `start <= end` (use `Range::ordered` for safety).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

impl Range {
    /// Create a range. `start` and `end` are used as-is.
    pub fn new(start: Position, end: Position) -> Self {
        Range { start, end }
    }

    /// Create range ensuring `start <= end` by swapping if needed.
    pub fn ordered(a: Position, b: Position) -> Self {
        if a <= b {
            Range { start: a, end: b }
        } else {
            Range { start: b, end: a }
        }
    }

    /// True iff start == end (zero-length range / cursor position).
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    /// True iff `pos` is strictly inside (start <= pos < end) or the range is
    /// collapsed at that position.
    pub fn contains_position(&self, pos: Position) -> bool {
        pos >= self.start && pos < self.end
    }

    /// True iff `other` is fully contained within `self`.
    pub fn contains_range(&self, other: &Range) -> bool {
        other.start >= self.start && other.end <= self.end
    }

    /// True iff `self` and `other` share at least one character position.
    pub fn intersects(&self, other: &Range) -> bool {
        self.start < other.end && other.start < self.end
    }

    /// True iff `self` and `other` share any character or touch (adjacent).
    pub fn intersects_or_touches(&self, other: &Range) -> bool {
        self.start <= other.end && other.start <= self.end
    }
}

impl std::fmt::Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} → {}]", self.start, self.end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pos(l: u32, c: u32) -> Position {
        Position::new(l, c)
    }

    #[test]
    fn range_new_stores_endpoints() {
        let r = Range::new(pos(1, 1), pos(2, 5));
        assert_eq!(r.start, pos(1, 1));
        assert_eq!(r.end, pos(2, 5));
    }

    #[test]
    fn range_ordered_swaps_when_needed() {
        let r = Range::ordered(pos(3, 1), pos(1, 1));
        assert_eq!(r.start, pos(1, 1));
        assert_eq!(r.end, pos(3, 1));
    }

    #[test]
    fn range_is_empty_when_start_equals_end() {
        let r = Range::new(pos(2, 4), pos(2, 4));
        assert!(r.is_empty());
    }

    #[test]
    fn range_not_empty_when_span() {
        let r = Range::new(pos(1, 1), pos(1, 5));
        assert!(!r.is_empty());
    }

    #[test]
    fn range_contains_position_inside() {
        let r = Range::new(pos(1, 1), pos(1, 10));
        assert!(r.contains_position(pos(1, 5)));
    }

    #[test]
    fn range_contains_position_at_start() {
        let r = Range::new(pos(1, 1), pos(1, 10));
        assert!(r.contains_position(pos(1, 1)));
    }

    #[test]
    fn range_does_not_contain_position_at_end() {
        let r = Range::new(pos(1, 1), pos(1, 10));
        // end is exclusive
        assert!(!r.contains_position(pos(1, 10)));
    }

    #[test]
    fn range_contains_range() {
        let outer = Range::new(pos(1, 1), pos(5, 1));
        let inner = Range::new(pos(2, 1), pos(3, 1));
        assert!(outer.contains_range(&inner));
        assert!(!inner.contains_range(&outer));
    }

    #[test]
    fn range_intersects_overlapping() {
        let a = Range::new(pos(1, 1), pos(2, 5));
        let b = Range::new(pos(2, 1), pos(3, 1));
        assert!(a.intersects(&b));
        assert!(b.intersects(&a));
    }

    #[test]
    fn range_does_not_intersect_adjacent() {
        // a ends exactly where b starts → no overlap (end exclusive)
        let a = Range::new(pos(1, 1), pos(1, 5));
        let b = Range::new(pos(1, 5), pos(1, 10));
        assert!(!a.intersects(&b));
    }

    #[test]
    fn range_intersects_or_touches_adjacent() {
        let a = Range::new(pos(1, 1), pos(1, 5));
        let b = Range::new(pos(1, 5), pos(1, 10));
        assert!(a.intersects_or_touches(&b));
    }
}
