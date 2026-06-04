//! Struct declaration and constructors for [`AuthzInterceptor`].

use std::sync::Arc;

use crate::api::authz::AuthzPolicy;

/// Inbound interceptor that runs an [`AuthzPolicy`] against the caller's identity.
pub struct AuthzInterceptor {
    pub(crate) policy: Arc<dyn AuthzPolicy>,
}

impl AuthzInterceptor {
    /// Construct from any policy.
    pub fn from_policy<P: AuthzPolicy + 'static>(policy: P) -> Self {
        Self {
            policy: Arc::new(policy),
        }
    }
    /// Construct from an already-shared policy.
    pub fn from_shared_policy(policy: Arc<dyn AuthzPolicy>) -> Self {
        Self { policy }
    }
}
