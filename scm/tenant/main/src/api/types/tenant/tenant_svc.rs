//! [`TenantSvc`] — namespace for tenant resolver service utilities.

/// Namespace type for tenant resolver service factory utilities.
///
/// Use [`TenantSvc::create_config_builder`] to bootstrap TOML config loading,
/// and [`TenantSvc::validate`] to run config validation.
pub struct TenantSvc;
