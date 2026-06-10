//! [`TenantError`] — errors produced by the tenant resolver layer.

use thiserror::Error;

/// Error variants for tenant resolution and configuration.
#[derive(Debug, Error)]
pub enum TenantError {
    /// The provided TOML configuration is invalid.
    #[error("invalid tenant resolver configuration: {0}")]
    InvalidConfig(String),
}
