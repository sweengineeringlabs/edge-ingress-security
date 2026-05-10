//! `BearerLayer` — tower layer that injects JWT verification into an axum service stack.

use std::sync::Arc;
use swe_edge_ingress_verifier::TokenVerifier;

/// A tower [`Layer`](tower::Layer) that wraps an inner service with bearer-token
/// authentication.  On each request the `Authorization: Bearer <token>` header
/// is extracted and verified; the resulting [`VerifiedClaims`](super::verified_claims::VerifiedClaims)
/// are inserted into the request extensions.  Requests missing or failing
/// authentication receive an HTTP 401 response.
#[derive(Clone)]
pub struct BearerLayer {
    pub(crate) verifier: Arc<dyn TokenVerifier>,
}

impl BearerLayer {
    /// Construct from any [`TokenVerifier`] implementation.
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
            Ok(serde_json::from_str(r#"{"sub":"test"}"#).unwrap())
        }
    }

    /// @covers: BearerLayer::new — stores the verifier.
    #[test]
    fn test_bearer_layer_new_stores_verifier() {
        let layer = BearerLayer::new(Arc::new(AlwaysOk));
        // Arc pointer is non-null (layer was constructed without panic).
        let _ = layer.verifier.verify("x").unwrap();
    }
}
