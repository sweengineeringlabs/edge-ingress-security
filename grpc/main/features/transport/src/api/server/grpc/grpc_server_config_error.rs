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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grpc_server_config_error_tls_required_has_descriptive_message() {
        let e = GrpcServerConfigError::TlsRequiredButMissing;
        assert!(e.to_string().contains("tls_required"));
    }
}
