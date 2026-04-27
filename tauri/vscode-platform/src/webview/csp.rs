//! CSP nonce generation and policy construction.

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use uuid::Uuid;

/// Extra sources to append to specific directives.
#[derive(Debug, Default, Clone)]
pub struct CspExtras {
    pub img_src: Vec<String>,
    pub script_src: Vec<String>,
    pub style_src: Vec<String>,
}

/// Generate a cryptographically-random nonce string (UUID v4 bytes, base64url, 22 chars).
pub fn generate_nonce() -> String {
    let bytes = Uuid::new_v4().as_bytes().to_vec();
    URL_SAFE_NO_PAD.encode(&bytes)
}

/// Build a Content-Security-Policy header value.
///
/// Default policy mirrors videoPreview.ts:
/// `default-src 'none'; img-src data: {csp_source}; media-src {csp_source};
///  script-src 'nonce-{nonce}'; style-src {csp_source} 'nonce-{nonce}'`
pub fn build_csp(csp_source: &str, nonce: &str, extras: &CspExtras) -> String {
    let img_extra = if extras.img_src.is_empty() {
        String::new()
    } else {
        format!(" {}", extras.img_src.join(" "))
    };
    let script_extra = if extras.script_src.is_empty() {
        String::new()
    } else {
        format!(" {}", extras.script_src.join(" "))
    };
    let style_extra = if extras.style_src.is_empty() {
        String::new()
    } else {
        format!(" {}", extras.style_src.join(" "))
    };

    format!(
        "default-src 'none'; img-src data: {src}{img_extra}; media-src {src}; \
         script-src 'nonce-{nonce}'{script_extra}; \
         style-src {src} 'nonce-{nonce}'{style_extra}",
        src = csp_source,
        nonce = nonce,
        img_extra = img_extra,
        script_extra = script_extra,
        style_extra = style_extra,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn generate_nonce_is_at_least_22_chars() {
        let n = generate_nonce();
        assert!(n.len() >= 22, "nonce length {} < 22: {:?}", n.len(), n);
    }

    #[test]
    fn generate_nonce_produces_unique_values() {
        let nonces: HashSet<String> = (0..100).map(|_| generate_nonce()).collect();
        assert_eq!(nonces.len(), 100, "nonces not unique");
    }

    #[test]
    fn build_csp_embeds_nonce_twice() {
        let csp = build_csp("https://example.com", "TESTNONCE", &CspExtras::default());
        let count = csp.matches("TESTNONCE").count();
        assert_eq!(count, 2, "nonce appears {} times, expected 2: {}", count, csp);
    }

    #[test]
    fn build_csp_embeds_source_at_least_twice() {
        let src = "https://example.com";
        let csp = build_csp(src, "nonce123", &CspExtras::default());
        let count = csp.matches(src).count();
        assert!(count >= 2, "source appears {} times, expected >=2: {}", count, csp);
    }

    #[test]
    fn build_csp_has_correct_default_directives() {
        let csp = build_csp("https://cdn.example.com", "abc123", &CspExtras::default());
        assert!(csp.starts_with("default-src 'none'"), "missing default-src: {}", csp);
        assert!(csp.contains("img-src data: https://cdn.example.com"), "missing img-src: {}", csp);
        assert!(csp.contains("media-src https://cdn.example.com"), "missing media-src: {}", csp);
        assert!(csp.contains("script-src 'nonce-abc123'"), "missing script-src: {}", csp);
        assert!(
            csp.contains("style-src https://cdn.example.com 'nonce-abc123'"),
            "missing style-src: {}",
            csp
        );
    }

    #[test]
    fn build_csp_appends_extras() {
        let extras = CspExtras {
            img_src: vec!["blob:".to_string()],
            script_src: vec!["'unsafe-eval'".to_string()],
            style_src: vec!["'unsafe-inline'".to_string()],
        };
        let csp = build_csp("https://src", "n", &extras);
        assert!(csp.contains("img-src data: https://src blob:"), "img extras: {}", csp);
        assert!(csp.contains("script-src 'nonce-n' 'unsafe-eval'"), "script extras: {}", csp);
        assert!(
            csp.contains("style-src https://src 'nonce-n' 'unsafe-inline'"),
            "style extras: {}",
            csp
        );
    }
}
