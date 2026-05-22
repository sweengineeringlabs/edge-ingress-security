//! SAF service functions for the mTLS auth interceptor.

use crate::api::traits::{Processor, Validator};

/// Returns `true` when the mTLS interceptor is an authorization gate.
pub fn is_authorization_interceptor() -> bool {
    true
}

/// Returns `true` when the provided interceptor satisfies the [`Processor`]
/// contract — i.e., it is both `Send` and `Sync`.
pub fn is_processor<T: Processor>(_interceptor: &T) -> bool {
    true
}

/// Returns `true` when the provided value satisfies the [`Validator`]
/// contract.
pub fn is_validator<T: Validator>(_value: &T) -> bool {
    true
}
