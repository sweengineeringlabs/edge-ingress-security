//! API layer — gRPC domain types, port, and interceptors.
pub(crate) mod application_config;
pub(crate) mod architecture_config_builder;
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
pub(crate) mod value_object;
