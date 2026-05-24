//! API layer — config, error, and contracts for the inbound bearer interceptor.

pub(crate) mod bearer;
pub(crate) mod processor;
pub(crate) mod traits;
pub mod types;

pub use bearer::{
    BearerAuthError, BearerIngressConfig, BearerIngressInterceptor, BearerSecret,
    AUTHORIZATION_HEADER, EXTRACTED_BEARER_SUBJECT,
};
