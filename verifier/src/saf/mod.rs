//! SAF layer — verifier public facade.

mod verifier_svc;

pub use crate::api::error::VerifierError;
pub use crate::api::jwt::verifier::JwtVerifier;
pub use crate::api::jwt::verifier::TokenVerifier;
pub use crate::api::types::{
    ApiKeyVerifier, ApplicationConfigBuilder, Claims, ClaimsBuilder, JwtConfig, JwtKey,
    NoopVerifierExtension, VerifierSvc,
};
