//! Marker trait for authorization interceptors.

use crate::api::traits::GrpcIngressInterceptor;

/// Marker trait for an inbound interceptor that gates dispatch on
/// authorization.
///
/// Implementors **must** also override
/// [`GrpcIngressInterceptor::is_authorization`] to return `true` so
/// the default-deny startup check (in [`crate::TonicGrpcServer`]) can
/// detect that an authz gate is wired in.
pub trait AuthorizationInterceptor: GrpcIngressInterceptor {}
