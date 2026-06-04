//! `BearerTokenInterceptor` — gRPC inbound auth gate.

use std::sync::Arc;

use swe_edge_ingress_verifier::TokenVerifier;

/// A gRPC inbound interceptor that authenticates calls via a `Bearer` token
/// in the `authorization` metadata key.
///
/// Wire up by calling `.push(Arc::new(BearerTokenInterceptor::new(verifier)))` on
/// a [`GrpcIngressInterceptorChain`](swe_edge_ingress_grpc_transport::GrpcIngressInterceptorChain).
/// The verified `sub` claim is forwarded under `x-edge-extracted-bearer-subject`
/// for downstream authz policies.
///
/// # Examples
///
/// ```rust,no_run
/// use std::sync::Arc;
/// use swe_edge_ingress_grpc_transport::GrpcIngressInterceptorChain;
/// use swe_edge_ingress_grpc_verifier::BearerTokenInterceptor;
/// use swe_edge_ingress_verifier::{JwtConfig, JwtKey, JwtVerifier};
///
/// let config = JwtConfig {
///     key: JwtKey::Hs256 { secret: b"32-byte-secret-for-hs256-here!!!".to_vec() },
///     required_issuer: Some("https://auth.example.com".to_string()),
///     required_audience: None,
///     leeway_seconds: 0,
/// };
/// let verifier = JwtVerifier::from_config(&config).expect("valid config");
/// let interceptor = BearerTokenInterceptor::new(Arc::new(verifier));
///
/// let chain = GrpcIngressInterceptorChain::new().push(Arc::new(interceptor));
/// assert_eq!(chain.len(), 1);
/// assert!(chain.contains_authorization());
/// ```
pub struct BearerTokenInterceptor {
    pub(crate) verifier: Arc<dyn TokenVerifier>,
}

impl BearerTokenInterceptor {
    /// Construct from any [`TokenVerifier`].
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use std::sync::Arc;
    /// use swe_edge_ingress_grpc_verifier::BearerTokenInterceptor;
    /// use swe_edge_ingress_verifier::{JwtConfig, JwtKey, JwtVerifier};
    ///
    /// let cfg = JwtConfig {
    ///     key: JwtKey::Hs256 { secret: b"32-byte-secret-for-hs256-here!!!".to_vec() },
    ///     required_issuer: None,
    ///     required_audience: None,
    ///     leeway_seconds: 0,
    /// };
    /// let verifier = Arc::new(JwtVerifier::from_config(&cfg).unwrap());
    /// let interceptor = BearerTokenInterceptor::new(verifier);
    /// ```
    pub fn new(verifier: Arc<dyn TokenVerifier>) -> Self {
        Self { verifier }
    }
}
