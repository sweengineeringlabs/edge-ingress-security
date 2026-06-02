//! `VerifiedClaims` — axum request extension carrying authenticated JWT claims.

use swe_edge_ingress_verifier::Claims;

/// Newtype carrying verified JWT claims, inserted into axum request extensions
/// by the bearer authentication layer.
///
/// Extract with `axum::extract::Extension<VerifiedClaims>` inside a handler.
#[derive(Debug, Clone)]
pub struct VerifiedClaims(pub Claims);

impl VerifiedClaims {
    /// Access the inner [`Claims`].
    pub fn claims(&self) -> &Claims {
        &self.0
    }
}
