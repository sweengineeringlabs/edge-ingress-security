//! Error type returned by the mTLS interceptor when access is denied.

/// Reasons the mTLS interceptor rejects a call.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum MtlsAuthError {
    /// The request reached the interceptor without any peer-cert
    /// fingerprint set — typically a plaintext / TLS-only conn.
    #[error("no mTLS peer identity present in request metadata")]
    MissingIdentity,

    /// The peer's CN was not on the configured allowlist.
    #[error("peer CN '{0}' is not on the allowlist")]
    DisallowedCn(String),

    /// None of the peer's DNS SANs matched the configured allowlist.
    #[error("none of the peer's DNS SANs match the allowlist")]
    DisallowedSan,
}
