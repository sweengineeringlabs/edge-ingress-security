//! `BearerLayer` — tower layer that injects JWT verification into an axum service stack.

use std::sync::Arc;
use swe_edge_ingress_verifier::TokenVerifier;

/// A tower [`Layer`](tower::Layer) that wraps an inner service with bearer-token
/// authentication.  On each request the `Authorization: Bearer <token>` header
/// is extracted and verified; the resulting [`VerifiedClaims`](crate::api::vo::verified_claims::VerifiedClaims)
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
