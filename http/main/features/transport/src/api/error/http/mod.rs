//! HTTP-specific error types.
pub mod http_dispatcher_error;
pub mod http_ingress_error;
pub use http_dispatcher_error::HttpDispatcherError;
pub use http_ingress_error::HttpIngressError;
