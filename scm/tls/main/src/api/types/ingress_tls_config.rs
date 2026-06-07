//! Server-side TLS/mTLS configuration.

use serde::{Deserialize, Serialize};

/// Server-side TLS (and optionally mTLS) configuration for inbound servers.
///
/// Paths point to PEM-encoded files that are read eagerly when
/// [`TlsSvc::build_tls_acceptor`](crate::TlsSvc::build_tls_acceptor) is called — failures
/// surface at startup, not on the first connection.
///
/// # Examples
///
/// ```rust
/// use swe_edge_ingress_tls::IngressTlsConfig;
///
/// // TLS only — no client cert required.
/// let cfg = IngressTlsConfig::tls("certs/server.crt", "certs/server.key");
/// assert!(!cfg.is_mtls());
/// assert_eq!(cfg.cert_pem_path, "certs/server.crt");
///
/// // mTLS — clients must present a cert signed by the given CA.
/// let cfg = IngressTlsConfig::mtls("certs/server.crt", "certs/server.key", "certs/client-ca.crt");
/// assert!(cfg.is_mtls());
/// assert_eq!(cfg.client_ca_pem_path.as_deref(), Some("certs/client-ca.crt"));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngressTlsConfig {
    /// Path to a PEM file containing the server certificate chain (leaf first).
    pub cert_pem_path: String,

    /// Path to a PEM file containing the server's private key (RSA, EC, or
    /// Ed25519).
    pub key_pem_path: String,

    /// If set, enables mTLS: clients must present a certificate signed by the
    /// CA in this PEM file. When absent, client certificates are not required.
    pub client_ca_pem_path: Option<String>,
}

impl IngressTlsConfig {
    /// TLS-only: server authenticates with `cert`/`key`; client certificates
    /// are not required.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_ingress_tls::IngressTlsConfig;
    /// let cfg = IngressTlsConfig::tls("certs/server.crt", "certs/server.key");
    /// assert!(cfg.client_ca_pem_path.is_none());
    /// ```
    pub fn tls(cert_pem_path: impl Into<String>, key_pem_path: impl Into<String>) -> Self {
        Self {
            cert_pem_path: cert_pem_path.into(),
            key_pem_path: key_pem_path.into(),
            client_ca_pem_path: None,
        }
    }

    /// mTLS: server authenticates with `cert`/`key`; clients must present a
    /// certificate signed by `client_ca`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_ingress_tls::IngressTlsConfig;
    /// let cfg = IngressTlsConfig::mtls("certs/server.crt", "certs/server.key", "certs/ca.crt");
    /// assert!(cfg.is_mtls());
    /// assert_eq!(cfg.client_ca_pem_path.as_deref(), Some("certs/ca.crt"));
    /// ```
    pub fn mtls(
        cert_pem_path: impl Into<String>,
        key_pem_path: impl Into<String>,
        client_ca_pem_path: impl Into<String>,
    ) -> Self {
        Self {
            cert_pem_path: cert_pem_path.into(),
            key_pem_path: key_pem_path.into(),
            client_ca_pem_path: Some(client_ca_pem_path.into()),
        }
    }

    /// Whether client certificate verification is required.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_ingress_tls::IngressTlsConfig;
    /// assert!(!IngressTlsConfig::tls("c.crt", "k.key").is_mtls());
    /// assert!(IngressTlsConfig::mtls("c.crt", "k.key", "ca.crt").is_mtls());
    /// ```
    pub fn is_mtls(&self) -> bool {
        self.client_ca_pem_path.is_some()
    }
}
