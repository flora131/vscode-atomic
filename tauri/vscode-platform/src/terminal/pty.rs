//! PTY backend — portable-pty-backed local implementation.
//!
//! Mirrors the surface of src/vs/workbench/contrib/terminal/electron-browser/localPty.ts:
//! spawn shell with cwd/env, resize, write input, read output stream, exit code.

use std::{
    collections::HashMap,
    io::{Read, Write},
    path::PathBuf,
    sync::Arc,
};

use anyhow::Result;
use async_trait::async_trait;
use portable_pty::{native_pty_system, CommandBuilder, PtySize};
use tokio::sync::mpsc;

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// Options passed to [`spawn`].
#[derive(Debug, Clone)]
pub struct PtySpawnOptions {
    pub shell: PathBuf,
    pub args: Vec<String>,
    pub cwd: Option<PathBuf>,
    pub env: HashMap<String, String>,
    pub cols: u16,
    pub rows: u16,
}

/// Events emitted on the output channel.
#[derive(Debug)]
pub enum PtyOutputEvent {
    Data(Vec<u8>),
    Exit(ExitCode),
}

/// Exit code of the child process.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExitCode(pub u32);

// ---------------------------------------------------------------------------
// IPty trait
// ---------------------------------------------------------------------------

#[async_trait]
pub trait IPty: Send + Sync {
    /// Write raw bytes to the PTY master (stdin of the child).
    fn write(&self, data: &[u8]) -> Result<()>;

    /// Resize the PTY.
    fn resize(&self, cols: u16, rows: u16) -> Result<()>;

    /// Send SIGKILL (Unix) / TerminateProcess (Windows) to the child.
    fn kill(&self) -> Result<()>;

    /// Block (async) until the child exits and return its exit code.
    async fn wait_for_exit(&self) -> Result<ExitCode>;
}

// ---------------------------------------------------------------------------
// LocalPty
// ---------------------------------------------------------------------------

/// PTY handle backed by [`portable_pty`].
pub struct LocalPty {
    master: std::sync::Mutex<Box<dyn portable_pty::MasterPty + Send>>,
    child: std::sync::Mutex<Box<dyn portable_pty::Child + Send + Sync>>,
    writer: std::sync::Mutex<Box<dyn Write + Send>>,
}

impl LocalPty {
    fn new(
        master: Box<dyn portable_pty::MasterPty + Send>,
        child: Box<dyn portable_pty::Child + Send + Sync>,
        writer: Box<dyn Write + Send>,
    ) -> Self {
        Self {
            master: std::sync::Mutex::new(master),
            child: std::sync::Mutex::new(child),
            writer: std::sync::Mutex::new(writer),
        }
    }
}

#[async_trait]
impl IPty for LocalPty {
    fn write(&self, data: &[u8]) -> Result<()> {
        let mut w = self.writer.lock().unwrap();
        w.write_all(data)?;
        Ok(())
    }

    fn resize(&self, cols: u16, rows: u16) -> Result<()> {
        let master = self.master.lock().unwrap();
        master.resize(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        })?;
        Ok(())
    }

    fn kill(&self) -> Result<()> {
        let mut child = self.child.lock().unwrap();
        child.kill()?;
        Ok(())
    }

    async fn wait_for_exit(&self) -> Result<ExitCode> {
        // `wait` is blocking; run on a blocking thread so we don't stall the async runtime.
        let status = tokio::task::spawn_blocking({
            // We need an owned handle.  portable-pty's Child::wait takes &mut self, so we
            // hold the mutex for the duration of the blocking call.
            let child_ptr = &self.child as *const std::sync::Mutex<Box<dyn portable_pty::Child + Send + Sync>>;
            // SAFETY: LocalPty is kept alive for the duration of this future (Arc contract).
            let child_ptr = child_ptr as usize;
            move || {
                let mutex = unsafe { &*(child_ptr as *const std::sync::Mutex<Box<dyn portable_pty::Child + Send + Sync>>) };
                let mut child = mutex.lock().unwrap();
                child.wait()
            }
        })
        .await??;

        Ok(ExitCode(status.exit_code()))
    }
}

// ---------------------------------------------------------------------------
// spawn
// ---------------------------------------------------------------------------

/// Spawn a PTY process.
///
/// Returns `(Arc<dyn IPty>, mpsc::Receiver<PtyOutputEvent>)`.
/// The receiver yields `PtyOutputEvent::Data` chunks followed by a single
/// `PtyOutputEvent::Exit` when the child exits.
pub fn spawn(opts: PtySpawnOptions) -> Result<(Arc<dyn IPty>, mpsc::Receiver<PtyOutputEvent>)> {
    let pty_system = native_pty_system();
    let pair = pty_system.openpty(PtySize {
        rows: opts.rows,
        cols: opts.cols,
        pixel_width: 0,
        pixel_height: 0,
    })?;

    let mut cmd = CommandBuilder::new(&opts.shell);
    for arg in &opts.args {
        cmd.arg(arg);
    }
    if let Some(ref cwd) = opts.cwd {
        cmd.cwd(cwd);
    }
    for (k, v) in &opts.env {
        cmd.env(k, v);
    }

    let child = pair.slave.spawn_command(cmd)?;
    let writer = pair.master.take_writer()?;
    let mut reader = pair.master.try_clone_reader()?;

    let (tx, rx) = mpsc::channel::<PtyOutputEvent>(256);

    // Spawn a dedicated thread to drain the PTY reader and forward to the channel.
    let tx_clone = tx.clone();
    std::thread::spawn(move || {
        let mut buf = [0u8; 4096];
        loop {
            match reader.read(&mut buf) {
                Ok(0) | Err(_) => break,
                Ok(n) => {
                    let data = buf[..n].to_vec();
                    if tx_clone.blocking_send(PtyOutputEvent::Data(data)).is_err() {
                        break;
                    }
                }
            }
        }
    });

    let pty = Arc::new(LocalPty::new(pair.master, child, writer));

    // Spawn a task to wait for exit and send the Exit event.
    {
        let pty_clone = Arc::clone(&pty) as Arc<dyn IPty>;
        let tx_exit = tx;
        tokio::spawn(async move {
            if let Ok(code) = pty_clone.wait_for_exit().await {
                let _ = tx_exit.send(PtyOutputEvent::Exit(code)).await;
            }
        });
    }

    Ok((pty, rx))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
#[cfg(unix)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::time::timeout;

    fn default_opts(shell: &str, args: &[&str]) -> PtySpawnOptions {
        PtySpawnOptions {
            shell: PathBuf::from(shell),
            args: args.iter().map(|s| s.to_string()).collect(),
            cwd: None,
            env: HashMap::new(),
            cols: 80,
            rows: 24,
        }
    }

    /// Drain all Data events until Exit arrives; returns collected bytes and exit code.
    async fn drain(
        mut rx: mpsc::Receiver<PtyOutputEvent>,
        deadline: Duration,
    ) -> (Vec<u8>, ExitCode) {
        let mut data = Vec::new();
        let mut code = ExitCode(u32::MAX);
        let _ = timeout(deadline, async {
            while let Some(event) = rx.recv().await {
                match event {
                    PtyOutputEvent::Data(d) => data.extend_from_slice(&d),
                    PtyOutputEvent::Exit(c) => {
                        code = c;
                        break;
                    }
                }
            }
        })
        .await;
        (data, code)
    }

    #[tokio::test]
    async fn spawn_echo_hello_exit_0() {
        let opts = default_opts("/bin/sh", &["-c", "echo hello"]);
        let (_pty, rx) = spawn(opts).expect("spawn failed");
        let (output, code) = drain(rx, Duration::from_secs(5)).await;
        let text = String::from_utf8_lossy(&output);
        assert!(
            text.contains("hello"),
            "expected 'hello' in output, got: {:?}",
            text
        );
        assert_eq!(code, ExitCode(0), "expected exit code 0");
    }

    #[tokio::test]
    async fn resize_returns_ok() {
        let opts = default_opts("/bin/sh", &["-c", "sleep 1"]);
        let (pty, _rx) = spawn(opts).expect("spawn failed");
        pty.resize(120, 40).expect("resize failed");
    }

    #[tokio::test]
    async fn kill_terminates_sleep() {
        let opts = default_opts("/bin/sh", &["-c", "sleep 30"]);
        let (pty, rx) = spawn(opts).expect("spawn failed");
        pty.kill().expect("kill failed");
        let (_output, code) = drain(rx, Duration::from_secs(5)).await;
        // Killed process: exit code != 0 (typically 137 on Linux, 143 on macOS, or signal-based).
        // We just assert it's not u32::MAX (meaning we did get an exit event).
        assert_ne!(code.0, u32::MAX, "expected exit event after kill");
    }
}
