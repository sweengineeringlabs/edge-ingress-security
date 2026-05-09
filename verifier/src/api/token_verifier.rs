//! `TokenVerifier` — protocol-agnostic JWT verification trait.

use crate::api::claims::Claims;
use crate::api::verifier_error::VerifierError;

/// Verifies an inbound bearer token string and returns its claims.
///
/// Implementations decide the algorithm (HS256, RS256, ES256), key
/// material, and which claims to enforce.  The trait is object-safe so
/// callers can hold `Arc<dyn TokenVerifier>`.
pub trait TokenVerifier: Send + Sync {
    /// Verify `token` and return the extracted [`Claims`] on success.
    fn verify(&self, token: &str) -> Result<Claims, VerifierError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct AlwaysOk;
    impl TokenVerifier for AlwaysOk {
        fn verify(&self, _: &str) -> Result<Claims, VerifierError> {
            Ok(serde_json::from_str(r#"{"sub":"test"}"#).unwrap())
        }
    }

    struct AlwaysFail;
    impl TokenVerifier for AlwaysFail {
        fn verify(&self, _: &str) -> Result<Claims, VerifierError> {
            Err(VerifierError::Invalid("always".into()))
        }
    }

    /// @covers: TokenVerifier — trait is object-safe.
    #[test]
    fn test_token_verifier_is_object_safe() {
        fn _assert(_: &dyn TokenVerifier) {}
    }

    /// @covers: TokenVerifier — ok implementation returns claims.
    #[test]
    fn test_token_verifier_ok_returns_claims() {
        assert!(AlwaysOk.verify("any").is_ok());
    }

    /// @covers: TokenVerifier — fail implementation returns error.
    #[test]
    fn test_token_verifier_fail_returns_error() {
        assert!(AlwaysFail.verify("any").is_err());
    }
}
