//! `VerifiedClaims` — request-scoped value carrying authenticated JWT claims.

use swe_edge_ingress_verifier::Claims;

/// Newtype carrying verified JWT claims, inserted into request extensions by
/// the bearer authentication layer for downstream handlers to read.
#[derive(Debug, Clone)]
pub struct VerifiedClaims(pub Claims);

impl VerifiedClaims {
    /// Access the inner [`Claims`].
    pub fn claims(&self) -> &Claims {
        &self.0
    }
}
