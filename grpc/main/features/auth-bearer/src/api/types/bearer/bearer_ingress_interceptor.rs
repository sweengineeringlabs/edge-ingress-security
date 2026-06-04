//! Struct declaration and constructor for [`BearerIngressInterceptor`].

use super::bearer_ingress_config::BearerIngressConfig;

/// [`GrpcIngressInterceptor`](swe_edge_ingress_grpc::GrpcIngressInterceptor)
/// that validates incoming JWT bearer tokens.
///
/// Push onto a [`GrpcIngressInterceptorChain`] so every gRPC call must
/// present a valid signed JWT in the `authorization: Bearer <token>` header.
/// The verified `sub` claim is forwarded as `x-edge-extracted-bearer-subject`
/// for downstream authz policies.
///
/// [`GrpcIngressInterceptorChain`]: swe_edge_ingress_grpc::GrpcIngressInterceptorChain
///
/// # Examples
///
/// ```rust,no_run
/// use swe_edge_ingress_grpc_auth_bearer::{BearerIngressConfig, BearerIngressInterceptor, BearerSecret};
///
/// let config = BearerIngressConfig {
///     secret: BearerSecret::Hs256 { secret: b"my-32-byte-secret-key-here!!!!!".to_vec() },
///     expected_issuer: "https://auth.example.com".to_string(),
///     expected_audience: "my-service".to_string(),
///     leeway_seconds: 0,
/// };
/// let interceptor = BearerIngressInterceptor::from_config(config);
/// // Add to: chain.push(Arc::new(interceptor))
/// ```
pub struct BearerIngressInterceptor {
    pub(crate) config: BearerIngressConfig,
}

impl BearerIngressInterceptor {
    /// Construct from config.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use swe_edge_ingress_grpc_auth_bearer::{BearerIngressConfig, BearerIngressInterceptor, BearerSecret};
    ///
    /// let interceptor = BearerIngressInterceptor::from_config(BearerIngressConfig {
    ///     secret: BearerSecret::Hs256 { secret: b"secret-key".to_vec() },
    ///     expected_issuer: "https://idp.example.com".to_string(),
    ///     expected_audience: "svc".to_string(),
    ///     leeway_seconds: 0,
    /// });
    /// ```
    pub fn from_config(config: BearerIngressConfig) -> Self {
        Self { config }
    }
}
