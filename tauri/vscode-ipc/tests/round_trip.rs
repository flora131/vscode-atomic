//! Integration tests for vscode-ipc RPC framework.
//!
//! Tests: round-trip dispatch, cancellation, shutdown via Barrier.

use std::sync::Arc;

use tokio::{
    sync::oneshot,
    time::{timeout, Duration},
};
use vscode_ipc::{
    dispatcher::{RpcDispatcher, RpcMethodBuilder, RpcCaller, RpcResponseDispatcher},
    framing::{start_json_rpc, JsonRpcSerializer},
    sync::new_barrier,
};

// ──────────────────────────────────────────────────────────────────────────────
// Helper
// ──────────────────────────────────────────────────────────────────────────────

fn make_dispatcher() -> RpcDispatcher<JsonRpcSerializer, ()> {
    let mut builder: RpcMethodBuilder<JsonRpcSerializer, ()> =
        RpcMethodBuilder::new(JsonRpcSerializer::new(), ());

    builder.register_async("echo", |params: String, _ctx: Arc<()>| async move {
        Ok::<String, anyhow::Error>(params)
    });

    builder.register_sync("add", |params: (i64, i64), _ctx: &()| {
        Ok::<i64, anyhow::Error>(params.0 + params.1)
    });

    builder.build()
}

/// Spawn a server loop and return a ready-to-use RpcCaller + response routing task.
async fn spawn_server_and_client(
    dispatcher: RpcDispatcher<JsonRpcSerializer, ()>,
) -> RpcCaller<JsonRpcSerializer> {
    let (server_stream, client_stream) = tokio::io::duplex(65536);
    let (server_read, server_write) = tokio::io::split(server_stream);
    let (client_read, client_write) = tokio::io::split(client_stream);

    let (shutdown_rx, shutdown_tx) = new_barrier::<()>();

    // Server — move shutdown_tx into the task so it outlives this function.
    tokio::spawn(async move {
        let _keep_alive = shutdown_tx; // drop only when server task exits
        let _ = start_json_rpc(dispatcher, server_read, server_write, shutdown_rx).await;
    });

    let serializer = JsonRpcSerializer::new();
    let (write_tx, write_rx) = tokio::sync::mpsc::unbounded_channel::<Vec<u8>>();
    let caller = RpcCaller::new(serializer.clone(), write_tx);

    // Forward outbound client bytes
    let mut cw = client_write;
    let mut write_rx_consumer = write_rx;
    tokio::spawn(async move {
        use tokio::io::AsyncWriteExt;
        while let Some(msg) = write_rx_consumer.recv().await {
            if cw.write_all(&msg).await.is_err() {
                break;
            }
        }
    });

    // Route inbound responses
    let resp_dispatcher =
        RpcResponseDispatcher::with_calls(serializer.clone(), caller.calls_map());
    tokio::spawn(async move {
        use tokio::io::AsyncBufReadExt;
        let mut buf_reader = tokio::io::BufReader::new(client_read);
        let mut line = String::new();
        while buf_reader.read_line(&mut line).await.unwrap_or(0) > 0 {
            resp_dispatcher.dispatch(line.as_bytes());
            line.clear();
        }
    });

    caller
}

// ──────────────────────────────────────────────────────────────────────────────
// Test 1: async echo round-trip
// ──────────────────────────────────────────────────────────────────────────────
#[tokio::test]
async fn test_async_echo_round_trip() {
    let caller = spawn_server_and_client(make_dispatcher()).await;
    let rx = caller.call::<_, _, String>("echo", "hello world".to_string());
    let result = timeout(Duration::from_secs(5), rx)
        .await
        .expect("timeout")
        .expect("channel closed")
        .expect("rpc error");
    assert_eq!(result, "hello world");
}

// ──────────────────────────────────────────────────────────────────────────────
// Test 2: sync add round-trip
// ──────────────────────────────────────────────────────────────────────────────
#[tokio::test]
async fn test_sync_add_round_trip() {
    let caller = spawn_server_and_client(make_dispatcher()).await;
    let rx = caller.call::<_, _, i64>("add", (3i64, 4i64));
    let result = timeout(Duration::from_secs(5), rx)
        .await
        .expect("timeout")
        .expect("channel closed")
        .expect("rpc error");
    assert_eq!(result, 7);
}

// ──────────────────────────────────────────────────────────────────────────────
// Test 3: cancellation via Barrier shutdown cancels in-flight call
// ──────────────────────────────────────────────────────────────────────────────
#[tokio::test]
async fn test_shutdown_cancels_in_flight_call() {
    let serializer = JsonRpcSerializer::new();
    let mut builder: RpcMethodBuilder<JsonRpcSerializer, ()> =
        RpcMethodBuilder::new(serializer.clone(), ());

    // "slow" handler: signal when entered, then sleep forever
    let (handler_entered_tx, handler_entered_rx) = oneshot::channel::<()>();
    let handler_entered_tx = Arc::new(std::sync::Mutex::new(Some(handler_entered_tx)));

    builder.register_async("slow", move |_params: (), _ctx: Arc<()>| {
        let sender = handler_entered_tx.clone();
        async move {
            if let Some(tx) = sender.lock().unwrap().take() {
                let _ = tx.send(());
            }
            tokio::time::sleep(Duration::from_secs(60)).await;
            Ok::<String, anyhow::Error>("done".to_string())
        }
    });

    let dispatcher = builder.build();

    let (server_stream, client_stream) = tokio::io::duplex(65536);
    let (server_read, server_write) = tokio::io::split(server_stream);
    let (client_read, client_write) = tokio::io::split(client_stream);

    let (shutdown_rx, shutdown_tx) = new_barrier::<()>();

    tokio::spawn(async move {
        let _ = start_json_rpc(dispatcher, server_read, server_write, shutdown_rx).await;
    });

    let (write_tx, write_rx) = tokio::sync::mpsc::unbounded_channel::<Vec<u8>>();
    let caller = RpcCaller::new(serializer.clone(), write_tx);

    let mut cw = client_write;
    let mut write_rx_consumer = write_rx;
    tokio::spawn(async move {
        use tokio::io::AsyncWriteExt;
        while let Some(msg) = write_rx_consumer.recv().await {
            if cw.write_all(&msg).await.is_err() {
                break;
            }
        }
    });

    let rx = caller.call::<_, _, String>("slow", ());

    let resp_dispatcher =
        RpcResponseDispatcher::with_calls(serializer.clone(), caller.calls_map());
    tokio::spawn(async move {
        use tokio::io::AsyncBufReadExt;
        let mut buf_reader = tokio::io::BufReader::new(client_read);
        let mut line = String::new();
        while buf_reader.read_line(&mut line).await.unwrap_or(0) > 0 {
            resp_dispatcher.dispatch(line.as_bytes());
            line.clear();
        }
        // EOF: server closed the connection. Cancel all pending calls so their
        // receivers are resolved (with channel-closed error) instead of hanging.
        resp_dispatcher.cancel_all();
    });

    // Wait until the slow handler is entered, then signal shutdown
    let _ = handler_entered_rx.await;
    shutdown_tx.open(());

    // The call rx should resolve within the timeout (closed or error, not stuck forever)
    let res = timeout(Duration::from_secs(5), rx).await;
    assert!(
        res.is_ok(),
        "shutdown did not unblock in-flight call within 5 seconds"
    );
}

// ──────────────────────────────────────────────────────────────────────────────
// Test 4: shutdown via Barrier — fast call completes, then server exits cleanly
// ──────────────────────────────────────────────────────────────────────────────
#[tokio::test]
async fn test_shutdown_via_barrier() {
    let dispatcher = make_dispatcher();

    let (server_stream, client_stream) = tokio::io::duplex(65536);
    let (server_read, server_write) = tokio::io::split(server_stream);
    let (client_read, client_write) = tokio::io::split(client_stream);

    let (shutdown_rx, shutdown_tx) = new_barrier::<()>();

    let server_handle = tokio::spawn(async move {
        start_json_rpc(dispatcher, server_read, server_write, shutdown_rx).await
    });

    let serializer = JsonRpcSerializer::new();
    let (write_tx, write_rx) = tokio::sync::mpsc::unbounded_channel::<Vec<u8>>();
    let caller = RpcCaller::new(serializer.clone(), write_tx);

    let mut cw = client_write;
    let mut write_rx_consumer = write_rx;
    tokio::spawn(async move {
        use tokio::io::AsyncWriteExt;
        while let Some(msg) = write_rx_consumer.recv().await {
            if cw.write_all(&msg).await.is_err() {
                break;
            }
        }
    });

    let rx = caller.call::<_, _, String>("echo", "barrier test".to_string());

    let resp_dispatcher =
        RpcResponseDispatcher::with_calls(serializer.clone(), caller.calls_map());
    tokio::spawn(async move {
        use tokio::io::AsyncBufReadExt;
        let mut buf_reader = tokio::io::BufReader::new(client_read);
        let mut line = String::new();
        while buf_reader.read_line(&mut line).await.unwrap_or(0) > 0 {
            resp_dispatcher.dispatch(line.as_bytes());
            line.clear();
        }
    });

    // Fast call should complete before shutdown
    let result = timeout(Duration::from_secs(5), rx)
        .await
        .expect("timeout")
        .expect("channel closed")
        .expect("rpc error");
    assert_eq!(result, "barrier test");

    // Trigger shutdown
    shutdown_tx.open(());

    // Server task exits cleanly
    let server_result = timeout(Duration::from_secs(5), server_handle)
        .await
        .expect("server did not shut down in time")
        .expect("server panicked");

    assert!(
        server_result.is_ok(),
        "server exited with error: {server_result:?}"
    );
}
