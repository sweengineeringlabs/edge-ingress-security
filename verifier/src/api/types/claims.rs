//! `Claims` — the verified payload extracted from a JWT.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Verified JWT claims surfaced after successful token verification.
///
/// Standard registered claims are typed; any non-standard claims are
/// collected in `custom` as raw JSON values.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// `sub` — subject (who the token is about).
    #[serde(default)]
    pub sub: Option<String>,
    /// `iss` — issuer.
    #[serde(default)]
    pub iss: Option<String>,
    /// `aud` — audience (string or array of strings in the wild).
    #[serde(default)]
    pub aud: Option<serde_json::Value>,
    /// `exp` — expiry (Unix timestamp).
    #[serde(default)]
    pub exp: Option<u64>,
    /// `nbf` — not-before (Unix timestamp).
    #[serde(default)]
    pub nbf: Option<u64>,
    /// `iat` — issued-at (Unix timestamp).
    #[serde(default)]
    pub iat: Option<u64>,
    /// `jti` — JWT ID.
    #[serde(default)]
    pub jti: Option<String>,
    /// Non-standard claims (e.g. `role`, `scope`, `email`).
    #[serde(flatten)]
    pub custom: HashMap<String, serde_json::Value>,
}

impl Claims {
    /// Return the value of a custom claim by key.
    pub fn get(&self, key: &str) -> Option<&serde_json::Value> {
        self.custom.get(key)
    }
}
