//! SAF layer — verifier public facade.

mod verifier_svc;

pub use crate::api::error::VerifierError;
pub use crate::api::types::{
    ApiKeyVerifier, Claims, ClaimsBuilder, JwtConfig, JwtKey, JwtVerifier, NoopVerifierExtension,
    VerifierSvc,
};
