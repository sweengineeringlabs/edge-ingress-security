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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_authorization_defaults_to_false_for_plain_interceptors() {
        struct Plain;
        impl GrpcIngressInterceptor for Plain {
            fn before_dispatch(&self, _: &mut GrpcRequest) -> Result<(), GrpcIngressError> {
                Ok(())
            }
            fn after_dispatch(&self, _: &mut GrpcResponse) -> Result<(), GrpcIngressError> {
                Ok(())
            }
        }
        assert!(!Plain.is_authorization());
    }

    #[test]
    fn test_grpc_ingress_interceptor_is_object_safe() {
        fn _assert(_: &dyn GrpcIngressInterceptor) {}
    }
}
