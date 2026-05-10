//! `BearerService` — tower service that wraps an inner service with bearer-token auth.

use std::sync::Arc;

use swe_edge_ingress_verifier::TokenVerifier;

/// Tower service wrapping an inner service `S` with bearer-token authentication.
///
/// Produced by [`BearerLayer`](super::bearer_layer::BearerLayer); do not construct directly.
/// On each request the `Authorization: Bearer <token>` header is extracted and verified;
/// auth failures become `401 Unauthorized` responses without reaching the inner service.
#[derive(Clone)]
pub struct BearerService<S> {
    pub(crate) inner:    S,
    pub(crate) verifier: Arc<dyn TokenVerifier>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use swe_edge_ingress_verifier::{Claims, VerifierError};

    struct AlwaysOk;
    impl TokenVerifier for AlwaysOk {
        fn verify(&self, _: &str) -> Result<Claims, VerifierError> {
            Ok(serde_json::from_str(r#"{"sub":"t"}"#).unwrap())
        }
    }

    /// @covers: BearerService
    #[test]
    fn test_bearer_service_stores_verifier_and_inner() {
        let svc = BearerService {
            inner:    (),
            verifier: Arc::new(AlwaysOk) as Arc<dyn TokenVerifier>,
        };
        assert!(svc.verifier.verify("any").is_ok());
    }
}
