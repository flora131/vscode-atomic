//! Base error types.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum VsCodeError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialisation error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("{0}")]
    Other(String),
}
