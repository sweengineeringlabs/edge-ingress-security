//! Configuration for the mTLS auth interceptor.

use serde::{Deserialize, Serialize};

/// Configuration for [`MtlsAuthInterceptor`](crate::MtlsAuthInterceptor).
///
/// `allowed_cns` and `allowed_san_dns` are matched as exact
/// case-insensitive strings — wildcard / glob support is intentionally
/// out of scope for v1 (a misconfigured wildcard is a more dangerous
/// vulnerability than a missing entry).  Empty allowlists are treated
/// as "any verified peer is OK".
///
/// # Examples
///
/// ```rust
/// use swe_edge_ingress_grpc_auth_mtls::MtlsAuthConfig;
///
/// // Accept any client that completed the mTLS handshake.
/// let cfg = MtlsAuthConfig::allow_any_verified_peer();
/// assert!(cfg.allowed_cns.is_empty());
///
/// // Restrict to specific client certificate CNs.
/// let cfg = MtlsAuthConfig::restrict_to_cns(vec![
///     "client-a.internal".to_string(),
///     "client-b.internal".to_string(),
/// ]);
/// assert_eq!(cfg.allowed_cns.len(), 2);
/// assert!(!cfg.allow_unauthenticated_methods);
/// ```
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_ingress_grpc_auth_mtls::MtlsAuthConfig;
    /// let cfg = MtlsAuthConfig::allow_any_verified_peer();
    /// assert!(cfg.allowed_cns.is_empty() && cfg.allowed_san_dns.is_empty());
    /// ```
    pub fn allow_any_verified_peer() -> Self {
        Self::default()
    }

    /// Restrict access to peers presenting one of `cns` as their CN.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_ingress_grpc_auth_mtls::MtlsAuthConfig;
    /// let cfg = MtlsAuthConfig::restrict_to_cns(vec!["svc-a.internal".to_string()]);
    /// assert_eq!(cfg.allowed_cns, vec!["svc-a.internal"]);
    /// ```
    pub fn restrict_to_cns(cns: impl IntoIterator<Item = String>) -> Self {
        Self {
            allowed_cns: cns.into_iter().collect(),
            ..Self::default()
        }
    }

    /// Load config from a TOML string.  Per-crate convention — values
    /// belong in `config/application.toml`, never in source.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_ingress_grpc_auth_mtls::MtlsAuthConfig;
    ///
    /// let toml = r#"
    ///     allowed_cns = ["svc-a.internal", "svc-b.internal"]
    /// "#;
    /// let cfg = MtlsAuthConfig::from_toml(toml).unwrap();
    /// assert_eq!(cfg.allowed_cns.len(), 2);
    /// ```
    pub fn from_toml(s: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(s)
    }
}
