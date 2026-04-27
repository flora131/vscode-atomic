//! URI — port of `src/vs/base/common/uri.ts`.
//!
//! Parses `scheme://authority/path?query#fragment`.
//! Uses the `url` crate for RFC-3986 parsing, but preserves VS Code–specific
//! custom schemes (vscode-file://, vscode-webview://, etc.).
//!
//! `VsUri` is immutable; mutation creates a new value (`with`).

use std::fmt;
use url::Url;

// ─────────────────────────────────────────────────────────────────────────────
// VsUri
// ─────────────────────────────────────────────────────────────────────────────

/// Immutable URI value (mirrors TS `URI` class).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VsUri {
    pub scheme:    String,
    pub authority: String,
    pub path:      String,
    pub query:     String,
    pub fragment:  String,
}

impl VsUri {
    /// Create from components directly (no URL encoding — caller responsible).
    pub fn new(
        scheme: impl Into<String>,
        authority: impl Into<String>,
        path: impl Into<String>,
        query: impl Into<String>,
        fragment: impl Into<String>,
    ) -> Self {
        Self {
            scheme:    scheme.into(),
            authority: authority.into(),
            path:      path.into(),
            query:     query.into(),
            fragment:  fragment.into(),
        }
    }

    /// Parse a URI string. Falls back to `file` scheme if none present (TS compat).
    pub fn parse(input: &str) -> Result<Self, UriError> {
        // First try standard URL parsing.
        if let Ok(url) = Url::parse(input) {
            let scheme    = url.scheme().to_owned();
            let authority = url.host_str().map(|h| {
                if let Some(port) = url.port() {
                    format!("{h}:{port}")
                } else {
                    h.to_owned()
                }
            }).unwrap_or_default();
            let path     = url.path().to_owned();
            let query    = url.query().unwrap_or("").to_owned();
            let fragment = url.fragment().unwrap_or("").to_owned();
            return Ok(Self { scheme, authority, path, query, fragment });
        }

        // Fallback: hand-roll parse for schemes unknown to the `url` crate.
        Self::parse_raw(input)
    }

    /// Hand-rolled parser matching the TS regex:
    /// `/^(([^:/?#]+?):)?(\/\/([^/?#]*))?([^?#]*)(\?([^#]*))?(#(.*))?/`
    fn parse_raw(input: &str) -> Result<Self, UriError> {
        let s = input;
        let mut scheme    = String::new();
        let mut authority = String::new();
        let mut path      = String::new();

        let rest = if let Some(pos) = s.find(':') {
            // Potential scheme: only valid chars before ':'
            let candidate = &s[..pos];
            if candidate.chars().all(|c| c.is_alphanumeric() || c == '+' || c == '-' || c == '.') && !candidate.is_empty() {
                scheme = candidate.to_ascii_lowercase();
                &s[pos + 1..]
            } else {
                s
            }
        } else {
            s
        };

        // fragment
        let (rest, fragment) = if let Some(pos) = rest.find('#') {
            (&rest[..pos], rest[pos + 1..].to_owned())
        } else {
            (rest, String::new())
        };

        // query
        let (rest, query) = if let Some(pos) = rest.find('?') {
            (&rest[..pos], rest[pos + 1..].to_owned())
        } else {
            (rest, String::new())
        };

        // authority + path
        if rest.starts_with("//") {
            let after_slashes = &rest[2..];
            if let Some(pos) = after_slashes.find('/') {
                authority = after_slashes[..pos].to_owned();
                path = after_slashes[pos..].to_owned();
            } else {
                authority = after_slashes.to_owned();
            }
        } else {
            path = rest.to_owned();
        }

        // TS schemeFix: no scheme → 'file'
        if scheme.is_empty() {
            scheme = "file".to_owned();
        }

        Ok(Self { scheme, authority, path, query, fragment })
    }

    /// Create a `file://` URI from a filesystem path.
    pub fn from_file_path(fspath: &str) -> Self {
        let path = if fspath.starts_with('/') {
            fspath.to_owned()
        } else {
            format!("/{fspath}")
        };
        Self { scheme: "file".to_owned(), authority: String::new(), path, query: String::new(), fragment: String::new() }
    }

    /// Returns the platform-specific file-system path (authority + path for UNC).
    pub fn fs_path(&self) -> String {
        if !self.authority.is_empty() {
            format!("//{}/{}", self.authority, self.path.trim_start_matches('/'))
        } else {
            self.path.clone()
        }
    }

    /// Create a new URI with some components replaced (None = keep current; empty string = clear).
    pub fn with(
        &self,
        scheme:    Option<&str>,
        authority: Option<&str>,
        path:      Option<&str>,
        query:     Option<&str>,
        fragment:  Option<&str>,
    ) -> Self {
        Self {
            scheme:    scheme   .unwrap_or(&self.scheme)   .to_owned(),
            authority: authority.unwrap_or(&self.authority).to_owned(),
            path:      path     .unwrap_or(&self.path)     .to_owned(),
            query:     query    .unwrap_or(&self.query)    .to_owned(),
            fragment:  fragment .unwrap_or(&self.fragment) .to_owned(),
        }
    }

    /// Serialize back to a string (RFC 3986).
    pub fn to_string_encoded(&self) -> String {
        let mut out = String::new();
        if !self.scheme.is_empty() {
            out.push_str(&self.scheme);
            out.push(':');
        }
        if !self.authority.is_empty() {
            out.push_str("//");
            out.push_str(&self.authority);
        }
        out.push_str(&self.path);
        if !self.query.is_empty() {
            out.push('?');
            out.push_str(&self.query);
        }
        if !self.fragment.is_empty() {
            out.push('#');
            out.push_str(&self.fragment);
        }
        out
    }
}

impl fmt::Display for VsUri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string_encoded())
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Error
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, thiserror::Error)]
pub enum UriError {
    #[error("URI parse error: {0}")]
    ParseError(String),
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // Test 1 — roundtrip: parse then format.
    #[test]
    fn parse_and_format_http() {
        let raw = "https://example.com:8080/some/path?q=1&r=2#sec";
        let uri = VsUri::parse(raw).unwrap();
        assert_eq!(uri.scheme, "https");
        assert_eq!(uri.authority, "example.com:8080");
        assert_eq!(uri.path, "/some/path");
        assert_eq!(uri.query, "q=1&r=2");
        assert_eq!(uri.fragment, "sec");
        let roundtrip = uri.to_string_encoded();
        assert_eq!(roundtrip, raw);
    }

    // Test 2 — vscode-specific custom scheme (not handled by `url` crate).
    #[test]
    fn parse_vscode_file_scheme() {
        let raw = "vscode-file://vscode-app/path/to/resource";
        let uri = VsUri::parse(raw).unwrap();
        assert_eq!(uri.scheme, "vscode-file");
        assert_eq!(uri.authority, "vscode-app");
        assert_eq!(uri.path, "/path/to/resource");
        assert_eq!(uri.to_string_encoded(), raw);
    }

    // Test 3 — from_file_path + fs_path roundtrip.
    #[test]
    fn file_path_roundtrip() {
        let fspath = "/home/user/project/file.ts";
        let uri = VsUri::from_file_path(fspath);
        assert_eq!(uri.scheme, "file");
        assert_eq!(uri.fs_path(), fspath);
    }

    // Test 4 — `with` produces new URI with selective overrides.
    #[test]
    fn with_selective_override() {
        let base = VsUri::parse("https://example.com/foo?bar#baz").unwrap();
        let modified = base.with(None, None, Some("/new-path"), Some(""), None);
        assert_eq!(modified.scheme, "https");
        assert_eq!(modified.path, "/new-path");
        assert_eq!(modified.query, "");         // cleared
        assert_eq!(modified.fragment, "baz");   // unchanged
    }
}
