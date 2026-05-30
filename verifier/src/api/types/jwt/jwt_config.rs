//! `JwtConfig` — configuration for `JwtVerifier`.

use serde::{Deserialize, Serialize};

use super::JwtKey;

/// Configuration for [`JwtVerifier`](crate::JwtVerifier).
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
