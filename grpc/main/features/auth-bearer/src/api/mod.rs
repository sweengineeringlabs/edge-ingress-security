//! API layer — config, error, and contracts for the inbound bearer interceptor.

pub(crate) mod bearer;
pub mod error;
pub mod traits;
pub mod types;

pub use bearer::{
    BearerIngressConfig, BearerIngressInterceptor, BearerSecret, AUTHORIZATION_HEADER,
    EXTRACTED_BEARER_SUBJECT,
};
pub use error::BearerAuthError;
