//! Serialization: NotebookData → JSON bytes.
//! Mirrors extensions/ipynb/src/serializers.ts logic.

use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use serde_json::{json, Map, Value};

use crate::notebook::{
    data::{NotebookCellData, NotebookCellKind, NotebookCellOutput, NotebookData},
    serializer::NotebookError,
};

const DEFAULT_NBFORMAT: i64 = 4;
const DEFAULT_NBFORMAT_MINOR: i64 = 5;

/// Serialize NotebookData to UTF-8 JSON bytes (trailing newline, 2-space indent).
pub fn notebook_data_to_bytes(
    data: &NotebookData,
    preferred_language: &str,
) -> Result<Vec<u8>, NotebookError> {
    let nb_json = notebook_data_to_json(data, preferred_language)?;
    // ipynb always ends with trailing newline
    let mut s = serde_json::to_string_pretty(&nb_json)?;
    s.push('\n');
    Ok(s.into_bytes())
}

fn notebook_data_to_json(
    data: &NotebookData,
    preferred_language: &str,
) -> Result<Value, NotebookError> {
    // Start from stored metadata (which has nbformat, nbformat_minor, metadata)
    let mut nb = if let Value::Object(ref m) = data.metadata {
        m.clone()
    } else {
        Map::new()
    };

    // Ensure nbformat fields
    nb.entry("nbformat")
        .or_insert_with(|| json!(DEFAULT_NBFORMAT));
    nb.entry("nbformat_minor")
        .or_insert_with(|| json!(DEFAULT_NBFORMAT_MINOR));
    nb.entry("metadata").or_insert_with(|| json!({}));

    // Determine preferred cell language from notebook metadata
    let pref_lang = nb
        .get("metadata")
        .and_then(|m| m.get("language_info"))
        .and_then(|li| li.get("name"))
        .and_then(|n| n.as_str())
        .unwrap_or(preferred_language);

    let cells: Vec<Value> = data
        .cells
        .iter()
        .map(|c| cell_data_to_jupyter(c, pref_lang))
        .collect::<Result<_, _>>()?;

    nb.insert("cells".into(), Value::Array(cells));

    Ok(Value::Object(nb))
}

fn cell_data_to_jupyter(cell: &NotebookCellData, preferred_language: &str) -> Result<Value, NotebookError> {
    let source = split_source_for_jupyter(&cell.value);

    // Recover id if stored in metadata
    let id = cell
        .metadata
        .get("id")
        .and_then(|v| v.as_str())
        .map(|s| Value::String(s.to_string()));

    let cell_metadata = cell
        .metadata
        .get("metadata")
        .cloned()
        .unwrap_or(json!({}));

    match cell.kind {
        NotebookCellKind::Markup => {
            let mut obj = Map::new();
            obj.insert("cell_type".into(), json!("markdown"));
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            obj.insert("metadata".into(), cell_metadata);
            obj.insert("source".into(), source);
            Ok(Value::Object(obj))
        }
        NotebookCellKind::Code => {
            if cell.language_id == "raw" {
                let mut obj = Map::new();
                obj.insert("cell_type".into(), json!("raw"));
                if let Some(id) = id {
                    obj.insert("id".into(), id);
                }
                obj.insert("metadata".into(), cell_metadata);
                obj.insert("source".into(), source);
                return Ok(Value::Object(obj));
            }

            // Regular code cell
            let execution_count = cell
                .execution_summary
                .as_ref()
                .and_then(|es| es.execution_order)
                .map(|n| json!(n))
                .unwrap_or(Value::Null);

            let outputs: Vec<Value> = cell
                .outputs
                .iter()
                .map(|o| output_to_jupyter(o))
                .collect::<Result<_, _>>()?;

            // Add vscode languageId to metadata if different from preferred
            let mut meta_map = if let Value::Object(ref m) = cell_metadata {
                m.clone()
            } else {
                Map::new()
            };
            if cell.language_id != preferred_language && cell.language_id != "raw" {
                let mut vscode_meta = Map::new();
                vscode_meta.insert("languageId".into(), json!(cell.language_id));
                meta_map.insert("vscode".into(), Value::Object(vscode_meta));
            }

            let mut obj = Map::new();
            obj.insert("cell_type".into(), json!("code"));
            obj.insert("execution_count".into(), execution_count);
            if let Some(id) = id {
                obj.insert("id".into(), id);
            }
            obj.insert("metadata".into(), Value::Object(meta_map));
            obj.insert("outputs".into(), Value::Array(outputs));
            obj.insert("source".into(), source);
            Ok(Value::Object(obj))
        }
    }
}

/// Convert a NotebookCellOutput back to a Jupyter output JSON object.
fn output_to_jupyter(output: &NotebookCellOutput) -> Result<Value, NotebookError> {
    let output_type = output
        .metadata
        .get("outputType")
        .and_then(|v| v.as_str())
        .unwrap_or("display_data");

    match output_type {
        "stream" => {
            let item = output
                .items
                .first()
                .ok_or_else(|| NotebookError::Invalid("stream output has no items".into()))?;
            let name = if item.mime.contains("stderr") {
                "stderr"
            } else {
                "stdout"
            };
            let text = String::from_utf8_lossy(&item.data).into_owned();
            Ok(json!({
                "output_type": "stream",
                "name": name,
                "text": text
            }))
        }
        "error" => {
            let item = output
                .items
                .first()
                .ok_or_else(|| NotebookError::Invalid("error output has no items".into()))?;
            let err: Value = serde_json::from_slice(&item.data).unwrap_or(json!({}));
            let stack = err
                .get("stack")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .split('\n')
                .map(|s| Value::String(s.to_string()))
                .collect::<Vec<_>>();
            Ok(json!({
                "output_type": "error",
                "ename": err.get("name").and_then(|v| v.as_str()).unwrap_or(""),
                "evalue": err.get("message").and_then(|v| v.as_str()).unwrap_or(""),
                "traceback": stack
            }))
        }
        _ => {
            // display_data / execute_result / update_display_data
            let mut data_map = Map::new();
            for item in &output.items {
                let value = if item.mime.starts_with("image/") && item.mime != "image/svg+xml" {
                    // Re-encode to base64
                    Value::String(BASE64.encode(&item.data))
                } else {
                    // Try to parse as UTF-8 text
                    Value::String(String::from_utf8_lossy(&item.data).into_owned())
                };
                data_map.insert(item.mime.clone(), value);
            }

            let meta = output
                .metadata
                .get("metadata")
                .cloned()
                .unwrap_or(json!({}));
            let exec_count = output.metadata.get("executionCount").cloned();

            let mut obj = Map::new();
            obj.insert("output_type".into(), json!(output_type));
            obj.insert("data".into(), Value::Object(data_map));
            obj.insert("metadata".into(), meta);
            if let Some(ec) = exec_count {
                obj.insert("execution_count".into(), ec);
            }
            Ok(Value::Object(obj))
        }
    }
}

/// Split a multiline string into an array of lines (each keeping trailing `\n`
/// except the last), mirroring the TS serializer.
fn split_source_for_jupyter(source: &str) -> Value {
    if source.is_empty() {
        return Value::Array(vec![]);
    }
    let lines: Vec<&str> = source.split_inclusive('\n').collect();
    let arr: Vec<Value> = lines
        .iter()
        .map(|s| Value::String(s.to_string()))
        .collect();
    Value::Array(arr)
}
