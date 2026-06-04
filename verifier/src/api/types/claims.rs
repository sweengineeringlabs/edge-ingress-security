//! `Claims` — the verified payload extracted from a JWT.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Verified JWT claims surfaced after successful token verification.
///
/// Standard registered claims are typed; any non-standard claims are
/// collected in `custom` as raw JSON values. Use [`Claims::builder`] to
/// construct a value in tests; in production you receive this from
/// [`TokenVerifier::verify`](crate::TokenVerifier::verify).
///
/// # Examples
///
/// ```rust
/// use swe_edge_ingress_verifier::Claims;
///
/// let claims = Claims::builder()
///     .sub("user-123")
///     .iss("https://auth.example.com")
///     .build();
///
/// assert_eq!(claims.sub.as_deref(), Some("user-123"));
/// assert_eq!(claims.iss.as_deref(), Some("https://auth.example.com"));
/// assert!(claims.get("role").is_none());
/// ```
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_ingress_verifier::Claims;
    /// let claims = Claims::builder().build();
    /// assert!(claims.get("role").is_none());
    /// ```
    pub fn get(&self, key: &str) -> Option<&serde_json::Value> {
        self.custom.get(key)
    }

    /// Start building a [`Claims`] value with fluent setters.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_ingress_verifier::Claims;
    /// let claims = Claims::builder().sub("alice").iss("https://idp.example.com").build();
    /// assert_eq!(claims.sub.as_deref(), Some("alice"));
    /// ```
    pub fn builder() -> super::claims_builder::ClaimsBuilder {
        super::claims_builder::ClaimsBuilder::default()
    }
}
