//! Barrier/BarrierOpener — port of cli/src/util/sync.rs.
//!
//! A Barrier is a one-shot gate: it can be opened exactly once and carries a value.
//! All waiters receive the same value.

use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::watch;

// ─────────────────────────────────────────────
// Receivable trait
// ─────────────────────────────────────────────

/// Trait for types that can yield messages asynchronously.
#[async_trait]
pub trait Receivable<T>: Send {
    async fn recv_msg(&mut self) -> Option<T>;
}

// ─────────────────────────────────────────────
// Barrier / BarrierOpener
// ─────────────────────────────────────────────

/// A one-shot gate that can be waited on.
/// Cloning creates another waiter on the same gate.
#[derive(Clone)]
pub struct Barrier<T: Clone>(watch::Receiver<Option<T>>);

impl<T: Clone + Send + Sync + 'static> Barrier<T> {
    /// Block until the barrier is opened, returning the value.
    pub async fn wait(&mut self) -> Result<T, watch::error::RecvError> {
        loop {
            // If already open, return immediately.
            if let Some(v) = self.0.borrow().clone() {
                return Ok(v);
            }
            self.0.changed().await?;
            if let Some(v) = self.0.borrow().clone() {
                return Ok(v);
            }
        }
    }

    /// Returns `true` if the barrier has been opened.
    pub fn is_open(&self) -> bool {
        self.0.borrow().is_some()
    }
}

#[async_trait]
impl<T: Clone + Send + Sync + 'static> Receivable<T> for Barrier<T> {
    async fn recv_msg(&mut self) -> Option<T> {
        self.wait().await.ok()
    }
}

/// The write side of a Barrier; calling `open` broadcasts a value to all waiters.
#[derive(Clone)]
pub struct BarrierOpener<T: Clone>(Arc<watch::Sender<Option<T>>>);

impl<T: Clone> BarrierOpener<T> {
    /// Open the barrier. Subsequent calls are no-ops.
    pub fn open(&self, value: T) {
        self.0.send_if_modified(|v| {
            if v.is_none() {
                *v = Some(value);
                true
            } else {
                false
            }
        });
    }
}

/// Create a new (Barrier, BarrierOpener) pair.
pub fn new_barrier<T: Clone>() -> (Barrier<T>, BarrierOpener<T>) {
    let (tx, rx) = watch::channel(None);
    (Barrier(rx), BarrierOpener(Arc::new(tx)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn barrier_open_after_spawn() {
        let (mut b, opener) = new_barrier::<u32>();
        let (tx, rx) = tokio::sync::oneshot::channel::<u32>();
        tokio::spawn(async move {
            tx.send(b.wait().await.unwrap()).unwrap();
        });
        opener.open(42);
        assert_eq!(rx.await.unwrap(), 42);
    }

    #[tokio::test]
    async fn barrier_open_before_spawn() {
        let (barrier, opener) = new_barrier::<u32>();
        opener.open(99);
        let mut b1 = barrier.clone();
        let mut b2 = barrier.clone();
        let (tx1, rx1) = tokio::sync::oneshot::channel::<u32>();
        let (tx2, rx2) = tokio::sync::oneshot::channel::<u32>();
        tokio::spawn(async move { tx1.send(b1.wait().await.unwrap()).unwrap() });
        tokio::spawn(async move { tx2.send(b2.wait().await.unwrap()).unwrap() });
        assert_eq!(rx1.await.unwrap(), 99);
        assert_eq!(rx2.await.unwrap(), 99);
    }

    #[tokio::test]
    async fn barrier_is_open() {
        let (b, opener) = new_barrier::<()>();
        assert!(!b.is_open());
        opener.open(());
        assert!(b.is_open());
    }
}
