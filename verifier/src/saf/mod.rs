//! SAF layer — verifier public facade.

mod verifier_svc;

pub use crate::api::error::VerifierError;
pub use crate::api::types::{
    ApiKeyVerifier, Claims, JwtConfig, JwtKey, JwtVerifier, NoopVerifierExtension, VerifierSvc,
};
