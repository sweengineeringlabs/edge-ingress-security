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

#[cfg(test)]
mod tests {
    use super::*;
    use swe_edge_ingress_verifier::{Claims, VerifierError};

    struct AlwaysOk;
    impl TokenVerifier for AlwaysOk {
        fn verify(&self, _: &str) -> Result<Claims, VerifierError> {
            Ok(serde_json::from_str(r#"{"sub":"grpc-user"}"#).unwrap())
        }
    }

    /// @covers: BearerTokenInterceptor::new — stores the verifier.
    #[test]
    fn test_bearer_token_interceptor_new_stores_verifier() {
        let i = BearerTokenInterceptor::new(Arc::new(AlwaysOk));
        assert!(i.verifier.verify("x").is_ok());
    }
}
