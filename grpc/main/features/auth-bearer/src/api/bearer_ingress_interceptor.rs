//! Struct declaration and constructors for [`BearerIngressInterceptor`].

use crate::api::BearerIngressConfig;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{BearerIngressConfig, BearerSecret};

    fn test_cfg() -> BearerIngressConfig {
        BearerIngressConfig {
            secret: BearerSecret::Hs256 {
                secret: b"key".to_vec(),
            },
            expected_issuer: "iss".into(),
            expected_audience: "aud".into(),
            leeway_seconds: 0,
        }
    }

    /// @covers: from_config
    #[test]
    fn test_from_config_creates_interceptor() {
        let _ = BearerIngressInterceptor::from_config(test_cfg());
    }
}
