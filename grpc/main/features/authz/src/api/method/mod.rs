//! Method ACL API — config and policy struct.

pub(crate) mod method_acl_config;
pub(crate) mod method_acl_policy;

pub use crate::api::types::authz::MethodAclPolicy;
pub use method_acl_config::MethodAclConfig;
