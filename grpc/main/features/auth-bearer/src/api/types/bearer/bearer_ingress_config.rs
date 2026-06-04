//! Inbound (server) bearer configuration.

use serde::{Deserialize, Serialize};

use super::bearer_secret::BearerSecret;

/// Inbound (server) bearer config.
///
/// Loaded from TOML under the `[bearer]` key by the SAF factory.
/// `expected_issuer` and `expected_audience` are enforced as exact matches
/// against the JWT `iss` and `aud` claims after signature verification.
///
/// # Examples
///
/// ```rust
/// use swe_edge_ingress_grpc_auth_bearer::{BearerIngressConfig, BearerSecret};
///
/// let config = BearerIngressConfig {
///     secret: BearerSecret::Hs256 { secret: b"my-32-byte-secret-key-here!!!!!".to_vec() },
///     expected_issuer: "https://auth.example.com".to_string(),
///     expected_audience: "my-grpc-service".to_string(),
///     leeway_seconds: 5,
/// };
///
/// assert_eq!(config.expected_issuer, "https://auth.example.com");
/// assert_eq!(config.leeway_seconds, 5);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BearerIngressConfig {
    /// Verification key material.
    pub secret: BearerSecret,
    /// Required `iss` value — tokens with a different issuer are rejected.
    pub expected_issuer: String,
    /// Required `aud` value — tokens with a different audience are rejected.
    pub expected_audience: String,
    /// Maximum acceptable clock skew when checking `exp`/`nbf`, in seconds.
    pub leeway_seconds: u64,
}
