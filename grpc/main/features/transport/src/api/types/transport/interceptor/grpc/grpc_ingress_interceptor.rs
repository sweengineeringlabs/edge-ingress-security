//! Inbound interceptor trait.

use crate::api::port::grpc::GrpcIngressError;
use crate::api::value::{GrpcRequest, GrpcResponse};

/// An interceptor for inbound gRPC calls.
pub trait GrpcIngressInterceptor: Send + Sync {
    /// Run before the handler is dispatched.  Returning `Err(_)`
    /// aborts the call — the handler is **not** invoked.
    fn before_dispatch(&self, req: &mut GrpcRequest) -> Result<(), GrpcIngressError>;

    /// Run after the handler returns successfully.  Returning
    /// `Err(_)` converts the call result into that error.
    fn after_dispatch(&self, resp: &mut GrpcResponse) -> Result<(), GrpcIngressError>;

    /// Whether this interceptor enforces authorisation.
    ///
    /// Implementations of [`AuthorizationInterceptor`] **must** override
    /// this to return `true` so the server-startup default-deny check
    /// can detect them.  The default `false` keeps non-authz
    /// interceptors out of the gate-discovery path.
    fn is_authorization(&self) -> bool {
        false
    }
}
