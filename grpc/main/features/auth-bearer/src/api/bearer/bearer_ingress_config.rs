//! Inbound (server) bearer configuration.

use serde::{Deserialize, Serialize};

use super::bearer_secret::BearerSecret;

/// Inbound (server) bearer config.
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
