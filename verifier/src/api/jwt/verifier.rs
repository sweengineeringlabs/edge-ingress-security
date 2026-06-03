//! `JwtVerifier` interface — counterpart to `core/jwt/verifier.rs`.
//!
//! `core/jwt/verifier.rs` implements [`TokenVerifier`] for [`JwtVerifier`];
//! this mirror exposes both through the SAF.

pub use crate::api::traits::TokenVerifier;
pub use crate::api::types::jwt::JwtVerifier;
