//! Marker trait for authorization interceptors.

use super::grpc::grpc_ingress_interceptor::GrpcIngressInterceptor;

/// Marker trait for an inbound interceptor that gates dispatch on
/// authorization.
///
/// Implementors **must** also override
/// [`GrpcIngressInterceptor::is_authorization`] to return `true` so
/// the default-deny startup check (in [`crate::TonicGrpcServer`]) can
/// detect that an authz gate is wired in.
///
/// ## Why a marker trait?
///
/// At server startup we need to ask the chain "is at least one
/// interceptor an authz gate?" without dispatching a request.  A
/// marker trait on its own is not visible through `Arc<dyn
/// GrpcIngressInterceptor>`, so we pair it with the
/// `is_authorization()` method on the base trait — the marker is the
/// declarative contract, the method is the runtime detection hook.
pub trait AuthorizationInterceptor: GrpcIngressInterceptor {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::port::grpc_ingress::GrpcIngressError;
    use crate::api::value_object::{GrpcRequest, GrpcResponse};

    #[test]
    fn test_authorization_interceptor_is_a_supertrait_of_grpc_ingress_interceptor() {
        // Verify that a type implementing both traits compiles and is usable.
        struct AlwaysAllow;
        impl GrpcIngressInterceptor for AlwaysAllow {
            fn before_dispatch(&self, _: &mut GrpcRequest) -> Result<(), GrpcIngressError> {
                Ok(())
            }
            fn after_dispatch(&self, _: &mut GrpcResponse) -> Result<(), GrpcIngressError> {
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
