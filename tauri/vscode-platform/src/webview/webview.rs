//! WebviewPanel and Webview types mirroring vscode.d.ts:10083-10164.

use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use uuid::Uuid;

/// Options controlling Webview capabilities.
#[derive(Debug, Clone)]
pub struct WebviewOptions {
    pub enable_scripts: bool,
    pub enable_command_uris: bool,
    pub local_resource_roots: Vec<String>,
    pub retain_context_when_hidden: bool,
}

impl Default for WebviewOptions {
    fn default() -> Self {
        Self {
            enable_scripts: false,
            enable_command_uris: false,
            local_resource_roots: Vec::new(),
            retain_context_when_hidden: false,
        }
    }
}

/// View column for panel placement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewColumn {
    Active,
    Beside,
    One,
    Two,
    Three,
}

/// Options for WebviewPanel behaviour.
#[derive(Debug, Clone, Default)]
pub struct WebviewPanelOptions {
    pub enable_find_widget: bool,
    pub retain_context_when_hidden: bool,
}

/// Emitter<T> — broadcast channel for events.
pub struct Emitter<T: Clone + Send + 'static> {
    tx: tokio::sync::broadcast::Sender<T>,
}

impl<T: Clone + Send + 'static> Emitter<T> {
    pub fn new() -> Self {
        let (tx, _) = tokio::sync::broadcast::channel(64);
        Self { tx }
    }

    pub fn emit(&self, value: T) {
        let _ = self.tx.send(value);
    }

    pub fn subscribe(&self) -> tokio::sync::broadcast::Receiver<T> {
        self.tx.subscribe()
    }
}

impl<T: Clone + Send + 'static> Default for Emitter<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Core Webview — holds HTML, message bus, URI translator.
pub struct Webview {
    pub id: Uuid,
    pub html: RwLock<String>,
    pub on_did_receive: Emitter<serde_json::Value>,
    pub post_message_tx: mpsc::Sender<serde_json::Value>,
    post_message_rx: tokio::sync::Mutex<mpsc::Receiver<serde_json::Value>>,
}

impl Webview {
    pub fn new() -> Arc<Self> {
        let (tx, rx) = mpsc::channel(64);
        Arc::new(Self {
            id: Uuid::new_v4(),
            html: RwLock::new(String::new()),
            on_did_receive: Emitter::new(),
            post_message_tx: tx,
            post_message_rx: tokio::sync::Mutex::new(rx),
        })
    }

    pub async fn html(&self) -> String {
        self.html.read().await.clone()
    }

    pub async fn set_html(&self, html: impl Into<String>) {
        *self.html.write().await = html.into();
    }

    /// Translate a filesystem path to a vscode-webview:// URI.
    ///
    /// Format: `vscode-webview://<panel-id>/<percent-encoded-path>`
    pub fn as_webview_uri(&self, path: &str) -> String {
        let encoded = percent_encode_path(path);
        format!("vscode-webview://{}/{}", self.id, encoded)
    }

    /// Receive the next message posted from the webview side.
    pub async fn recv_post_message(&self) -> Option<serde_json::Value> {
        self.post_message_rx.lock().await.recv().await
    }
}

impl Default for Webview {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel(64);
        Self {
            id: Uuid::new_v4(),
            html: RwLock::new(String::new()),
            on_did_receive: Emitter::new(),
            post_message_tx: tx,
            post_message_rx: tokio::sync::Mutex::new(rx),
        }
    }
}

/// WebviewPanel wraps a Webview and adds lifecycle events.
pub struct WebviewPanel {
    pub id: Uuid,
    pub view_type: String,
    pub title: String,
    pub webview: Arc<Webview>,
    pub on_did_dispose: Emitter<()>,
    pub on_did_change_view_state: Emitter<()>,
}

impl WebviewPanel {
    pub fn new(view_type: impl Into<String>, title: impl Into<String>) -> Arc<Self> {
        Arc::new(Self {
            id: Uuid::new_v4(),
            view_type: view_type.into(),
            title: title.into(),
            webview: Webview::new(),
            on_did_dispose: Emitter::new(),
            on_did_change_view_state: Emitter::new(),
        })
    }
}

fn percent_encode_path(path: &str) -> String {
    path.chars()
        .flat_map(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' | '/' => {
                vec![c]
            }
            other => {
                let s = other.to_string();
                s.bytes()
                    .flat_map(|b| {
                        format!("%{:02X}", b).chars().collect::<Vec<_>>()
                    })
                    .collect()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn webview_set_and_get_html() {
        let wv = Webview::new();
        wv.set_html("<html>hello</html>").await;
        assert_eq!(wv.html().await, "<html>hello</html>");
    }

    #[test]
    fn webview_as_webview_uri_format() {
        let wv = Webview::new();
        let uri = wv.as_webview_uri("/path/to/file.js");
        assert!(
            uri.starts_with("vscode-webview://"),
            "uri does not start with vscode-webview://: {}",
            uri
        );
        assert!(
            uri.contains(wv.id.to_string().as_str()),
            "uri missing panel id: {}",
            uri
        );
        assert!(uri.contains("file.js"), "uri missing path: {}", uri);
    }

    #[tokio::test]
    async fn webview_post_message_received_via_on_did_receive() {
        let wv = Webview::new();
        let mut sub = wv.on_did_receive.subscribe();

        // Simulate the webview side emitting a received message
        let msg = serde_json::json!({"type": "ready"});
        wv.on_did_receive.emit(msg.clone());

        let received = sub.recv().await.expect("should receive message");
        assert_eq!(received, msg);
    }

    #[tokio::test]
    async fn webview_post_message_tx_consumer_receives() {
        let wv = Webview::new();
        let tx = wv.post_message_tx.clone();

        let msg = serde_json::json!({"action": "update"});
        tx.send(msg.clone()).await.expect("send failed");

        let received = wv.recv_post_message().await.expect("should recv");
        assert_eq!(received, msg);
    }

    #[test]
    fn webview_panel_new_has_unique_ids() {
        let p1 = WebviewPanel::new("markdown.preview", "Title1");
        let p2 = WebviewPanel::new("markdown.preview", "Title2");
        assert_ne!(p1.id, p2.id);
    }
}
