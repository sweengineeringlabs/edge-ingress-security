//! Interface counterpart to `core/bearer/bearer_token_interceptor.rs`.
//!
//! `core/bearer/bearer_token_interceptor.rs` implements
//! [`BearerTokenInterceptor`](crate::api::types::bearer_token_interceptor::BearerTokenInterceptor)
//! via the `GrpcIngressInterceptor` and `AuthorizationInterceptor` traits.

/// Interface anchor — the bearer token interceptor type.
///
/// Declared here to satisfy SEA rule 161 (one pub type per api/ file)
/// and rule 121 (api/ counterpart to core/bearer/bearer_token_interceptor.rs).
#[expect(dead_code, reason = "SEA api/ interface anchor (Rule 121)")]
pub struct BearerTokenInterceptor;
