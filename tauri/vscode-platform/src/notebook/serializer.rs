//! NotebookSerializer trait and SerializerOptions.

use std::collections::HashMap;

use async_trait::async_trait;
use thiserror::Error;

use super::data::NotebookData;

/// Errors that can occur during notebook serialization/deserialization.
#[derive(Debug, Error)]
pub enum NotebookError {
    #[error("unsupported nbformat version: {0}; only nbformat >= 4 is supported")]
    UnsupportedNbformat(i64),

    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("invalid notebook: {0}")]
    Invalid(String),
}

/// Options controlling serialization behaviour, mirroring the TS options.
#[derive(Debug, Clone, Default)]
pub struct SerializerOptions {
    /// If true, outputs are not persisted on serialize.
    pub transient_outputs: bool,
    /// Per-metadata-key flags for transient cell metadata.
    pub transient_cell_metadata: HashMap<String, bool>,
    /// Per-metadata-key flags for cell content metadata.
    pub cell_content_metadata: HashMap<String, bool>,
}

/// Core notebook serializer trait.
#[async_trait]
pub trait NotebookSerializer: Send + Sync {
    /// Deserialize raw bytes (UTF-8 JSON `.ipynb`) into `NotebookData`.
    async fn deserialize(&self, content: &[u8]) -> Result<NotebookData, NotebookError>;

    /// Serialize `NotebookData` back to raw bytes.
    async fn serialize(&self, data: &NotebookData) -> Result<Vec<u8>, NotebookError>;
}
