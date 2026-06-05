//! Inbound interceptor trait.

use crate::api::error::GrpcIngressError;
use crate::api::value::{GrpcRequest, GrpcResponse};

/// Inbound interceptor that runs before and after every dispatched request.
///
/// Interceptors are registered on a [`crate::GrpcIngressInterceptorChain`] and
/// run in insertion order. Any interceptor may short-circuit the chain by
/// returning `Err`.
pub trait GrpcIngressInterceptor: Send + Sync {
    /// Runs before the request is forwarded to the handler.
    fn before_dispatch(&self, req: &mut GrpcRequest) -> Result<(), GrpcIngressError>;
    /// Runs after the handler response is ready.
    fn after_dispatch(&self, resp: &mut GrpcResponse) -> Result<(), GrpcIngressError>;
    /// Returns `true` if this interceptor is an authorization gate.
    fn is_authorization(&self) -> bool {
        false
    }
}
