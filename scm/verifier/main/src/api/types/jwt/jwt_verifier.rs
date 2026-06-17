//! `JwtVerifier` — HS256/RS256/ES256 bearer-token verifier.

/// JWT verifier supporting HS256, RS256, and ES256.
///
/// Validates signature, `exp`, `nbf`, and optionally `iss` and `aud`.
/// Construct via [`JwtVerifier::from_config`]. The verifier implements
/// [`TokenVerifier`](crate::TokenVerifier) so it can be boxed and stored
/// as `Arc<dyn TokenVerifier>` for use with `BearerTokenInterceptor`
/// from `swe-edge-ingress-grpc-verifier`.
///
/// # Examples
///
/// ```rust,no_run
/// use swe_edge_ingress_verifier::{JwtConfig, JwtKey, JwtVerifier, TokenVerifier};
///
/// let config = JwtConfig {
///     key: JwtKey::Hs256 { secret: b"super-secret-key-32-bytes-long!!".to_vec() },
///     required_issuer: Some("https://auth.example.com".to_string()),
///     required_audience: None,
///     leeway_seconds: 0,
/// };
///
/// let verifier = JwtVerifier::from_config(&config).expect("valid key material");
/// // In tests, use a real signed token from your identity provider.
/// // verifier.verify("eyJ...").unwrap();
/// ```
pub struct JwtVerifier {
    pub(crate) key: jsonwebtoken::DecodingKey,
    pub(crate) validation: jsonwebtoken::Validation,
}
