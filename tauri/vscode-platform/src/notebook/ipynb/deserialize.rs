//! Deserialization: JSON bytes → NotebookData.
//! Mirrors extensions/ipynb/src/deserializers.ts logic.

use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use serde_json::Value;

use crate::notebook::{
    data::{
        NotebookCellData, NotebookCellExecutionSummary, NotebookCellKind, NotebookCellOutput,
        NotebookCellOutputItem, NotebookData,
    },
    serializer::NotebookError,
};

/// Concatenate multiline source: string | string[] → String, normalise CRLF.
pub fn concat_multiline_source(src: &Value) -> String {
    let raw = match src {
        Value::String(s) => s.clone(),
        Value::Array(arr) => {
            let mut out = String::new();
            for (i, v) in arr.iter().enumerate() {
                let s = v.as_str().unwrap_or("");
                if i < arr.len() - 1 && !s.ends_with('\n') {
                    out.push_str(s);
                    out.push('\n');
                } else {
                    out.push_str(s);
                }
            }
            out
        }
        _ => String::new(),
    };
    raw.replace("\r\n", "\n")
}

/// Derive preferred language from notebook metadata.
pub fn get_preferred_language(metadata: Option<&Value>) -> String {
    if let Some(meta) = metadata {
        if let Some(lang) = meta
            .get("language_info")
            .and_then(|li| li.get("name"))
            .and_then(|n| n.as_str())
        {
            return translate_kernel_language(lang);
        }
        if let Some(lang) = meta
            .get("kernelspec")
            .and_then(|ks| ks.get("language"))
            .and_then(|l| l.as_str())
        {
            return translate_kernel_language(lang);
        }
    }
    "python".to_string()
}

fn translate_kernel_language(language: &str) -> String {
    let lang = language.to_lowercase();
    match lang.as_str() {
        "c#" | "csharp" => "csharp".to_string(),
        "f#" | "fsharp" => "fsharp".to_string(),
        "q#" | "qsharp" => "qsharp".to_string(),
        "c++11" | "c++12" | "c++14" => "c++".to_string(),
        _ => lang,
    }
}

/// Convert a single Jupyter output JSON object to NotebookCellOutput.
fn convert_output(output: &Value) -> NotebookCellOutput {
    let output_type = output
        .get("output_type")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    match output_type {
        "stream" => {
            let name = output
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("stdout");
            let text = output
                .get("text")
                .map(|v| concat_multiline_source(v))
                .unwrap_or_default();
            let mime = if name == "stderr" {
                "application/vnd.code.notebook.stderr"
            } else {
                "application/vnd.code.notebook.stdout"
            };
            let item = NotebookCellOutputItem {
                mime: mime.to_string(),
                data: text.into_bytes(),
            };
            NotebookCellOutput {
                items: vec![item],
                metadata: Value::Object({
                    let mut m = serde_json::Map::new();
                    m.insert("outputType".into(), Value::String("stream".into()));
                    m
                }),
            }
        }
        "error" => {
            let ename = output
                .get("ename")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let evalue = output
                .get("evalue")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let traceback = output
                .get("traceback")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str())
                        .collect::<Vec<_>>()
                        .join("\n")
                })
                .unwrap_or_default();
            let err_obj = serde_json::json!({
                "name": ename,
                "message": evalue,
                "stack": traceback,
            });
            let item = NotebookCellOutputItem {
                mime: "application/vnd.code.notebook.error".to_string(),
                data: serde_json::to_vec(&err_obj).unwrap_or_default(),
            };
            NotebookCellOutput {
                items: vec![item],
                metadata: Value::Object({
                    let mut m = serde_json::Map::new();
                    m.insert("outputType".into(), Value::String("error".into()));
                    m
                }),
            }
        }
        // display_data, execute_result, update_display_data
        _ => {
            let data_map = output.get("data").and_then(|v| v.as_object());
            let items: Vec<NotebookCellOutputItem> = data_map
                .map(|map| {
                    map.iter()
                        .map(|(mime, value)| convert_mime_value(mime, value))
                        .collect()
                })
                .unwrap_or_default();

            let mut meta_obj = serde_json::Map::new();
            meta_obj.insert(
                "outputType".into(),
                Value::String(output_type.to_string()),
            );
            if let Some(m) = output.get("metadata") {
                meta_obj.insert("metadata".into(), m.clone());
            }
            if let Some(ec) = output.get("execution_count") {
                meta_obj.insert("executionCount".into(), ec.clone());
            }

            NotebookCellOutput {
                items,
                metadata: Value::Object(meta_obj),
            }
        }
    }
}

/// Convert a single mime/value pair from Jupyter output data dict.
fn convert_mime_value(mime: &str, value: &Value) -> NotebookCellOutputItem {
    // Images (except svg) are base64-encoded in ipynb; decode to bytes.
    if mime.starts_with("image/") && mime != "image/svg+xml" {
        if let Some(b64) = value.as_str() {
            // Strip any whitespace that may appear in multiline base64
            let cleaned: String = b64.chars().filter(|c| !c.is_whitespace()).collect();
            if let Ok(bytes) = BASE64.decode(&cleaned) {
                return NotebookCellOutputItem {
                    mime: mime.to_string(),
                    data: bytes,
                };
            }
        }
    }

    // Text/array-of-strings
    if mime.starts_with("text/") || matches!(mime, "application/json") {
        let text = match value {
            Value::String(s) => s.clone(),
            Value::Array(_) => concat_multiline_source(value),
            other => serde_json::to_string(other).unwrap_or_default(),
        };
        return NotebookCellOutputItem {
            mime: mime.to_string(),
            data: text.into_bytes(),
        };
    }

    // Everything else: JSON-encode value
    let data = serde_json::to_vec(value).unwrap_or_default();
    NotebookCellOutputItem {
        mime: mime.to_string(),
        data,
    }
}

/// Core mapping: Jupyter INotebookContent JSON → NotebookData.
pub fn jupyter_notebook_model_to_notebook_data(
    json: &Value,
    preferred_language: &str,
) -> Result<NotebookData, NotebookError> {
    let cells_json = match json.get("cells").and_then(|v| v.as_array()) {
        Some(arr) => arr,
        None => {
            return Err(NotebookError::Invalid(
                "notebook content is missing cells array".into(),
            ))
        }
    };

    let metadata = {
        // Store everything except cells as notebook metadata
        let mut m = json.clone();
        if let Value::Object(ref mut map) = m {
            map.remove("cells");
        }
        m
    };

    let cells: Vec<NotebookCellData> = cells_json
        .iter()
        .filter_map(|cell| create_cell_data(cell, preferred_language))
        .collect();

    Ok(NotebookData { cells, metadata })
}

fn create_cell_data(cell: &Value, preferred_language: &str) -> Option<NotebookCellData> {
    let cell_type = cell.get("cell_type")?.as_str()?;
    let source = cell
        .get("source")
        .map(|s| concat_multiline_source(s))
        .unwrap_or_default();
    match cell_type {
        "code" => {
            let outputs: Vec<NotebookCellOutput> = cell
                .get("outputs")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().map(convert_output).collect())
                .unwrap_or_default();

            // Determine cell language: check vscode custom metadata first
            let lang_id = cell
                .get("metadata")
                .and_then(|m| m.get("vscode"))
                .and_then(|v| v.get("languageId"))
                .and_then(|l| l.as_str())
                .unwrap_or(preferred_language)
                .to_string();

            let execution_order = cell
                .get("execution_count")
                .and_then(|v| v.as_u64())
                .filter(|&n| n > 0);

            // Build metadata preserving execution_count, id, attachments
            let mut meta_obj = serde_json::Map::new();
            if let Some(ec) = cell.get("execution_count") {
                meta_obj.insert("execution_count".into(), ec.clone());
            }
            if let Some(m) = cell.get("metadata") {
                meta_obj.insert("metadata".into(), m.clone());
            }
            if let Some(id) = cell.get("id").and_then(|v| v.as_str()) {
                meta_obj.insert("id".into(), Value::String(id.to_string()));
            }

            Some(NotebookCellData {
                kind: NotebookCellKind::Code,
                value: source,
                language_id: lang_id,
                outputs,
                metadata: Value::Object(meta_obj),
                execution_summary: Some(NotebookCellExecutionSummary {
                    execution_order,
                    success: None,
                }),
            })
        }
        "markdown" => {
            let mut meta_obj = serde_json::Map::new();
            if let Some(m) = cell.get("metadata") {
                meta_obj.insert("metadata".into(), m.clone());
            }
            if let Some(id) = cell.get("id").and_then(|v| v.as_str()) {
                meta_obj.insert("id".into(), Value::String(id.to_string()));
            }
            if let Some(attachments) = cell.get("attachments") {
                meta_obj.insert("attachments".into(), attachments.clone());
            }

            Some(NotebookCellData {
                kind: NotebookCellKind::Markup,
                value: source,
                language_id: "markdown".to_string(),
                outputs: vec![],
                metadata: Value::Object(meta_obj),
                execution_summary: None,
            })
        }
        "raw" => {
            let mut meta_obj = serde_json::Map::new();
            if let Some(m) = cell.get("metadata") {
                meta_obj.insert("metadata".into(), m.clone());
            }
            if let Some(id) = cell.get("id").and_then(|v| v.as_str()) {
                meta_obj.insert("id".into(), Value::String(id.to_string()));
            }

            Some(NotebookCellData {
                kind: NotebookCellKind::Code,
                value: source,
                language_id: "raw".to_string(),
                outputs: vec![],
                metadata: Value::Object(meta_obj),
                execution_summary: None,
            })
        }
        _ => None,
    }
}
