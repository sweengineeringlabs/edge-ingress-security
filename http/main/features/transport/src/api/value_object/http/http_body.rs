//! HTTP request body type.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::form_part::FormPart;

/// HTTP request body types.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum HttpBody {
    /// JSON body.
    Json(serde_json::Value),
    /// Raw bytes body.
    Raw(Vec<u8>),
    /// URL-encoded form body.
    Form(HashMap<String, String>),
    /// Multipart form body.
    Multipart(Vec<FormPart>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_body_json_variant_holds_value() {
        let v = serde_json::json!({"key": "val"});
        let body = HttpBody::Json(v.clone());
        assert!(matches!(body, HttpBody::Json(_)));
    }

    #[test]
    fn test_http_body_raw_variant_holds_bytes() {
        let body = HttpBody::Raw(vec![1, 2, 3]);
        assert!(matches!(body, HttpBody::Raw(ref b) if b == &[1, 2, 3]));
    }
}
