//! Interface contract for HTTP config validation.
//!
//! This file is the api/ counterpart to `core::validator::http_config_validator`
//! per SEA Rule 121. It declares the marker trait that the core implementation
//! fulfills.

/// Marker trait for types that implement HTTP config validation.
///
/// The `HttpConfigValidator` in `core/` must implement this trait
/// to satisfy the SEA interface–implementation pairing.
pub trait HttpConfigValidatorContract: Send + Sync {}

#[cfg(test)]
mod tests {
    use super::*;

    struct Dummy;
    impl HttpConfigValidatorContract for Dummy {}

    #[test]
    fn test_http_config_validator_contract_is_object_safe() {
        fn _assert(_: &dyn HttpConfigValidatorContract) {}
    }
}
