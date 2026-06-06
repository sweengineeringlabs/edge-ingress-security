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
