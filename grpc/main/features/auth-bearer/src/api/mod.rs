//! API layer — config, error, and contracts for the inbound bearer interceptor.

pub(crate) mod application_config_builder;
pub(crate) mod bearer;
pub(crate) mod processor;
pub(crate) mod traits;

pub use application_config_builder::ApplicationConfigBuilder;
pub use bearer::{
    BearerAuthError, BearerIngressConfig, BearerIngressInterceptor, BearerSecret,
    AUTHORIZATION_HEADER, EXTRACTED_BEARER_SUBJECT,
};
