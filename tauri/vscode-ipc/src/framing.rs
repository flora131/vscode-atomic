//! JSON-RPC framing over AsyncRead/AsyncWrite.
//!
//! Each message is a newline-terminated JSON object (same as cli/src/json_rpc.rs).
//! A `Content-Length` variant is also provided for LSP-style framing.

use tokio::{
    io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader},
    pin,
    sync::mpsc,
};

use crate::{
    dispatcher::{MaybeSync, RpcDispatcher, Serialization},
    sync::Barrier,
};

// ─────────────────────────────────────────────
// JsonRpcSerializer
// ─────────────────────────────────────────────

/// Serializes to a newline-terminated JSON byte string.
#[derive(Clone, Debug)]
pub struct JsonRpcSerializer;

impl JsonRpcSerializer {
    pub fn new() -> Self {
        Self
    }
}

impl Default for JsonRpcSerializer {
    fn default() -> Self {
        Self::new()
    }
}

impl Serialization for JsonRpcSerializer {
    fn serialize(&self, value: &impl serde::Serialize) -> Vec<u8> {
        let mut v = serde_json::to_vec(value).expect("serialize failed");
        v.push(b'\n');
        v
    }

    fn deserialize<P: serde::de::DeserializeOwned>(&self, b: &[u8]) -> anyhow::Result<P> {
        serde_json::from_slice(b).map_err(|e| anyhow::anyhow!("deserialize error: {e}"))
    }
}

// ─────────────────────────────────────────────
// start_json_rpc — newline-framed async loop
// ─────────────────────────────────────────────

/// Run the JSON-RPC read/write loop until:
/// - `shutdown_rx` barrier fires, or
/// - the read side returns EOF, or
/// - a write error occurs.
///
/// Returns `Ok(Some(S))` when shutdown barrier fired, `Ok(None)` on EOF.
pub async fn start_json_rpc<C, S>(
    dispatcher: RpcDispatcher<JsonRpcSerializer, C>,
    read: impl AsyncRead + Unpin,
    mut write: impl AsyncWrite + Unpin,
    mut shutdown_rx: Barrier<S>,
) -> std::io::Result<Option<S>>
where
    C: Send + Sync + 'static,
    S: Clone + Send + Sync + 'static,
{
    let (write_tx, mut write_rx) = mpsc::channel::<Vec<u8>>(64);
    let mut reader = BufReader::new(read);
    let mut read_buf = String::new();

    let shutdown_fut = shutdown_rx.wait();
    pin!(shutdown_fut);

    loop {
        tokio::select! {
            // Shutdown barrier fired
            shutdown_val = &mut shutdown_fut => {
                return Ok(shutdown_val.ok());
            }

            // Outbound data ready
            Some(w) = write_rx.recv() => {
                write.write_all(&w).await?;
            }

            // Inbound line
            n = reader.read_line(&mut read_buf) => {
                let result = match n {
                    Ok(0) => return Ok(None), // EOF
                    Ok(n) => dispatcher.dispatch(&read_buf.as_bytes()[..n]),
                    Err(e) => return Err(e),
                };
                read_buf.truncate(0);

                match result {
                    MaybeSync::Sync(Some(v)) => {
                        write.write_all(&v).await?;
                    }
                    MaybeSync::Sync(None) => {}
                    MaybeSync::Future(fut) => {
                        let tx = write_tx.clone();
                        tokio::spawn(async move {
                            if let Some(v) = fut.await {
                                let _ = tx.send(v).await;
                            }
                        });
                    }
                }
            }
        }
    }
}

// ─────────────────────────────────────────────
// Content-Length framed writer/reader helpers
// ─────────────────────────────────────────────

/// Write a single message with `Content-Length` framing (LSP-style).
pub async fn write_content_length(
    write: &mut (impl AsyncWrite + Unpin),
    body: &[u8],
) -> std::io::Result<()> {
    let header = format!("Content-Length: {}\r\n\r\n", body.len());
    write.write_all(header.as_bytes()).await?;
    write.write_all(body).await?;
    Ok(())
}

/// Read a single `Content-Length`-framed message.
pub async fn read_content_length(
    reader: &mut (impl tokio::io::AsyncBufRead + Unpin),
) -> std::io::Result<Vec<u8>> {
    let mut content_length: Option<usize> = None;
    let mut header_buf = String::new();

    // Read headers
    loop {
        header_buf.clear();
        let n = reader.read_line(&mut header_buf).await?;
        if n == 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "EOF in headers",
            ));
        }
        let line = header_buf.trim_end_matches(['\r', '\n']);
        if line.is_empty() {
            break; // blank line separates headers from body
        }
        if let Some(rest) = line.strip_prefix("Content-Length: ") {
            content_length = rest.trim().parse().ok();
        }
    }

    let len = content_length.ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::InvalidData, "Missing Content-Length")
    })?;

    let mut body = vec![0u8; len];
    use tokio::io::AsyncReadExt;
    reader.read_exact(&mut body).await?;
    Ok(body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_appends_newline() {
        let s = JsonRpcSerializer::new();
        let v = s.serialize(&serde_json::json!({"a": 1}));
        assert!(v.ends_with(b"\n"));
        let parsed: serde_json::Value = serde_json::from_slice(&v).unwrap();
        assert_eq!(parsed["a"], 1);
    }

    #[tokio::test]
    async fn content_length_round_trip() {
        let (mut client, server) = tokio::io::duplex(4096);
        let body = b"{\"method\":\"test\"}";
        write_content_length(&mut client, body).await.unwrap();

        let mut server_reader = tokio::io::BufReader::new(server);
        let received = read_content_length(&mut server_reader).await.unwrap();
        assert_eq!(received, body);
    }
}
