//! Error type for TLS configuration loading.

/// Errors that can occur while loading or building a server TLS configuration.
///
/// Returned by [`TlsSvc::build_tls_acceptor`](crate::TlsSvc::build_tls_acceptor).
/// The path that failed is always included in the message so operators can
/// identify which file is broken without reading source.
///
/// # Examples
///
/// ```rust
/// use swe_edge_ingress_tls::IngressTlsError;
///
/// let err = IngressTlsError::CertParse("no certificates found in PEM".to_string());
/// assert!(err.to_string().contains("no certificates"));
///
/// // Match on the variant to apply different recovery strategies.
/// match err {
///     IngressTlsError::CertParse(msg) => eprintln!("cert parse failed: {msg}"),
///     _ => {}
/// }
/// ```
#[derive(Debug, thiserror::Error)]
pub enum IngressTlsError {
    /// Certificate file could not be opened.
    #[error("failed to load certificate from {0}: {1}")]
    CertLoad(String, #[source] std::io::Error),

    /// Certificate PEM data could not be parsed, or the file contained no
    /// certificates.
    #[error("failed to parse certificate: {0}")]
    CertParse(String),

    /// Private key file could not be opened.
    #[error("failed to load private key from {0}: {1}")]
    KeyLoad(String, #[source] std::io::Error),

    /// Private key PEM data could not be parsed, or the file contained no
    /// key.
    #[error("failed to parse private key: {0}")]
    KeyParse(String),

    /// The TLS server configuration could not be built from the provided
    /// certificate and key materials.
    #[error("TLS configuration error: {0}")]
    Config(String),
}
