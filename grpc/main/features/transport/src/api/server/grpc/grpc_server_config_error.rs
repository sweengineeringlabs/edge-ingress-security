//! Error returned by TonicGrpcServer::from_config.

/// Error returned by [`super::tonic_grpc_server::TonicGrpcServer::from_config`].
#[derive(Debug, thiserror::Error)]
pub enum GrpcServerConfigError {
    #[error(
        "tls_required is set but no TLS configuration supplied — \
         attach an IngressTlsConfig via with_tls(...) or call \
         allow_plaintext() to opt out"
    )]
    /// `tls_required` is set but no `IngressTlsConfig` was attached.
    TlsRequiredButMissing,
}
