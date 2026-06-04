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
