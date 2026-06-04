//! `AuthzPolicy` trait — the pluggable allow/deny rule.

use swe_edge_ingress_grpc::PeerIdentity;

/// A policy decision: allow the call or deny it with a reason.
///
/// The decision is intentionally simple — anything more nuanced
/// (e.g. quotas, soft-deny with downgrade) belongs in a separate
/// crate that wraps this one.
///
/// Any `Fn(&PeerIdentity, &str) -> bool + Send + Sync` closure automatically
/// implements this trait, so closures can be passed directly to
/// [`AuthzInterceptor::from_policy`](crate::AuthzInterceptor::from_policy)
/// without boilerplate.
///
/// # Examples
///
/// ```rust,no_run
/// use swe_edge_ingress_grpc_authz::{AuthzInterceptor, AuthzPolicy};
/// use swe_edge_ingress_grpc::PeerIdentity;
///
/// // Closure impl — no boilerplate needed.
/// let interceptor = AuthzInterceptor::from_policy(
///     |identity: &PeerIdentity, method: &str| {
///         // Allow health checks unconditionally; everything else requires a CN.
///         method == "/grpc.health.v1.Health/Check"
///             || identity.cn.is_some()
///     }
/// );
///
/// // Struct impl for complex rules.
/// struct DenyAll;
/// impl AuthzPolicy for DenyAll {
///     fn allows(&self, _identity: &PeerIdentity, _method: &str) -> bool { false }
/// }
/// ```
pub trait AuthzPolicy: Send + Sync {
    /// Returns `true` when `identity` is permitted to invoke `method`.
    ///
    /// `method` is the fully-qualified gRPC method path, e.g.
    /// `"/pkg.Service/Method"`.
    ///
    /// `identity` is the cryptographically-verified caller — the
    /// authz interceptor only invokes the policy after upstream
    /// auth interceptors (mTLS, bearer) have populated metadata.
    fn allows(&self, identity: &PeerIdentity, method: &str) -> bool;
}

impl<F> AuthzPolicy for F
where
    F: Fn(&PeerIdentity, &str) -> bool + Send + Sync,
{
    fn allows(&self, identity: &PeerIdentity, method: &str) -> bool {
        (self)(identity, method)
    }
}
