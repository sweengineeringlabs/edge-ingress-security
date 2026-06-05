//! Configuration error for [`crate::TonicGrpcServer`].

/// Error returned when [`crate::TonicGrpcServer`] configuration is invalid.
#[derive(Debug, thiserror::Error)]
pub enum GrpcServerConfigError {
    /// `tls_required` is set but no TLS configuration was supplied.
    #[error(
        "tls_required is set but no TLS configuration supplied — \
         call `.with_tls(cfg)` before `.serve()`"
    )]
    TlsRequiredButMissing,
}
