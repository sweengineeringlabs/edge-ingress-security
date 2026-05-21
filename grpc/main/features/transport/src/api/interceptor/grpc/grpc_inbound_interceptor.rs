//! Inbound interceptor trait.

use crate::api::port::grpc_inbound::GrpcInboundError;
use crate::api::value_object::{GrpcRequest, GrpcResponse};

/// An interceptor for inbound gRPC calls.
pub trait GrpcInboundInterceptor: Send + Sync {
    /// Run before the handler is dispatched.  Returning `Err(_)`
    /// aborts the call — the handler is **not** invoked.
    fn before_dispatch(&self, req: &mut GrpcRequest) -> Result<(), GrpcInboundError>;

    /// Run after the handler returns successfully.  Returning
    /// `Err(_)` converts the call result into that error.
    fn after_dispatch(&self, resp: &mut GrpcResponse) -> Result<(), GrpcInboundError>;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_authorization_defaults_to_false_for_plain_interceptors() {
        struct Plain;
        impl GrpcInboundInterceptor for Plain {
            fn before_dispatch(&self, _: &mut GrpcRequest) -> Result<(), GrpcInboundError> {
                Ok(())
            }
            fn after_dispatch(&self, _: &mut GrpcResponse) -> Result<(), GrpcInboundError> {
                Ok(())
            }
        }
        assert!(!Plain.is_authorization());
    }

    #[test]
    fn test_grpc_inbound_interceptor_is_object_safe() {
        fn _assert(_: &dyn GrpcInboundInterceptor) {}
    }
}
