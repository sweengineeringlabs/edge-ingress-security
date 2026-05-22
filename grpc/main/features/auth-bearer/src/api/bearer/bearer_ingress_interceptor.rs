//! Struct declaration and constructor for [`BearerIngressInterceptor`].

use super::bearer_ingress_config::BearerIngressConfig;

/// [`GrpcIngressInterceptor`](swe_edge_ingress_grpc::GrpcIngressInterceptor)
/// that validates incoming JWT bearer tokens.
pub struct BearerIngressInterceptor {
    pub(crate) config: BearerIngressConfig,
}

impl BearerIngressInterceptor {
    /// Construct from config.
    pub fn from_config(config: BearerIngressConfig) -> Self {
        Self { config }
    }
}
