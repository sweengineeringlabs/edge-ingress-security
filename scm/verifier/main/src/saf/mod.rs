//! SAF layer — verifier public facade.

mod verifier_svc;

pub use crate::api::error::VerifierError;
pub use crate::api::traits::TokenVerifier;
pub use crate::api::types::{
    ApiKeyVerifier, ApplicationConfigBuilder, NoopVerifierExtension, VerifierSvc,
};
pub use crate::api::types::{Claims, ClaimsBuilder, JwtConfig, JwtKey};
pub use crate::spi::jwt::jsonwebtoken::JwtVerifier;
