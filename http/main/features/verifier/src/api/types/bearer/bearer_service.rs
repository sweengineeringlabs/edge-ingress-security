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
    pub(crate) inner: S,
    pub(crate) verifier: Arc<dyn TokenVerifier>,
}
