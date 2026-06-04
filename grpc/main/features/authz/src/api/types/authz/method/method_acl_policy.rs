//! Struct declaration and constructors for [`MethodAclPolicy`].

use crate::api::method::MethodAclConfig;

/// Policy that consults a [`MethodAclConfig`] keyed on the caller's CN.
#[derive(Debug, Clone)]
pub struct MethodAclPolicy {
    pub(crate) config: MethodAclConfig,
}

impl MethodAclPolicy {
    /// Construct from an ACL config.
    pub fn from_config(config: MethodAclConfig) -> Self {
        Self { config }
    }
}
