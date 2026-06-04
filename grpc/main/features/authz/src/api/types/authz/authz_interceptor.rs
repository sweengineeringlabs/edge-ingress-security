//! Struct declaration and constructors for [`AuthzInterceptor`].

use std::sync::Arc;

use crate::api::authz::AuthzPolicy;

/// Inbound interceptor that runs an [`AuthzPolicy`] against the caller's identity.
///
/// Push onto a [`GrpcIngressInterceptorChain`] after authentication interceptors
/// (mTLS or bearer) so that `identity` metadata is already populated when the
/// policy runs.
///
/// [`GrpcIngressInterceptorChain`]: swe_edge_ingress_grpc::GrpcIngressInterceptorChain
///
/// # Examples
///
/// ```rust,no_run
/// use swe_edge_ingress_grpc::PeerIdentity;
/// use swe_edge_ingress_grpc_authz::AuthzInterceptor;
///
/// // Closure policy — allow health checks unconditionally.
/// let interceptor = AuthzInterceptor::from_policy(
///     |_identity: &PeerIdentity, method: &str| method.ends_with("/Check"),
/// );
/// ```
pub struct AuthzInterceptor {
    pub(crate) policy: Arc<dyn AuthzPolicy>,
}

impl AuthzInterceptor {
    /// Construct from any policy.
    ///
    /// Accepts a struct implementing [`AuthzPolicy`] or any
    /// `Fn(&PeerIdentity, &str) -> bool + Send + Sync` closure.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use swe_edge_ingress_grpc::PeerIdentity;
    /// use swe_edge_ingress_grpc_authz::AuthzInterceptor;
    ///
    /// let i = AuthzInterceptor::from_policy(
    ///     |_: &PeerIdentity, _: &str| true // allow all (dev only)
    /// );
    /// ```
    pub fn from_policy<P: AuthzPolicy + 'static>(policy: P) -> Self {
        Self {
            policy: Arc::new(policy),
        }
    }
    /// Construct from an already-shared policy.
    ///
    /// Use this when multiple interceptors share the same policy instance.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use std::sync::Arc;
    /// use swe_edge_ingress_grpc::PeerIdentity;
    /// use swe_edge_ingress_grpc_authz::{AuthzInterceptor, AuthzPolicy};
    ///
    /// struct DenyAll;
    /// impl AuthzPolicy for DenyAll {
    ///     fn allows(&self, _: &PeerIdentity, _: &str) -> bool { false }
    /// }
    ///
    /// let policy = Arc::new(DenyAll);
    /// let i = AuthzInterceptor::from_shared_policy(policy);
    /// ```
    pub fn from_shared_policy(policy: Arc<dyn AuthzPolicy>) -> Self {
        Self { policy }
    }
}
