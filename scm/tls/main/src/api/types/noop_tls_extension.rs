//! `NoopTlsExtension` — default no-op extension placeholder.

/// Default no-op extension. Satisfies any [`TlsExtension`](crate::api::traits::TlsExtension)
/// bound without altering behaviour.
pub struct NoopTlsExtension;
