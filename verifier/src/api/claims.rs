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

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: Claims — round-trips through JSON.
    #[test]
    fn test_claims_round_trips_through_json() {
        let json = r#"{"sub":"alice","exp":9999999999,"role":"admin"}"#;
        let c: Claims = serde_json::from_str(json).expect("deserialize");
        assert_eq!(c.sub.as_deref(), Some("alice"));
        assert_eq!(c.exp, Some(9999999999));
        assert_eq!(c.get("role").and_then(|v| v.as_str()), Some("admin"));
    }

    /// @covers: Claims::get — returns None for absent key.
    #[test]
    fn test_get_returns_none_for_absent_claim() {
        let c: Claims = serde_json::from_str("{}").unwrap();
        assert!(c.get("missing").is_none());
    }
}
