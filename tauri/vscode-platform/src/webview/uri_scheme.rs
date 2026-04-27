//! URI scheme helpers for vscode-webview:// protocol.
//!
//! resolve_webview_uri validates that a requested path lies within
//! allowed_roots (rejects path traversal).
//! No Tauri runtime dependency — consumers wire the returned handler closure.

use std::path::{Component, PathBuf};
use thiserror::Error;

/// Errors from URI resolution.
#[derive(Debug, Error, PartialEq)]
pub enum WebviewError {
    #[error("path traversal detected: {0}")]
    PathTraversal(String),
    #[error("path {0} not within any allowed root")]
    NotAllowed(String),
    #[error("path contains null bytes")]
    NullBytes,
}

/// Resolve a raw path string to a canonical PathBuf, rejecting traversal.
///
/// `allowed_roots` — list of allowed root directory paths (as `&str` slices).
/// The path must be a child of at least one root; `..` components are rejected.
pub fn resolve_webview_uri(path: &str, allowed_roots: &[&str]) -> Result<PathBuf, WebviewError> {
    if path.contains('\0') {
        return Err(WebviewError::NullBytes);
    }

    let pb = PathBuf::from(path);

    // Reject any ParentDir components (path traversal).
    for component in pb.components() {
        if component == Component::ParentDir {
            return Err(WebviewError::PathTraversal(path.to_string()));
        }
    }

    // Must fall within at least one allowed root.
    if allowed_roots.is_empty() {
        return Ok(pb);
    }

    let canonical = if pb.is_absolute() {
        pb.clone()
    } else {
        // Relative paths: check against each root
        let mut found = None;
        for root in allowed_roots {
            let candidate = PathBuf::from(root).join(&pb);
            if candidate.starts_with(root) {
                found = Some(candidate);
                break;
            }
        }
        return found.ok_or_else(|| WebviewError::NotAllowed(path.to_string()));
    };

    for root in allowed_roots {
        if canonical.starts_with(root) {
            return Ok(canonical);
        }
    }

    Err(WebviewError::NotAllowed(path.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_allows_path_within_root() {
        let roots = &["/allowed/root"];
        let result = resolve_webview_uri("/allowed/root/file.js", roots);
        assert!(result.is_ok(), "expected ok: {:?}", result);
        assert_eq!(result.unwrap(), PathBuf::from("/allowed/root/file.js"));
    }

    #[test]
    fn resolve_rejects_parent_dir_traversal() {
        let roots = &["/allowed/root"];
        let result = resolve_webview_uri("/allowed/root/../secret.txt", roots);
        assert_eq!(result, Err(WebviewError::PathTraversal("/allowed/root/../secret.txt".to_string())));
    }

    #[test]
    fn resolve_rejects_pure_parent_dir() {
        let roots = &["/allowed"];
        let result = resolve_webview_uri("../../etc/passwd", roots);
        assert_eq!(result, Err(WebviewError::PathTraversal("../../etc/passwd".to_string())));
    }

    #[test]
    fn resolve_rejects_path_outside_all_roots() {
        let roots = &["/allowed/root"];
        let result = resolve_webview_uri("/other/path/file.js", roots);
        assert_eq!(result, Err(WebviewError::NotAllowed("/other/path/file.js".to_string())));
    }

    #[test]
    fn resolve_rejects_null_bytes() {
        let roots = &["/allowed"];
        let result = resolve_webview_uri("/allowed/file\0.js", roots);
        assert_eq!(result, Err(WebviewError::NullBytes));
    }

    #[test]
    fn resolve_with_empty_roots_allows_any_clean_path() {
        let result = resolve_webview_uri("/any/path/file.js", &[]);
        assert!(result.is_ok());
    }

    #[test]
    fn resolve_relative_path_joined_to_root() {
        let roots = &["/allowed/root"];
        let result = resolve_webview_uri("subdir/file.js", roots);
        assert!(result.is_ok(), "relative path should work: {:?}", result);
    }
}
