use tauri::http::{header, Request, Response, StatusCode, Uri};

const VSCODE_FILE_SCHEME: &str = "vscode-file";
const VSCODE_WEBVIEW_SCHEME: &str = "vscode-webview";
const PLACEHOLDER_BODY: &str = "VS Code Atomic custom protocol placeholder";

#[derive(Debug, PartialEq, Eq)]
pub enum ProtocolValidationError {
    InvalidScheme,
    InvalidAuthority,
    InvalidPath,
    InvalidNonce,
}

pub fn register_custom_protocols(
    builder: tauri::Builder<tauri::Wry>,
) -> tauri::Builder<tauri::Wry> {
    builder
        .register_uri_scheme_protocol(VSCODE_FILE_SCHEME, |_ctx, request| {
            handle_vscode_file_request(request)
        })
        .register_uri_scheme_protocol(VSCODE_WEBVIEW_SCHEME, |_ctx, request| {
            handle_vscode_webview_request(request)
        })
}

fn handle_vscode_file_request(request: Request<Vec<u8>>) -> Response<Vec<u8>> {
    match validate_vscode_file_uri(request.uri()) {
        Ok(()) => placeholder_response(StatusCode::NOT_FOUND, None),
        Err(_) => placeholder_response(StatusCode::FORBIDDEN, None),
    }
}

fn handle_vscode_webview_request(request: Request<Vec<u8>>) -> Response<Vec<u8>> {
    match validate_vscode_webview_uri(request.uri()) {
        Ok(nonce) => placeholder_response(StatusCode::NOT_FOUND, Some(nonce.as_str())),
        Err(_) => placeholder_response(StatusCode::FORBIDDEN, None),
    }
}

pub fn validate_vscode_file_uri(uri: &Uri) -> Result<(), ProtocolValidationError> {
    validate_scheme(uri, VSCODE_FILE_SCHEME)?;
    validate_file_authority(uri)?;
    validate_strict_path(uri.path())
}

pub fn validate_vscode_webview_uri(uri: &Uri) -> Result<String, ProtocolValidationError> {
    validate_scheme(uri, VSCODE_WEBVIEW_SCHEME)?;
    validate_webview_authority(uri)?;
    validate_strict_path(uri.path())?;
    validate_nonce(uri.query())
}

fn validate_scheme(uri: &Uri, expected_scheme: &str) -> Result<(), ProtocolValidationError> {
    match uri.scheme_str() {
        Some(scheme) if scheme.eq_ignore_ascii_case(expected_scheme) => Ok(()),
        _ => Err(ProtocolValidationError::InvalidScheme),
    }
}

fn validate_file_authority(uri: &Uri) -> Result<(), ProtocolValidationError> {
    match uri.host() {
        None | Some("") | Some("vscode-file") => Ok(()),
        _ => Err(ProtocolValidationError::InvalidAuthority),
    }
}

fn validate_webview_authority(uri: &Uri) -> Result<(), ProtocolValidationError> {
    let Some(host) = uri.host() else {
        return Err(ProtocolValidationError::InvalidAuthority);
    };

    if host.is_empty()
        || host.eq_ignore_ascii_case("localhost")
        || host.eq_ignore_ascii_case("127.0.0.1")
        || host.eq_ignore_ascii_case("[::1]")
    {
        return Err(ProtocolValidationError::InvalidAuthority);
    }

    if host
        .split('.')
        .all(|label| !label.is_empty() && label.bytes().all(is_dns_label_byte))
    {
        Ok(())
    } else {
        Err(ProtocolValidationError::InvalidAuthority)
    }
}

fn validate_strict_path(path: &str) -> Result<(), ProtocolValidationError> {
    if path.is_empty() || !path.starts_with('/') || path.contains('\\') || path.contains('\0') {
        return Err(ProtocolValidationError::InvalidPath);
    }

    let lower = path.to_ascii_lowercase();
    if lower.contains("%2e") || lower.contains("%2f") || lower.contains("%5c") {
        return Err(ProtocolValidationError::InvalidPath);
    }

    if path
        .split('/')
        .any(|segment| segment == ".." || segment == ".")
    {
        return Err(ProtocolValidationError::InvalidPath);
    }

    Ok(())
}

fn validate_nonce(query: Option<&str>) -> Result<String, ProtocolValidationError> {
    let Some(query) = query else {
        return Err(ProtocolValidationError::InvalidNonce);
    };

    for pair in query.split('&') {
        let mut parts = pair.splitn(2, '=');
        if parts.next() == Some("nonce") {
            let nonce = parts.next().unwrap_or_default();
            if nonce.len() >= 16 && nonce.bytes().all(is_nonce_byte) {
                return Ok(nonce.to_string());
            }
            return Err(ProtocolValidationError::InvalidNonce);
        }
    }

    Err(ProtocolValidationError::InvalidNonce)
}

fn is_dns_label_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'-'
}

fn is_nonce_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'-'
}

fn placeholder_response(status: StatusCode, nonce: Option<&str>) -> Response<Vec<u8>> {
    let csp = match nonce {
        Some(nonce) => format!(
            "default-src 'none'; script-src 'nonce-{nonce}'; style-src 'nonce-{nonce}'; img-src 'none'; connect-src 'none'; frame-ancestors 'none'; base-uri 'none'"
        ),
        None => "default-src 'none'; frame-ancestors 'none'; base-uri 'none'".to_string(),
    };

    Response::builder()
        .status(status)
        .header(header::CONTENT_TYPE, "text/plain; charset=utf-8")
        .header(header::CONTENT_SECURITY_POLICY, csp)
        .body(PLACEHOLDER_BODY.as_bytes().to_vec())
        .expect("failed to build custom protocol response")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn uri(value: &str) -> Uri {
        value.parse().expect("test URI should parse")
    }

    #[test]
    fn validates_vscode_file_safe_paths() {
        assert_eq!(
            validate_vscode_file_uri(&uri("vscode-file:///safe/path.txt")),
            Ok(())
        );
        assert_eq!(
            validate_vscode_file_uri(&uri("vscode-file://vscode-file/safe/path.txt")),
            Ok(())
        );
    }

    #[test]
    fn rejects_vscode_file_untrusted_authority_and_traversal() {
        assert_eq!(
            validate_vscode_file_uri(&uri("vscode-file://evil.example/safe/path.txt")),
            Err(ProtocolValidationError::InvalidAuthority)
        );
        assert_eq!(
            validate_vscode_file_uri(&uri("vscode-file:///safe/../secret.txt")),
            Err(ProtocolValidationError::InvalidPath)
        );
        assert_eq!(
            validate_vscode_file_uri(&uri("vscode-file:///safe/%2e%2e/secret.txt")),
            Err(ProtocolValidationError::InvalidPath)
        );
    }

    #[test]
    fn validates_vscode_webview_origin_path_and_nonce() {
        assert_eq!(
            validate_vscode_webview_uri(&uri(
                "vscode-webview://extension-host-1/index.html?nonce=abcDEF1234567890"
            )),
            Ok("abcDEF1234567890".to_string())
        );
    }

    #[test]
    fn rejects_vscode_webview_without_isolated_origin_or_nonce() {
        assert_eq!(
            validate_vscode_webview_uri(&uri(
                "vscode-webview://localhost/index.html?nonce=abcDEF1234567890"
            )),
            Err(ProtocolValidationError::InvalidAuthority)
        );
        assert_eq!(
            validate_vscode_webview_uri(&uri("vscode-webview://extension-host-1/index.html")),
            Err(ProtocolValidationError::InvalidNonce)
        );
        assert_eq!(
            validate_vscode_webview_uri(&uri(
                "vscode-webview://extension-host-1/index.html?nonce=short"
            )),
            Err(ProtocolValidationError::InvalidNonce)
        );
    }
}
