//! Error returned by TonicGrpcServer::serve.

use crate::api::server::grpc::grpc_server_config_error::GrpcServerConfigError;

/// Error returned by [`super::tonic_grpc_server::TonicGrpcServer::serve`].
#[derive(Debug, thiserror::Error)]
pub enum TonicServerError {
    /// Failed to bind the server socket.
    #[error("failed to bind to {0}: {1}")]
    Bind(String, #[source] std::io::Error),
    /// TLS acceptor construction failed.
    #[error("TLS: {0}")]
    Tls(#[source] swe_edge_ingress_tls::IngressTlsError),
    /// Server configuration was rejected.
    #[error("server config rejected: {0}")]
    Config(#[source] GrpcServerConfigError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tonic_server_error_bind_formats_correctly() {
        let err = TonicServerError::Bind(
            "127.0.0.1:443".into(),
            std::io::Error::new(std::io::ErrorKind::AddrInUse, "port in use"),
        );
        let msg = err.to_string();
        assert!(msg.contains("127.0.0.1:443"));
    }
}
