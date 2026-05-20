//! Error type returned by [`AxumHttpServer::serve`].

/// Error returned by [`AxumHttpServer::serve`].
#[derive(Debug, thiserror::Error)]
pub enum AxumServerError {
    /// Failed to bind the server socket.
    #[error("failed to bind to {0}: {1}")]
    Bind(String, #[source] std::io::Error),
    /// Server encountered an I/O error while serving.
    #[error("server error: {0}")]
    Serve(#[source] std::io::Error),
    /// TLS acceptor construction failed.
    #[error("TLS: {0}")]
    Tls(#[source] swe_edge_ingress_tls::IngressTlsError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_axum_server_error_bind_formats_with_address() {
        let e = AxumServerError::Bind(
            "0.0.0.0:8080".into(),
            std::io::Error::new(std::io::ErrorKind::AddrInUse, "in use"),
        );
        assert!(e.to_string().contains("0.0.0.0:8080"), "{e}");
    }

    #[test]
    fn test_axum_server_error_serve_formats_correctly() {
        let e = AxumServerError::Serve(std::io::Error::new(
            std::io::ErrorKind::BrokenPipe,
            "broken",
        ));
        assert!(e.to_string().contains("server error"), "{e}");
    }
}
