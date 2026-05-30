//! API layer — verifier traits, config, and value objects.

pub(crate) mod error;
pub(crate) mod traits;
pub(crate) mod types;

pub(crate) use error::VerifierError;
pub(crate) use traits::{TokenVerifier, Validator};
pub(crate) use types::{
    ApiKeyVerifier, Claims, JwtConfig, JwtKey, JwtVerifier, NoopVerifierExtension, VerifierSvc,
};
pub(crate) mod jwt;
