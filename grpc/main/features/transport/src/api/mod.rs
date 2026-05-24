//! API layer — gRPC domain types, port, and interceptors.

pub mod error;
pub mod types;

pub(crate) mod application_config;
pub(crate) mod audit_sink;
pub(crate) mod grpc_timeout;
pub(crate) mod handler;
pub(crate) mod handler_dispatch;
pub(crate) mod health_service;
pub(crate) mod interceptor;
pub(crate) mod peer_identity;
pub(crate) mod port;
pub(crate) mod server;
pub(crate) mod status_codes;
pub(crate) mod traits;
pub(crate) mod validator;
pub(crate) mod value;
