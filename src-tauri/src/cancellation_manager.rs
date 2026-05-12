use std::{
    collections::{HashMap, HashSet},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
};

use crate::contracts::{CancelRequest, CancellationId, RequestId};

#[derive(Default)]
pub struct CancellationManager {
    inner: Mutex<CancellationState>,
}

impl CancellationManager {
    pub fn begin(
        &self,
        request_id: RequestId,
        cancellation_id: Option<CancellationId>,
    ) -> RequestCancellationGuard<'_> {
        let cancelled = Arc::new(AtomicBool::new(false));

        if let Some(cancellation_id) = cancellation_id.clone() {
            let mut state = self.inner.lock().expect("cancellation state lock poisoned");
            if state.cancelled.remove(&cancellation_id) {
                cancelled.store(true, Ordering::SeqCst);
            }
            state.active.insert(
                cancellation_id.clone(),
                ActiveCancellation {
                    request_id: request_id.clone(),
                    cancelled: cancelled.clone(),
                },
            );
        }

        RequestCancellationGuard {
            manager: self,
            cancellation_id,
            cancelled,
        }
    }

    pub fn cancel(&self, request: &CancelRequest) -> bool {
        let mut state = self.inner.lock().expect("cancellation state lock poisoned");

        if let Some(active) = state.active.get(&request.cancellation_id) {
            let request_id_matches = match request.request_id.as_ref() {
                Some(request_id) => request_id == &active.request_id,
                None => true,
            };

            if request_id_matches {
                active.cancelled.store(true, Ordering::SeqCst);
                return true;
            }

            return false;
        }

        state.cancelled.insert(request.cancellation_id.clone());
        true
    }

    fn finish(&self, cancellation_id: &CancellationId) {
        let mut state = self.inner.lock().expect("cancellation state lock poisoned");
        state.active.remove(cancellation_id);
        state.cancelled.remove(cancellation_id);
    }
}

pub struct RequestCancellationGuard<'a> {
    manager: &'a CancellationManager,
    cancellation_id: Option<CancellationId>,
    cancelled: Arc<AtomicBool>,
}

impl RequestCancellationGuard<'_> {
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::SeqCst)
    }
}

impl Drop for RequestCancellationGuard<'_> {
    fn drop(&mut self) {
        if let Some(cancellation_id) = &self.cancellation_id {
            self.manager.finish(cancellation_id);
        }
    }
}

#[derive(Default)]
struct CancellationState {
    active: HashMap<CancellationId, ActiveCancellation>,
    cancelled: HashSet<CancellationId>,
}

struct ActiveCancellation {
    request_id: RequestId,
    cancelled: Arc<AtomicBool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cancel_marks_matching_active_request() {
        let manager = CancellationManager::default();
        let guard = manager.begin(
            "req-1".to_string(),
            Some(CancellationId("cancel-1".to_string())),
        );

        assert!(manager.cancel(&CancelRequest {
            cancellation_id: CancellationId("cancel-1".to_string()),
            request_id: Some("req-1".to_string()),
        }));
        assert!(guard.is_cancelled());
    }

    #[test]
    fn cancel_ignores_mismatched_request_id() {
        let manager = CancellationManager::default();
        let guard = manager.begin(
            "req-1".to_string(),
            Some(CancellationId("cancel-1".to_string())),
        );

        assert!(!manager.cancel(&CancelRequest {
            cancellation_id: CancellationId("cancel-1".to_string()),
            request_id: Some("req-2".to_string()),
        }));
        assert!(!guard.is_cancelled());
    }
}
