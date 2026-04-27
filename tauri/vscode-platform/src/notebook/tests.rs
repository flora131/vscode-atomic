#[cfg(test)]
mod tests {
    use crate::notebook::{
        data::NotebookCellKind,
        ipynb::IpynbSerializer,
        serializer::{NotebookSerializer, SerializerOptions},
    };

    const SAMPLE_NOTEBOOK: &str = r###"{
  "nbformat": 4,
  "nbformat_minor": 5,
  "metadata": {
    "kernelspec": {
      "display_name": "Python 3",
      "language": "python",
      "name": "python3"
    },
    "language_info": {
      "name": "python",
      "version": "3.8.0"
    }
  },
  "cells": [
    {
      "cell_type": "code",
      "execution_count": 1,
      "id": "cell-001",
      "metadata": {},
      "outputs": [
        {
          "output_type": "stream",
          "name": "stdout",
          "text": ["Hello, world!\n"]
        }
      ],
      "source": ["print('Hello, world!')"]
    },
    {
      "cell_type": "markdown",
      "id": "cell-002",
      "metadata": {},
      "source": ["## Section\n", "Some text."]
    },
    {
      "cell_type": "raw",
      "id": "cell-003",
      "metadata": {},
      "source": ["raw content"]
    }
  ]
}
"###;

    const NBFORMAT3_NOTEBOOK: &str = r#"{
  "nbformat": 3,
  "nbformat_minor": 0,
  "metadata": {},
  "worksheets": []
}"#;

    const DISPLAY_DATA_NOTEBOOK: &str = r#"{
  "nbformat": 4,
  "nbformat_minor": 5,
  "metadata": {},
  "cells": [
    {
      "cell_type": "code",
      "execution_count": 1,
      "id": "cell-img",
      "metadata": {},
      "outputs": [
        {
          "output_type": "display_data",
          "data": {
            "image/png": "iVBORw0KGgo=",
            "text/plain": ["<Figure>"]
          },
          "metadata": {}
        }
      ],
      "source": ["import matplotlib"]
    }
  ]
}"#;

    fn make_serializer() -> IpynbSerializer {
        IpynbSerializer::new(SerializerOptions::default(), "python".to_string())
    }

    #[tokio::test]
    async fn deserialize_preserves_cell_count() {
        let s = make_serializer();
        let data = s.deserialize(SAMPLE_NOTEBOOK.as_bytes()).await.unwrap();
        assert_eq!(data.cells.len(), 3);
    }

    #[tokio::test]
    async fn deserialize_preserves_cell_languages() {
        let s = make_serializer();
        let data = s.deserialize(SAMPLE_NOTEBOOK.as_bytes()).await.unwrap();
        assert_eq!(data.cells[0].language_id, "python");
        assert_eq!(data.cells[1].language_id, "markdown");
        assert_eq!(data.cells[2].language_id, "raw");
    }

    #[tokio::test]
    async fn deserialize_preserves_output_count() {
        let s = make_serializer();
        let data = s.deserialize(SAMPLE_NOTEBOOK.as_bytes()).await.unwrap();
        assert_eq!(data.cells[0].outputs.len(), 1);
        assert_eq!(data.cells[1].outputs.len(), 0);
    }

    #[tokio::test]
    async fn deserialize_cell_kinds() {
        let s = make_serializer();
        let data = s.deserialize(SAMPLE_NOTEBOOK.as_bytes()).await.unwrap();
        assert!(matches!(data.cells[0].kind, NotebookCellKind::Code));
        assert!(matches!(data.cells[1].kind, NotebookCellKind::Markup));
        // raw maps to Code kind
        assert!(matches!(data.cells[2].kind, NotebookCellKind::Code));
    }

    #[tokio::test]
    async fn round_trip_preserves_cell_count() {
        let s = make_serializer();
        let data = s.deserialize(SAMPLE_NOTEBOOK.as_bytes()).await.unwrap();
        let bytes = s.serialize(&data).await.unwrap();
        let data2 = s.deserialize(&bytes).await.unwrap();
        assert_eq!(data.cells.len(), data2.cells.len());
    }

    #[tokio::test]
    async fn round_trip_preserves_cell_languages() {
        let s = make_serializer();
        let data = s.deserialize(SAMPLE_NOTEBOOK.as_bytes()).await.unwrap();
        let bytes = s.serialize(&data).await.unwrap();
        let data2 = s.deserialize(&bytes).await.unwrap();
        for (c1, c2) in data.cells.iter().zip(data2.cells.iter()) {
            assert_eq!(c1.language_id, c2.language_id);
        }
    }

    #[tokio::test]
    async fn round_trip_preserves_output_count() {
        let s = make_serializer();
        let data = s.deserialize(SAMPLE_NOTEBOOK.as_bytes()).await.unwrap();
        let bytes = s.serialize(&data).await.unwrap();
        let data2 = s.deserialize(&bytes).await.unwrap();
        for (c1, c2) in data.cells.iter().zip(data2.cells.iter()) {
            assert_eq!(c1.outputs.len(), c2.outputs.len());
        }
    }

    #[tokio::test]
    async fn reject_nbformat3_with_clear_error() {
        let s = make_serializer();
        let err = s
            .deserialize(NBFORMAT3_NOTEBOOK.as_bytes())
            .await
            .unwrap_err();
        let msg = err.to_string();
        assert!(
            msg.contains("nbformat") || msg.contains("version") || msg.contains("4"),
            "expected error about nbformat version, got: {msg}"
        );
    }

    #[tokio::test]
    async fn deserialize_image_output_decoded_from_base64() {
        let s = make_serializer();
        let data = s.deserialize(DISPLAY_DATA_NOTEBOOK.as_bytes()).await.unwrap();
        let output = &data.cells[0].outputs[0];
        let png_item = output.items.iter().find(|i| i.mime == "image/png");
        assert!(png_item.is_some(), "expected image/png output item");
        let item = png_item.unwrap();
        assert!(!item.data.is_empty());
    }

    #[tokio::test]
    async fn serialize_produces_valid_json_with_nbformat4() {
        let s = make_serializer();
        let data = s.deserialize(SAMPLE_NOTEBOOK.as_bytes()).await.unwrap();
        let bytes = s.serialize(&data).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(json["nbformat"], 4);
    }

    #[tokio::test]
    async fn concat_multiline_source_is_normalized() {
        let s = make_serializer();
        let data = s.deserialize(SAMPLE_NOTEBOOK.as_bytes()).await.unwrap();
        // markdown cell: source was ["## Section\n", "Some text."]
        // JSON \n becomes actual newline after serde_json parsing
        assert_eq!(data.cells[1].value, "## Section\nSome text.");
    }

    #[tokio::test]
    async fn deserialize_empty_bytes_returns_empty_notebook() {
        let s = make_serializer();
        let data = s.deserialize(b"").await.unwrap();
        assert_eq!(data.cells.len(), 0);
    }
}
