//! DAP wire framing: `Content-Length: N\r\n\r\n<json>` per message.

use anyhow::{anyhow, Context, Result};
use tokio::io::{AsyncBufRead, AsyncBufReadExt, AsyncWrite, AsyncWriteExt};

use super::protocol::ProtocolMessage;

// ─────────────────────────────────────────────────────────────────────────────
// Read
// ─────────────────────────────────────────────────────────────────────────────

/// Read one DAP message from an `AsyncBufRead`.
///
/// Reads headers until `\r\n`, extracts `Content-Length`, then reads exactly
/// that many bytes and deserializes as `ProtocolMessage`.
pub async fn read_message<R>(reader: &mut R) -> Result<ProtocolMessage>
where
    R: AsyncBufRead + Unpin,
{
    // Read headers
    let mut content_length: Option<usize> = None;
    loop {
        let mut line = String::new();
        let n = reader.read_line(&mut line).await.context("reading DAP header")?;
        if n == 0 {
            return Err(anyhow!("DAP stream closed unexpectedly"));
        }
        let trimmed = line.trim_end_matches(['\r', '\n']);
        if trimmed.is_empty() {
            // blank line — headers done
            break;
        }
        if let Some(value) = trimmed.strip_prefix("Content-Length: ") {
            content_length = Some(value.parse::<usize>().context("invalid Content-Length")?);
        }
        // ignore other headers
    }

    let len = content_length.ok_or_else(|| anyhow!("DAP message missing Content-Length header"))?;

    let mut buf = vec![0u8; len];
    // read_exact via tokio BufReader requires AsyncReadExt; use fill_buf loop
    use tokio::io::AsyncReadExt;
    reader.read_exact(&mut buf).await.context("reading DAP body")?;

    let msg: ProtocolMessage =
        serde_json::from_slice(&buf).context("deserializing DAP ProtocolMessage")?;
    Ok(msg)
}

// ─────────────────────────────────────────────────────────────────────────────
// Write
// ─────────────────────────────────────────────────────────────────────────────

/// Write one DAP message to an `AsyncWrite`.
///
/// Serializes `msg` to JSON, prefixes with `Content-Length: N\r\n\r\n`.
pub async fn write_message<W>(writer: &mut W, msg: &ProtocolMessage) -> Result<()>
where
    W: AsyncWrite + Unpin,
{
    let json = serde_json::to_vec(msg).context("serializing DAP ProtocolMessage")?;
    let header = format!("Content-Length: {}\r\n\r\n", json.len());
    writer.write_all(header.as_bytes()).await.context("writing DAP header")?;
    writer.write_all(&json).await.context("writing DAP body")?;
    writer.flush().await.context("flushing DAP writer")?;
    Ok(())
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{BufReader, duplex};

    #[tokio::test]
    async fn round_trip_request() {
        let (mut client, mut server) = duplex(4096);

        let msg = ProtocolMessage::request(1, "initialize", None);
        write_message(&mut client, &msg).await.unwrap();

        let mut reader = BufReader::new(&mut server);
        let received = read_message(&mut reader).await.unwrap();

        assert_eq!(received.seq, 1);
        assert_eq!(received.command.as_deref(), Some("initialize"));
    }

    #[tokio::test]
    async fn round_trip_response() {
        use serde_json::json;
        let (mut client, mut server) = duplex(4096);

        let body = json!({ "supportsConfigurationDoneRequest": true });
        let msg = ProtocolMessage::response_ok(2, 1, "initialize", Some(body.clone()));
        write_message(&mut client, &msg).await.unwrap();

        let mut reader = BufReader::new(&mut server);
        let received = read_message(&mut reader).await.unwrap();

        assert_eq!(received.seq, 2);
        assert_eq!(received.request_seq, Some(1));
        assert_eq!(received.success, Some(true));
        assert_eq!(received.body.as_ref().unwrap()["supportsConfigurationDoneRequest"], true);
    }

    #[tokio::test]
    async fn round_trip_event() {
        use serde_json::json;
        let (mut client, mut server) = duplex(4096);

        let body = json!({ "reason": "breakpoint", "threadId": 1 });
        let msg = ProtocolMessage::event(3, "stopped", Some(body));
        write_message(&mut client, &msg).await.unwrap();

        let mut reader = BufReader::new(&mut server);
        let received = read_message(&mut reader).await.unwrap();

        assert_eq!(received.event.as_deref(), Some("stopped"));
        assert_eq!(received.body.as_ref().unwrap()["reason"], "breakpoint");
    }

    #[tokio::test]
    async fn multiple_messages_sequential() {
        let (mut client, mut server) = duplex(4096);

        let m1 = ProtocolMessage::request(1, "initialize", None);
        let m2 = ProtocolMessage::request(2, "launch", None);
        write_message(&mut client, &m1).await.unwrap();
        write_message(&mut client, &m2).await.unwrap();

        let mut reader = BufReader::new(&mut server);
        let r1 = read_message(&mut reader).await.unwrap();
        let r2 = read_message(&mut reader).await.unwrap();

        assert_eq!(r1.seq, 1);
        assert_eq!(r2.seq, 2);
    }

    #[tokio::test]
    async fn missing_content_length_errors() {
        use tokio::io::BufReader;
        // Malformed message — no Content-Length
        let bad = b"Bad-Header: value\r\n\r\n{}";
        let mut reader = BufReader::new(bad.as_ref());
        let err = read_message(&mut reader).await;
        assert!(err.is_err());
        assert!(err.unwrap_err().to_string().contains("Content-Length"));
    }
}
