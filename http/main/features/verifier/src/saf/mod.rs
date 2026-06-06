//! Public facade — re-exports from `api/`.
mod verifier_svc;

pub use crate::api::error::HttpAuthError;
pub use crate::api::error::VerifierError;
pub use crate::api::types::VerifierSvc;
pub use crate::api::vo::VerifiedClaims;
pub use crate::spi::bearer::axum::{BearerLayer, BearerService, BearerServiceHelper};
