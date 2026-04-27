//! Transport trait + DuplexTransport adapter.
//!
//! Wraps any AsyncRead+AsyncWrite pair as a Transport.

use async_trait::async_trait;
use tokio::io::{AsyncRead, AsyncWrite, DuplexStream, ReadHalf, WriteHalf};

// ─────────────────────────────────────────────
// Transport trait
// ─────────────────────────────────────────────

/// A bidirectional byte-stream transport.
#[async_trait]
pub trait Transport: Send {
    type Read: AsyncRead + Unpin + Send;
    type Write: AsyncWrite + Unpin + Send;

    /// Split into read and write halves.
    fn split(self) -> (Self::Read, Self::Write);
}

// ─────────────────────────────────────────────
// DuplexTransport
// ─────────────────────────────────────────────

/// Wraps `tokio::io::DuplexStream` as a `Transport`.
pub struct DuplexTransport(pub DuplexStream);

impl Transport for DuplexTransport {
    type Read = ReadHalf<DuplexStream>;
    type Write = WriteHalf<DuplexStream>;

    fn split(self) -> (Self::Read, Self::Write) {
        tokio::io::split(self.0)
    }
}

/// A transport backed by arbitrary (read, write) halves.
pub struct SplitTransport<R, W> {
    pub read: R,
    pub write: W,
}

impl<R: AsyncRead + Unpin + Send + 'static, W: AsyncWrite + Unpin + Send + 'static> Transport
    for SplitTransport<R, W>
{
    type Read = R;
    type Write = W;

    fn split(self) -> (Self::Read, Self::Write) {
        (self.read, self.write)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    #[tokio::test]
    async fn duplex_transport_round_trip() {
        let (client, server) = tokio::io::duplex(256);
        let dt = DuplexTransport(client);
        let (mut r, mut w) = dt.split();

        let (mut sr, mut sw) = tokio::io::split(server);

        sw.write_all(b"ping").await.unwrap();
        let mut buf = [0u8; 4];
        r.read_exact(&mut buf).await.unwrap();
        assert_eq!(&buf, b"ping");

        w.write_all(b"pong").await.unwrap();
        let mut buf2 = [0u8; 4];
        sr.read_exact(&mut buf2).await.unwrap();
        assert_eq!(&buf2, b"pong");
    }
}
