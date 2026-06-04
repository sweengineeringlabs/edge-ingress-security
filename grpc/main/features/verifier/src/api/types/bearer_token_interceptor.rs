//! `BearerTokenInterceptor` — gRPC inbound auth gate.

use std::sync::Arc;

use swe_edge_ingress_verifier::TokenVerifier;

/// A gRPC inbound interceptor that authenticates calls via a `Bearer` token
/// in the `authorization` metadata key.
///
/// Wire up by calling `.push(Arc::new(BearerTokenInterceptor::new(verifier)))` on
/// a [`GrpcIngressInterceptorChain`](swe_edge_ingress_grpc_transport::GrpcIngressInterceptorChain).
pub struct BearerTokenInterceptor {
    pub(crate) verifier: Arc<dyn TokenVerifier>,
}

impl BearerTokenInterceptor {
    /// Construct from any [`TokenVerifier`].
    pub fn new(verifier: Arc<dyn TokenVerifier>) -> Self {
        Self { verifier }
    }
}
