//! SAF layer — public facade.

mod bearer_svc;

pub use crate::api::{
    ApplicationConfigBuilder, BearerAuthError, BearerIngressConfig, BearerIngressInterceptor,
    BearerSecret, AUTHORIZATION_HEADER, EXTRACTED_BEARER_SUBJECT,
};
pub use bearer_svc::{extracted_bearer_subject_key, validate_bearer_config};
