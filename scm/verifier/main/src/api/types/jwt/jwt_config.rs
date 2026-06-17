//! `JwtConfig` — configuration for `JwtVerifier`.

use serde::{Deserialize, Serialize};

use super::JwtKey;

/// Configuration for [`JwtVerifier`](crate::JwtVerifier).
///
/// Deserializes directly from TOML. `required_issuer` and `required_audience`
/// are optional — omit them to skip those claim checks. `leeway_seconds`
/// allows a clock skew tolerance (default 0).
///
/// # Examples
///
/// ```rust
/// use swe_edge_ingress_verifier::{JwtConfig, JwtKey};
///
/// let config = JwtConfig {
///     key: JwtKey::Hs256 { secret: b"my-secret".to_vec() },
///     required_issuer: Some("https://auth.example.com".to_string()),
///     required_audience: Some("my-service".to_string()),
///     leeway_seconds: 5,
/// };
///
/// assert_eq!(config.leeway_seconds, 5);
/// assert!(config.required_issuer.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtConfig {
    /// Key material and algorithm.
    pub key: JwtKey,
    /// Required `iss` claim value.  `None` skips issuer validation.
    #[serde(default)]
    pub required_issuer: Option<String>,
    /// Required `aud` claim value.  `None` skips audience validation.
    #[serde(default)]
    pub required_audience: Option<String>,
    /// Maximum clock skew in seconds when validating `exp`/`nbf`.
    #[serde(default)]
    pub leeway_seconds: u64,
}
