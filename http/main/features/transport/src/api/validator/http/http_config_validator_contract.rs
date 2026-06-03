//! Interface contract for HTTP config validation.
//!
//! This file is the api/ counterpart to `core::validator::http_config_validator`
//! per SEA Rule 121. It declares the marker trait that the core implementation
//! fulfills.

/// Marker trait for types that implement HTTP config validation.
///
/// The `HttpConfigValidator` in `core/` must implement this trait
/// to satisfy the SEA interface–implementation pairing.
#[expect(
    dead_code,
    reason = "SEA api/ interface anchor (Rule 121) — intentionally unused"
)]
pub trait HttpConfigValidatorContract: Send + Sync {}
