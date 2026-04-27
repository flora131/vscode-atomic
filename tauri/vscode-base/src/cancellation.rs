//! CancellationToken — thin wrapper over `tokio_util::sync::CancellationToken`.
//!
//! Mirrors `src/vs/base/common/cancellation.ts`:
//! - `CancellationTokenSource` — creates and owns a cancellable token.
//! - `child_token()` — propagates cancellation from parent to child.
//! - `is_cancelled()` / `cancel()` — synchronous state checks.

use tokio_util::sync::CancellationToken as TokioCT;

// ─────────────────────────────────────────────────────────────────────────────
// VsCancellationToken
// ─────────────────────────────────────────────────────────────────────────────

/// A cancellable token. Cloneable — all clones share the same cancellation state.
#[derive(Clone, Debug)]
pub struct VsCancellationToken {
    inner: TokioCT,
}

impl VsCancellationToken {
    /// Returns `true` if this token has been cancelled.
    pub fn is_cancelled(&self) -> bool {
        self.inner.is_cancelled()
    }

    /// Await cancellation asynchronously.
    pub async fn cancelled(&self) {
        self.inner.cancelled().await
    }

    /// Create a child token: cancelling the parent also cancels children.
    pub fn child_token(&self) -> VsCancellationToken {
        VsCancellationToken { inner: self.inner.child_token() }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// CancellationTokenSource
// ─────────────────────────────────────────────────────────────────────────────

/// Owns and controls a `VsCancellationToken`.
pub struct CancellationTokenSource {
    inner: TokioCT,
}

impl CancellationTokenSource {
    pub fn new() -> Self {
        Self { inner: TokioCT::new() }
    }

    /// Create a child source whose token is automatically cancelled when `parent` is.
    pub fn child_of(parent: &VsCancellationToken) -> Self {
        Self { inner: parent.inner.child_token() }
    }

    /// Get the token (cloneable, shareable).
    pub fn token(&self) -> VsCancellationToken {
        VsCancellationToken { inner: self.inner.clone() }
    }

    /// Cancel the token.
    pub fn cancel(&self) {
        self.inner.cancel();
    }

    /// Cancel and consume the source.
    pub fn dispose_cancelled(self) {
        self.inner.cancel();
    }
}

impl Default for CancellationTokenSource {
    fn default() -> Self {
        Self::new()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // Test 1 — basic cancel/is_cancelled.
    #[test]
    fn basic_cancel() {
        let src = CancellationTokenSource::new();
        let token = src.token();
        assert!(!token.is_cancelled());
        src.cancel();
        assert!(token.is_cancelled());
    }

    // Test 2 — child token cancelled when parent is cancelled.
    #[test]
    fn child_cancelled_by_parent() {
        let parent_src = CancellationTokenSource::new();
        let parent_token = parent_src.token();
        let child_token = parent_token.child_token();

        assert!(!child_token.is_cancelled());
        parent_src.cancel();
        assert!(child_token.is_cancelled(), "child should be cancelled by parent");
    }

    // Test 3 — cancelling child does NOT cancel parent.
    #[test]
    fn child_cancel_does_not_affect_parent() {
        let parent_src = CancellationTokenSource::new();
        let parent_token = parent_src.token();
        let child_src = CancellationTokenSource::child_of(&parent_token);

        child_src.cancel();
        assert!(child_src.token().is_cancelled(), "child is cancelled");
        assert!(!parent_token.is_cancelled(), "parent must stay alive");
    }

    // Test 4 — multiple clones share state.
    #[test]
    fn cloned_tokens_share_state() {
        let src = CancellationTokenSource::new();
        let t1 = src.token();
        let t2 = t1.clone();
        src.cancel();
        assert!(t1.is_cancelled());
        assert!(t2.is_cancelled());
    }
}
