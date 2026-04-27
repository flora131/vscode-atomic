//! Notebook data structures mirroring vscode.NotebookData etc.

use serde::{Deserialize, Serialize};

/// Mirrors vscode.NotebookCellKind.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NotebookCellKind {
    /// Markup (markdown) cell — vscode value 1.
    Markup,
    /// Code cell — vscode value 2.
    Code,
}

/// Mirrors vscode.NotebookCellExecutionSummary.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NotebookCellExecutionSummary {
    pub execution_order: Option<u64>,
    pub success: Option<bool>,
}

/// Mirrors vscode.NotebookCellOutputItem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotebookCellOutputItem {
    pub mime: String,
    /// Raw bytes of the output item.
    pub data: Vec<u8>,
}

/// Mirrors vscode.NotebookCellOutput.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotebookCellOutput {
    pub items: Vec<NotebookCellOutputItem>,
    pub metadata: serde_json::Value,
}

/// Mirrors vscode.NotebookCellData.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotebookCellData {
    pub kind: NotebookCellKind,
    pub value: String,
    pub language_id: String,
    pub outputs: Vec<NotebookCellOutput>,
    pub metadata: serde_json::Value,
    pub execution_summary: Option<NotebookCellExecutionSummary>,
}

/// Mirrors vscode.NotebookData.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotebookData {
    pub cells: Vec<NotebookCellData>,
    pub metadata: serde_json::Value,
}
