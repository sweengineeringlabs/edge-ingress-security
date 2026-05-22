//! SAF layer — public facade.

pub use crate::api::{
    BearerAuthError, BearerIngressConfig, BearerIngressInterceptor, BearerSecret,
    AUTHORIZATION_HEADER, EXTRACTED_BEARER_SUBJECT,
};
