//! Error type for authz rejections.

/// Reasons the authz interceptor rejects a call.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum AuthzError {
    /// No verified identity was present in the request metadata.
    /// This typically indicates the authz interceptor was wired
    /// in front of an mTLS / bearer interceptor instead of behind
    /// it — a configuration error.
    #[error("no verified identity present — authz must run after authn")]
    NoIdentity,

    /// The configured policy denied the call.  The string is a
    /// caller-safe sanitised reason; details are logged at WARN.
    #[error("authorization denied")]
    Denied,
}
