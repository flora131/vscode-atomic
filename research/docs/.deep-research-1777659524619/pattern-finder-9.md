# VS Code Rust Port: Core Async & RPC Patterns from `cli/`

## Research Finding: Partition 9 Pattern Catalog

This document catalogs concrete async, struct, and concurrent patterns from the existing Rust `cli/` codebase that would form the architectural foundation for a TypeScript/Electron to Tauri/Rust port of VS Code's core IDE functionality.

---

## Patterns Found

### Pattern 1: Async Task Spawning with Error Handling
**Where:** `cli/src/rpc.rs:483` and `cli/src/json_rpc.rs:85-96`
**What:** Fire-and-forget async tasks via `tokio::spawn` with message channel integration for result delivery.

```rust
// From json_rpc.rs - RPC response dispatch with spawned futures
MaybeSync::Future(fut) => {
    let write_tx = write_tx.clone();
    tokio::spawn(async move {
        if let Some(v) = fut.await {
            let _ = write_tx.send(v).await;
        }
    });
},
MaybeSync::Stream((dto, fut)) => {
    if let Some(dto) = dto {
        dispatcher.register_stream(write_tx.clone(), dto).await;
    }
    let write_tx = write_tx.clone();
    tokio::spawn(async move {
        if let Some(v) = fut.await {
            let _ = write_tx.send(v).await;
        }
    });
}
```

**Variations / call-sites:**
- `cli/src/rpc.rs:483` - Stream data pump spawning
- `cli/src/msgpack_rpc.rs:79-95` - Identical pattern with msgpack codec
- `cli/src/json_rpc.rs:85-96` - JSON RPC variant
- `cli/src/commands/serve_web.rs:124,171,322,351,378,629,761,776` - Web server connection handlers
- `cli/src/tunnels/control_server.rs:208-310` - Control protocol connections
- `cli/src/tunnels/agent_host.rs:141,273,522,542` - Agent host lifecycle management

### Pattern 2: RPC Builder with Generic Serialization Trait
**Where:** `cli/src/rpc.rs:49-91` and `cli/src/rpc.rs:310-382`
**What:** Transport-agnostic RPC dispatcher built from a `RpcBuilder` that registers sync/async/duplex method callbacks. Uses trait-based serialization allowing JSON or msgpack backends.

```rust
pub struct RpcBuilder<S> {
    serializer: Arc<S>,
    methods: HashMap<&'static str, Method>,
    calls: Arc<Mutex<HashMap<u32, DispatchMethod>>>,
}

pub struct RpcCaller<S: Serialization> {
    serializer: Arc<S>,
    calls: Arc<Mutex<HashMap<u32, DispatchMethod>>>,
    sender: mpsc::UnboundedSender<Vec<u8>>,
}

pub struct RpcDispatcher<S, C> {
    log: log::Logger,
    context: Arc<C>,
    serializer: Arc<S>,
    methods: Arc<HashMap<&'static str, Method>>,
    calls: Arc<Mutex<HashMap<u32, DispatchMethod>>>,
    streams: Streams,
}
```

**Variations / call-sites:**
- `cli/src/json_rpc.rs:21-42` - JSON serialization implementation
- `cli/src/msgpack_rpc.rs:24-41` - Msgpack serialization (preferred for binary efficiency)
- `cli/src/tunnels/singleton_client.rs:62` - Uses `new_json_rpc()` builder
- `cli/src/commands/agent_host.rs` - Multiple RPC dispatcher instantiations

### Pattern 3: Duplex Streaming with Tokio I/O Split
**Where:** `cli/src/rpc.rs:203-270` (register_duplex) and `cli/src/tunnels/server_bridge.rs:21-62`
**What:** Bidirectional duplex streams created via `tokio::io::duplex()` for request-response pairs. Streams are spawned as independent tasks reading and forwarding data.

```rust
pub async fn new(
    path: &Path,
    mut target: ServerMessageSink,
    decoder: ClientMessageDecoder,
) -> Result<Self, AnyError> {
    let stream = get_socket_rw_stream(path).await?;
    let (mut read, write) = socket_stream_split(stream);

    tokio::spawn(async move {
        let mut read_buf = vec![0; BUFFER_SIZE];
        loop {
            match read.read(&mut read_buf).await {
                Err(_) => return,
                Ok(0) => {
                    let _ = target.server_closed().await;
                    return;
                }
                Ok(s) => {
                    let send = target.server_message(&read_buf[..s]).await;
                    if send.is_err() {
                        return;
                    }
                }
            }
        }
    });

    Ok(ServerBridge { write, decoder })
}
```

**Variations / call-sites:**
- `cli/src/rpc.rs:248-252` - Stream pair creation with capacity-hint duplex
- `cli/src/async_pipe.rs:42-44` - Unix socket stream splitting
- `cli/src/tunnels/server_bridge.rs:29-46` - Background pump spawning

### Pattern 4: Barrier/Synchronization Primitive for Async Startup
**Where:** `cli/src/util/sync.rs:12-68` and `cli/src/tunnels/agent_host.rs:95-126`
**What:** Custom barrier using `tokio::sync::watch` channels for one-time async signaling with optional value passing. Used for coordinating server startup readiness.

```rust
#[derive(Clone)]
pub struct Barrier<T>(watch::Receiver<Option<T>>)
where
    T: Clone;

impl<T> Barrier<T>
where
    T: Clone,
{
    pub async fn wait(&mut self) -> Result<T, RecvError> {
        loop {
            self.0.changed().await?;
            if let Some(v) = self.0.borrow().clone() {
                return Ok(v);
            }
        }
    }

    pub fn is_open(&self) -> bool {
        self.0.borrow().is_some()
    }
}

pub fn new_barrier<T>() -> (Barrier<T>, BarrierOpener<T>)
where
    T: Clone,
{
    let (closed_tx, closed_rx) = watch::channel(None);
    (Barrier(closed_rx), BarrierOpener(Arc::new(closed_tx)))
}
```

**Variations / call-sites:**
- `cli/src/util/sync.rs:36-89` - Receivable trait implementations for broadcast/mpsc
- `cli/src/util/sync.rs:103-180` - Merged/concatenated receivables
- `cli/src/tunnels/agent_host.rs:70-71` - Ready barrier for server startup
- `cli/src/util/sync.rs:186-218` - Test examples with tokio::spawn

### Pattern 5: Async Process Management with Tokio CLI Commands
**Where:** `cli/src/util/command.rs:13-71` and `cli/src/util/command.rs:124-168`
**What:** Async command execution using `tokio::process::Command` with output capture and tree-kill for process cleanup. Non-blocking with futures composition.

```rust
pub async fn capture_command<A, I, S>(
    command_str: A,
    args: I,
) -> Result<std::process::Output, CodeError>
where
    A: AsRef<OsStr>,
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    new_tokio_command(&command_str)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .output()
        .await
        .map_err(|e| CodeError::CommandFailed {
            command: command_str.as_ref().to_string_lossy().to_string(),
            code: -1,
            output: e.to_string(),
        })
}

pub async fn kill_tree(process_id: u32) -> Result<(), CodeError> {
    use futures::future::join_all;
    let mut prgrep_cmd = Command::new("pgrep")
        .arg("-P")
        .arg(&parent_id)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .spawn()?;

    let mut kill_futures = vec![tokio::spawn(
        async move { kill_single_pid(parent_id).await }
    )];

    if let Some(stdout) = prgrep_cmd.stdout.take() {
        let mut reader = BufReader::new(stdout).lines();
        while let Some(line) = reader.next_line().await.unwrap_or(None) {
            kill_futures.push(tokio::spawn(async move { kill_single_pid(line).await }))
        }
    }

    join_all(kill_futures).await;
    prgrep_cmd.kill().await.ok();
    Ok(())
}
```

**Variations / call-sites:**
- `cli/src/util/command.rs:75-85` - Platform-specific command wrappers (Windows CREATE_NO_WINDOW)
- `cli/src/util/command.rs:13-29` - Checked output variant
- `cli/src/tunnels/agent_host.rs:171` - Script command spawning for server startup

### Pattern 6: Agent Host Lifecycle with Tokio Select and State Mutation
**Where:** `cli/src/tunnels/agent_host.rs:59-92` and `cli/src/tunnels/agent_host.rs:155-295`
**What:** Long-running state machine using `tokio::select!` for multi-path async coordination. Manages process lifecycle with backpressure (Mutex<Option<T>> pattern) and barrier synchronization for readiness.

```rust
pub struct AgentHostManager {
    log: log::Logger,
    config: AgentHostConfig,
    platform: Platform,
    cache: DownloadCache,
    update_service: UpdateService,
    latest_release: Mutex<Option<(Instant, Release)>>,
    running: Mutex<Option<RunningServer>>,
    ready: Mutex<Option<Barrier<Result<PathBuf, String>>>>,
}

pub async fn ensure_server(self: &Arc<Self>) -> Result<PathBuf, CodeError> {
    let ready = self.ready.lock().await;
    if let Some(barrier) = &*ready {
        if barrier.is_open() {
            let running = self.running.lock().await;
            if running.is_some() {
                return barrier
                    .clone()
                    .wait()
                    .await
                    .unwrap()
                    .map_err(CodeError::ServerDownloadError);
            }
        }
    }
    self.start_server().await
}

async fn run_server(
    self: &Arc<Self>,
    release: Release,
    server_dir: PathBuf,
    opener: BarrierOpener<Result<PathBuf, String>>,
) {
    let mut child = match cmd.spawn() { ... };
    let (mut stdout, mut stderr) = (...);
    
    let mut opener = Some(opener);
    let startup_deadline = tokio::time::sleep(STARTUP_TIMEOUT);
    tokio::pin!(startup_deadline);

    let mut ready = false;
    loop {
        tokio::select! {
            Ok(Some(l)) = stdout.next_line() => {
                if !ready && l.contains("Agent host server listening on") {
                    ready = true;
                    if let Some(o) = opener.take() {
                        o.open(Ok(socket_path.clone()));
                    }
                }
            }
            _ = &mut startup_deadline, if !ready => {
                if let Some(o) = opener.take() {
                    o.open(Ok(socket_path.clone()));
                }
                ready = true;
            }
            e = child.wait() => {
                if let Some(o) = opener.take() {
                    o.open(Err(format!("Server exited before ready: {e:?}")));
                }
                return;
            }
        }
    }
}
```

**Variations / call-sites:**
- `cli/src/tunnels/agent_host.rs:273` - Background log pump spawning after startup
- `cli/src/tunnels/agent_host.rs:421` - Update loop with periodic interval ticking
- `cli/src/tunnels/control_server.rs:245-355` - Main event loop with port/socket select arms

### Pattern 7: Select-Based Message Multiplexing with Type Variants
**Where:** `cli/src/json_rpc.rs:46-106` and `cli/src/msgpack_rpc.rs:46-110`
**What:** Multi-source async message handling using `tokio::select!` macro. Coordinates shutdown signals, outbound message channels, inbound stream data, and client reads into a unified event loop.

```rust
pub async fn start_json_rpc<C: Send + Sync + 'static, S: Clone>(
    dispatcher: rpc::RpcDispatcher<JsonRpcSerializer, C>,
    read: impl AsyncRead + Unpin,
    mut write: impl AsyncWrite + Unpin,
    mut msg_rx: impl Receivable<Vec<u8>>,
    mut shutdown_rx: Barrier<S>,
) -> io::Result<Option<S>> {
    let (write_tx, mut write_rx) = mpsc::channel::<Vec<u8>>(8);
    let mut read = BufReader::new(read);
    let mut read_buf = String::new();
    let shutdown_fut = shutdown_rx.wait();
    pin!(shutdown_fut);

    loop {
        tokio::select! {
            r = &mut shutdown_fut => return Ok(r.ok()),
            Some(w) = write_rx.recv() => {
                write.write_all(&w).await?;
            },
            Some(w) = msg_rx.recv_msg() => {
                write.write_all(&w).await?;
            },
            n = read.read_line(&mut read_buf) => {
                let r = match n {
                    Ok(0) => return Ok(None),
                    Ok(n) => dispatcher.dispatch(&read_buf.as_bytes()[..n]),
                    Err(e) => return Err(e)
                };
                read_buf.truncate(0);
                // ... spawn async futures for MaybeSync variants
            }
        }
    }
}
```

**Variations / call-sites:**
- `cli/src/msgpack_rpc.rs:66-110` - Binary frame-based variant with decoder state
- `cli/src/tunnels/control_server.rs:245-355` - More complex variant with 5+ select arms
- `cli/src/util/sync.rs:157-179` - Receivable trait with select! inside trait methods

---

## Summary

The `cli/` codebase demonstrates mature Rust async patterns critical for an IDE port:

1. **Async spawning** is uniform: `tokio::spawn(async move { ... })` for detached background tasks with message channels for result delivery.

2. **RPC dispatch** is serialization-agnostic via trait bounds (`Serialization`), allowing JSON/msgpack interchange without logic duplication.

3. **Duplex I/O** follows the tokio pattern: `split()`, independent read/write pumps in spawned tasks, with optional decoders.

4. **Barriers** replace traditional channels for one-time startup synchronization with optional value passing.

5. **Process lifecycle** leverages `tokio::process::Command` and `join_all()` for non-blocking execution and multi-process teardown.

6. **State machines** use `tokio::select!` as the core control structure, managing multiple async sources (timers, I/O, signals, channels) without callback pyramids.

7. **Message multiplexing** abstracts over transport (Unix sockets, named pipes, TCP) via generic traits, enabling reuse across agent host, control server, and web endpoints.

These patterns would form the architectural substrate for porting IDE subsystems: language server RPC, debug protocol tunneling, terminal I/O, file watching, and source control integration.

---

**Files in scope:** 75 files, 20,107 LOC across `cli/src/**/*.rs`
