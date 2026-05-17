//! SAF layer — public facade.

pub use crate::api::{
    BearerAuthError, BearerInboundConfig, BearerInboundInterceptor, BearerSecret,
    AUTHORIZATION_HEADER, EXTRACTED_BEARER_SUBJECT,
};
