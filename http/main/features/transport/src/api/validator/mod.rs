//! Validator interface counterpart — trait definitions for config validation.
pub(crate) mod http;
pub(crate) mod http_config_validator;

pub use http_config_validator::HttpConfigValidator;
