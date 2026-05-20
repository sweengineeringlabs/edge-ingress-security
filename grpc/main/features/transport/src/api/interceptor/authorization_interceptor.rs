//! Marker trait for authorization interceptors.

use super::grpc::grpc_inbound_interceptor::GrpcInboundInterceptor;

/// Marker trait for an inbound interceptor that gates dispatch on
/// authorization.
///
/// Implementors **must** also override
/// [`GrpcInboundInterceptor::is_authorization`] to return `true` so
/// the default-deny startup check (in [`crate::TonicGrpcServer`]) can
/// detect that an authz gate is wired in.
///
/// ## Why a marker trait?
///
/// At server startup we need to ask the chain "is at least one
/// interceptor an authz gate?" without dispatching a request.  A
/// marker trait on its own is not visible through `Arc<dyn
/// GrpcInboundInterceptor>`, so we pair it with the
/// `is_authorization()` method on the base trait — the marker is the
/// declarative contract, the method is the runtime detection hook.
pub trait AuthorizationInterceptor: GrpcInboundInterceptor {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::port::grpc_inbound::GrpcInboundError;
    use crate::api::value_object::{GrpcRequest, GrpcResponse};

    #[test]
    fn test_authorization_interceptor_is_a_supertrait_of_grpc_inbound_interceptor() {
        // Verify that a type implementing both traits compiles and is usable.
        struct AlwaysAllow;
        impl GrpcInboundInterceptor for AlwaysAllow {
            fn before_dispatch(&self, _: &mut GrpcRequest) -> Result<(), GrpcInboundError> {
                Ok(())
            }
            fn after_dispatch(&self, _: &mut GrpcResponse) -> Result<(), GrpcInboundError> {
                Ok(())
            }
            fn is_authorization(&self) -> bool {
                true
            }
        }
        impl AuthorizationInterceptor for AlwaysAllow {}
        let _ = AlwaysAllow;
    }
}
