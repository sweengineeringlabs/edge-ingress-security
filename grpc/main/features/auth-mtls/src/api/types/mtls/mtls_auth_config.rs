//! Configuration for the mTLS auth interceptor.

use serde::{Deserialize, Serialize};

/// Configuration for [`MtlsAuthInterceptor`](crate::MtlsAuthInterceptor).
///
/// `allowed_cns` and `allowed_san_dns` are matched as exact
/// case-insensitive strings — wildcard / glob support is intentionally
/// out of scope for v1 (a misconfigured wildcard is a more dangerous
/// vulnerability than a missing entry).  Empty allowlists are treated
/// as "any verified peer is OK".
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MtlsAuthConfig {
    /// When non-empty, the peer's CN MUST match one of these.  When
    /// empty, CN is not checked.
    #[serde(default)]
    pub allowed_cns: Vec<String>,
    /// When non-empty, AT LEAST ONE of the peer's DNS SANs MUST
    /// match one of these.  When empty, SANs are not checked.
    #[serde(default)]
    pub allowed_san_dns: Vec<String>,
    /// When `true`, methods listed in `unauthenticated_methods` skip
    /// the identity check entirely.  Default `false`.
    #[serde(default)]
    pub allow_unauthenticated_methods: bool,
    /// Method paths (e.g. `"/grpc.health.v1.Health/Check"`) that
    /// bypass the identity check.  Only consulted when
    /// `allow_unauthenticated_methods` is `true`.
    #[serde(default)]
    pub unauthenticated_methods: Vec<String>,
}

impl MtlsAuthConfig {
    /// Construct an empty config — any verified peer is accepted, no
    /// methods bypass auth.
    pub fn allow_any_verified_peer() -> Self {
        Self::default()
    }

    /// Restrict access to peers presenting one of `cns` as their CN.
    pub fn restrict_to_cns(cns: impl IntoIterator<Item = String>) -> Self {
        Self {
            allowed_cns: cns.into_iter().collect(),
            ..Self::default()
        }
    }

    /// Load config from a TOML string.  Per-crate convention — values
    /// belong in `config/application.toml`, never in source.
    pub fn from_toml(s: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(s)
    }
}
