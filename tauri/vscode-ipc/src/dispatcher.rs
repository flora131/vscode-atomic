//! RpcDispatcher — transport-agnostic JSON-RPC dispatcher.
//!
//! Port of cli/src/rpc.rs RpcBuilder / RpcMethodBuilder / RpcDispatcher / RpcCaller.

use std::{
    collections::HashMap,
    future,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc, Mutex,
    },
};

use futures::{future::BoxFuture, Future, FutureExt};
use serde::{de::DeserializeOwned, Serialize};
use tokio::sync::{mpsc, oneshot};

// ─────────────────────────────────────────────
// Serialization trait
// ─────────────────────────────────────────────

/// Transport-agnostic serialization strategy.
pub trait Serialization: Send + Sync + Clone + 'static {
    fn serialize(&self, value: &impl Serialize) -> Vec<u8>;
    fn deserialize<P: DeserializeOwned>(&self, b: &[u8]) -> anyhow::Result<P>;
}

// ─────────────────────────────────────────────
// Wire shapes
// ─────────────────────────────────────────────

#[derive(serde::Serialize)]
pub struct FullRequest<M, P> {
    pub id: Option<u32>,
    pub method: M,
    pub params: P,
}

#[derive(serde::Deserialize)]
pub struct RequestParams<P> {
    pub params: P,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct SuccessResponse<T> {
    pub id: u32,
    pub result: T,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct ErrorResponse {
    pub id: u32,
    pub error: ResponseError,
}

/// Error returned from an RPC call.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ResponseError {
    pub code: i32,
    pub message: String,
}

/// Approximate shape to classify incoming bytes.
#[derive(serde::Deserialize, Debug)]
pub struct PartialIncoming {
    pub id: Option<u32>,
    pub method: Option<String>,
    pub error: Option<ResponseError>,
}

// ─────────────────────────────────────────────
// MaybeSync — dispatch result
// ─────────────────────────────────────────────

/// Result of dispatching a single message: may be sync or async.
pub enum MaybeSync {
    /// Computed synchronously; bytes ready to send (or `None`).
    Sync(Option<Vec<u8>>),
    /// Needs to be driven by the caller.
    Future(BoxFuture<'static, Option<Vec<u8>>>),
}

// ─────────────────────────────────────────────
// Internal types (not exposed in public API)
// ─────────────────────────────────────────────

type SyncMethod = Arc<dyn Send + Sync + Fn(Option<u32>, &[u8]) -> Option<Vec<u8>>>;
type AsyncMethod =
    Arc<dyn Send + Sync + Fn(Option<u32>, &[u8]) -> BoxFuture<'static, Option<Vec<u8>>>>;

enum MethodKind {
    Sync(SyncMethod),
    Async(AsyncMethod),
}

enum Outcome {
    Success(Vec<u8>),
    Error(ResponseError),
}

type DispatchFn = Box<dyn Send + Sync + FnOnce(Outcome)>;

// ─────────────────────────────────────────────
// CallsMap — opaque shared pending-calls table
// ─────────────────────────────────────────────

/// Shared map of in-flight calls. Opaque to callers outside this module.
#[derive(Clone, Default)]
pub struct CallsMap(Arc<Mutex<HashMap<u32, DispatchFn>>>);

impl CallsMap {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&self, id: u32, f: DispatchFn) {
        self.0.lock().unwrap().insert(id, f);
    }

    fn remove(&self, id: u32) -> Option<DispatchFn> {
        self.0.lock().unwrap().remove(&id)
    }

    /// Drop all pending call slots (e.g. on shutdown).
    pub fn cancel_all(&self) {
        self.0.lock().unwrap().clear();
    }
}

// ─────────────────────────────────────────────
// Global message-id counter
// ─────────────────────────────────────────────

static MESSAGE_ID_COUNTER: AtomicU32 = AtomicU32::new(1);
fn next_message_id() -> u32 {
    MESSAGE_ID_COUNTER.fetch_add(1, Ordering::SeqCst)
}

// ─────────────────────────────────────────────
// RpcMethodBuilder
// ─────────────────────────────────────────────

/// Builds server-side method handlers; call `.build()` to get a `RpcDispatcher`.
pub struct RpcMethodBuilder<S: Serialization, C> {
    serializer: Arc<S>,
    context: Arc<C>,
    methods: HashMap<&'static str, MethodKind>,
    calls: CallsMap,
}

impl<S: Serialization, C: Send + Sync + 'static> RpcMethodBuilder<S, C> {
    pub fn new(serializer: S, context: C) -> Self {
        Self {
            serializer: Arc::new(serializer),
            context: Arc::new(context),
            methods: HashMap::new(),
            calls: CallsMap::new(),
        }
    }

    /// Register a synchronous handler.
    pub fn register_sync<P, R, F>(&mut self, method_name: &'static str, callback: F)
    where
        P: DeserializeOwned + 'static,
        R: Serialize + 'static,
        F: Fn(P, &C) -> anyhow::Result<R> + Send + Sync + 'static,
    {
        assert!(
            !self.methods.contains_key(method_name),
            "Method already registered: {method_name}"
        );
        let serial = self.serializer.clone();
        let ctx = self.context.clone();
        self.methods.insert(
            method_name,
            MethodKind::Sync(Arc::new(move |id, body| {
                let params: P = match serial.deserialize::<RequestParams<P>>(body) {
                    Ok(rp) => rp.params,
                    Err(err) => {
                        return id.map(|id| {
                            serial.serialize(&ErrorResponse {
                                id,
                                error: ResponseError {
                                    code: 0,
                                    message: format!("{err:?}"),
                                },
                            })
                        });
                    }
                };
                match callback(params, &ctx) {
                    Ok(result) => id.map(|id| serial.serialize(&SuccessResponse { id, result })),
                    Err(err) => id.map(|id| {
                        serial.serialize(&ErrorResponse {
                            id,
                            error: ResponseError {
                                code: -1,
                                message: format!("{err:?}"),
                            },
                        })
                    }),
                }
            })),
        );
    }

    /// Register an async handler.
    pub fn register_async<P, R, Fut, F>(&mut self, method_name: &'static str, callback: F)
    where
        P: DeserializeOwned + Send + 'static,
        R: Serialize + Send + Sync + 'static,
        Fut: Future<Output = anyhow::Result<R>> + Send + 'static,
        F: (Fn(P, Arc<C>) -> Fut) + Clone + Send + Sync + 'static,
    {
        let serial = self.serializer.clone();
        let ctx = self.context.clone();
        self.methods.insert(
            method_name,
            MethodKind::Async(Arc::new(move |id, body| {
                let params: P = match serial.deserialize::<RequestParams<P>>(body) {
                    Ok(rp) => rp.params,
                    Err(err) => {
                        return future::ready(id.map(|id| {
                            serial.serialize(&ErrorResponse {
                                id,
                                error: ResponseError {
                                    code: 0,
                                    message: format!("{err:?}"),
                                },
                            })
                        }))
                        .boxed();
                    }
                };

                let cb = callback.clone();
                let serial = serial.clone();
                let ctx = ctx.clone();
                async move {
                    match cb(params, ctx).await {
                        Ok(result) => {
                            id.map(|id| serial.serialize(&SuccessResponse { id, result }))
                        }
                        Err(err) => id.map(|id| {
                            serial.serialize(&ErrorResponse {
                                id,
                                error: ResponseError {
                                    code: -1,
                                    message: format!("{err:?}"),
                                },
                            })
                        }),
                    }
                }
                .boxed()
            })),
        );
    }

    /// Finalize and produce a `RpcDispatcher`.
    pub fn build(self) -> RpcDispatcher<S, C> {
        RpcDispatcher {
            context: self.context,
            serializer: self.serializer,
            methods: Arc::new(self.methods),
            calls: self.calls,
        }
    }
}

// ─────────────────────────────────────────────
// RpcDispatcher
// ─────────────────────────────────────────────

/// Server-side dispatcher. Parses raw bytes and routes to registered handlers.
#[derive(Clone)]
pub struct RpcDispatcher<S: Serialization, C> {
    context: Arc<C>,
    serializer: Arc<S>,
    methods: Arc<HashMap<&'static str, MethodKind>>,
    calls: CallsMap,
}

impl<S: Serialization, C: Send + Sync + 'static> RpcDispatcher<S, C> {
    /// Dispatch raw bytes; returns `MaybeSync` so the caller's loop can drive futures.
    pub fn dispatch(&self, body: &[u8]) -> MaybeSync {
        let partial: PartialIncoming = match self.serializer.deserialize(body) {
            Ok(p) => p,
            Err(_) => {
                tracing::warn!("Failed to deserialise RPC request");
                return MaybeSync::Sync(None);
            }
        };
        self.dispatch_partial(body, partial)
    }

    fn dispatch_partial(&self, body: &[u8], partial: PartialIncoming) -> MaybeSync {
        let id = partial.id;

        if let Some(method_name) = partial.method {
            match self.methods.get(method_name.as_str()) {
                Some(MethodKind::Sync(cb)) => MaybeSync::Sync(cb(id, body)),
                Some(MethodKind::Async(cb)) => MaybeSync::Future(cb(id, body)),
                None => MaybeSync::Sync(id.map(|id| {
                    self.serializer.serialize(&ErrorResponse {
                        id,
                        error: ResponseError {
                            code: -1,
                            message: format!("Method not found: {method_name}"),
                        },
                    })
                })),
            }
        } else if let Some(err) = partial.error {
            if let Some(cb) = id.and_then(|i| self.calls.remove(i)) {
                cb(Outcome::Error(err));
            }
            MaybeSync::Sync(None)
        } else {
            // Success response
            if let Some(cb) = id.and_then(|i| self.calls.remove(i)) {
                cb(Outcome::Success(body.to_vec()));
            }
            MaybeSync::Sync(None)
        }
    }

    pub fn context(&self) -> Arc<C> {
        self.context.clone()
    }
}

// ─────────────────────────────────────────────
// RpcCaller
// ─────────────────────────────────────────────

/// Client-side call issuer.
#[derive(Clone)]
pub struct RpcCaller<S: Serialization> {
    serializer: Arc<S>,
    calls: CallsMap,
    sender: mpsc::UnboundedSender<Vec<u8>>,
}

impl<S: Serialization> RpcCaller<S> {
    pub fn new(serializer: S, sender: mpsc::UnboundedSender<Vec<u8>>) -> Self {
        Self {
            serializer: Arc::new(serializer),
            calls: CallsMap::new(),
            sender,
        }
    }

    /// Return a shareable handle to the pending calls map.
    /// Pass it to `RpcResponseDispatcher::with_calls` to route responses back.
    pub fn calls_map(&self) -> CallsMap {
        self.calls.clone()
    }

    /// Send a fire-and-forget notification (no id, no reply).
    pub fn notify<M: AsRef<str> + Serialize, A: Serialize>(&self, method: M, params: A) -> bool {
        let body = self.serializer.serialize(&FullRequest {
            id: None::<u32>,
            method,
            params,
        });
        self.sender.send(body).is_ok()
    }

    /// Send a request and return a receiver for the typed response.
    pub fn call<M, A, R>(
        &self,
        method: M,
        params: A,
    ) -> oneshot::Receiver<Result<R, ResponseError>>
    where
        M: AsRef<str> + Serialize,
        A: Serialize,
        R: DeserializeOwned + Send + 'static,
    {
        let (tx, rx) = oneshot::channel();
        let id = next_message_id();
        let body = self.serializer.serialize(&FullRequest {
            id: Some(id),
            method,
            params,
        });

        if self.sender.send(body).is_err() {
            drop(tx);
            return rx;
        }

        let serial = self.serializer.clone();
        self.calls.insert(
            id,
            Box::new(move |outcome| {
                let _ = match outcome {
                    Outcome::Error(e) => tx.send(Err(e)),
                    Outcome::Success(raw) => match serial.deserialize::<SuccessResponse<R>>(&raw) {
                        Ok(sr) => tx.send(Ok(sr.result)),
                        Err(err) => tx.send(Err(ResponseError {
                            code: 0,
                            message: err.to_string(),
                        })),
                    },
                };
            }),
        );

        rx
    }
}

// ─────────────────────────────────────────────
// RpcResponseDispatcher — client-side reply router
// ─────────────────────────────────────────────

/// Routes server responses back to pending `RpcCaller` call slots.
#[derive(Clone)]
pub struct RpcResponseDispatcher<S: Serialization> {
    serializer: Arc<S>,
    calls: CallsMap,
}

impl<S: Serialization> RpcResponseDispatcher<S> {
    /// Create a new response dispatcher backed by the provided `CallsMap`
    /// (obtained from `RpcCaller::calls_map()`).
    pub fn with_calls(serializer: S, calls: CallsMap) -> Self {
        Self {
            serializer: Arc::new(serializer),
            calls,
        }
    }

    /// Feed a raw JSON line into the response router.
    pub fn dispatch(&self, body: &[u8]) {
        let partial: PartialIncoming = match self.serializer.deserialize(body) {
            Ok(p) => p,
            Err(_) => return,
        };

        if let Some(id) = partial.id {
            if let Some(cb) = self.calls.remove(id) {
                if let Some(err) = partial.error {
                    cb(Outcome::Error(err));
                } else {
                    cb(Outcome::Success(body.to_vec()));
                }
            }
        }
    }

    /// Drop all pending call slots (e.g. on shutdown).
    pub fn cancel_all(&self) {
        self.calls.cancel_all();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::framing::JsonRpcSerializer;

    fn echo_dispatcher() -> RpcDispatcher<JsonRpcSerializer, ()> {
        let mut builder = RpcMethodBuilder::new(JsonRpcSerializer::new(), ());
        builder.register_async("echo", |p: String, _ctx: Arc<()>| async move {
            Ok::<String, anyhow::Error>(p)
        });
        builder.build()
    }

    fn add_dispatcher() -> RpcDispatcher<JsonRpcSerializer, ()> {
        let mut builder = RpcMethodBuilder::new(JsonRpcSerializer::new(), ());
        builder.register_sync("add", |p: (i64, i64), _ctx: &()| {
            Ok::<i64, anyhow::Error>(p.0 + p.1)
        });
        builder.build()
    }

    #[tokio::test]
    async fn sync_dispatch_add() {
        let d = add_dispatcher();
        let serial = JsonRpcSerializer::new();
        let body = serial.serialize(&FullRequest {
            id: Some(1u32),
            method: "add",
            params: (3i64, 4i64),
        });
        let result = d.dispatch(&body);
        match result {
            MaybeSync::Sync(Some(bytes)) => {
                let sr: SuccessResponse<i64> = serial.deserialize(&bytes).unwrap();
                assert_eq!(sr.result, 7);
            }
            _ => panic!("expected Sync response"),
        }
    }

    #[tokio::test]
    async fn async_dispatch_echo() {
        let d = echo_dispatcher();
        let serial = JsonRpcSerializer::new();
        let body = serial.serialize(&FullRequest {
            id: Some(1u32),
            method: "echo",
            params: "hello",
        });
        let result = d.dispatch(&body);
        match result {
            MaybeSync::Future(fut) => {
                let bytes = fut.await.unwrap();
                let sr: SuccessResponse<String> = serial.deserialize(&bytes).unwrap();
                assert_eq!(sr.result, "hello");
            }
            _ => panic!("expected Future response"),
        }
    }

    #[test]
    fn unknown_method_returns_error() {
        let d = add_dispatcher();
        let serial = JsonRpcSerializer::new();
        let body = serial.serialize(&FullRequest {
            id: Some(99u32),
            method: "nope",
            params: (),
        });
        match d.dispatch(&body) {
            MaybeSync::Sync(Some(bytes)) => {
                let er: ErrorResponse = serial.deserialize(&bytes).unwrap();
                assert!(er.error.message.contains("Method not found"));
            }
            _ => panic!("expected Sync error"),
        }
    }
}
