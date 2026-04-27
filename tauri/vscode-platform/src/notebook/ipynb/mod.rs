//! ipynb-format notebook serializer.

pub mod deserialize;
pub mod serialize;

use async_trait::async_trait;

use crate::notebook::{
    data::NotebookData,
    serializer::{NotebookError, NotebookSerializer, SerializerOptions},
};

use self::{
    deserialize::{get_preferred_language, jupyter_notebook_model_to_notebook_data},
    serialize::notebook_data_to_bytes,
};

/// Serializer for `.ipynb` (Jupyter) notebooks.
pub struct IpynbSerializer {
    pub options: SerializerOptions,
    /// Fallback language when notebook metadata doesn't specify one.
    pub preferred_language: String,
}

impl IpynbSerializer {
    pub fn new(options: SerializerOptions, preferred_language: String) -> Self {
        Self {
            options,
            preferred_language,
        }
    }
}

#[async_trait]
impl NotebookSerializer for IpynbSerializer {
    async fn deserialize(&self, content: &[u8]) -> Result<NotebookData, NotebookError> {
        // Empty content → empty notebook
        let text = std::str::from_utf8(content)
            .map_err(|e| NotebookError::Invalid(format!("invalid UTF-8: {e}")))?;

        if text.trim().is_empty() {
            return Ok(NotebookData {
                cells: vec![],
                metadata: serde_json::Value::Object(Default::default()),
            });
        }

        let json: serde_json::Value = serde_json::from_str(text)?;

        // nbformat validation: reject < 4
        if let Some(nbformat) = json.get("nbformat").and_then(|v| v.as_i64()) {
            if nbformat < 4 {
                return Err(NotebookError::UnsupportedNbformat(nbformat));
            }
        }

        let preferred_lang = get_preferred_language(json.get("metadata"));
        let lang = if preferred_lang == "python" && self.preferred_language != "python" {
            &self.preferred_language
        } else {
            &preferred_lang
        };

        jupyter_notebook_model_to_notebook_data(&json, lang)
    }

    async fn serialize(&self, data: &NotebookData) -> Result<Vec<u8>, NotebookError> {
        notebook_data_to_bytes(data, &self.preferred_language)
    }
}
