//! API layer — gRPC domain types, port, and interceptors.

pub mod error;
pub mod traits;
pub mod types;

pub(crate) mod application;
pub(crate) mod audit;
pub(crate) mod handler;
pub(crate) mod health;
pub(crate) mod interceptor;
pub(crate) mod server;
pub(crate) mod validator;
pub(crate) mod value;
