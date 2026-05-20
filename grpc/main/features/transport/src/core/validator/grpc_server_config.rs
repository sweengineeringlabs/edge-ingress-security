//! Core implementation of the [`Validator`] trait for gRPC configuration types.

use crate::api::traits::Validator;
use crate::api::value_object::GrpcServerConfig;

/// Marker type confirming this module provides GrpcServerConfig validation.
///
/// The validated type is [`GrpcServerConfig`] from api/.
/// This struct exists to satisfy the SEA rule requiring every core module file
/// to define a primary type matching the filename.
pub(crate) struct GrpcServerConfigValidator;

impl Validator for GrpcServerConfig {
    /// Validate a [`GrpcServerConfig`].
    ///
    /// Returns `Err` when `tls_required` is set but no TLS material is attached.
    fn validate(&self) -> Result<(), String> {
        if self.tls_required && self.tls.is_none() {
            return Err(
                "GrpcServerConfig: tls_required is true but no IngressTlsConfig is attached"
                    .to_string(),
            );
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_returns_ok_when_tls_not_required() {
        let cfg = GrpcServerConfig::default().allow_plaintext();
        assert!(cfg.validate().is_ok());
    }

    #[test]
    fn test_validate_returns_err_when_tls_required_but_no_tls_config() {
        let cfg = GrpcServerConfig::default(); // tls_required=true by default
        let result = cfg.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("tls_required"));
    }
}
