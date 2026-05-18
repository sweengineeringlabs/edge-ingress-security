//! Validator interface — re-exports the Validator trait for core/ implementations.

pub(crate) mod grpc_server_config;

#[allow(unused_imports)]
pub use crate::api::traits::Validator;
