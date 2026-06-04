//! `GrpcIngressInterceptor` + `AuthorizationInterceptor` impl for `BearerTokenInterceptor`.

use swe_edge_ingress_grpc_transport::{
    AuthorizationInterceptor, GrpcIngressError, GrpcIngressInterceptor, GrpcRequest, GrpcResponse,
    GrpcStatusCode,
};
use swe_edge_ingress_verifier::VerifierError;

use crate::api::types::BearerTokenInterceptor;

impl GrpcIngressInterceptor for BearerTokenInterceptor {
    fn before_dispatch(&self, req: &mut GrpcRequest) -> Result<(), GrpcIngressError> {
        let raw = req
            .metadata
            .headers
            .get("authorization")
            .or_else(|| req.metadata.headers.get("Authorization"))
            .map(String::as_str)
            .ok_or_else(|| {
                GrpcIngressError::Status(
                    GrpcStatusCode::Unauthenticated,
                    "missing authorization metadata".into(),
                )
            })?;

        let token = raw.strip_prefix("Bearer ").ok_or_else(|| {
            GrpcIngressError::Status(
                GrpcStatusCode::Unauthenticated,
                "authorization must be 'Bearer <token>'".into(),
            )
        })?;

        self.verifier.verify(token).map(|_| ()).map_err(|e| {
            let msg = match &e {
                VerifierError::Expired => "token has expired".into(),
                VerifierError::NotYetValid => "token is not yet valid".into(),
                VerifierError::UnknownApiKey => "unknown API key".into(),
                VerifierError::ClaimMismatch(c) => format!("claim mismatch: {c}"),
                VerifierError::Invalid(_) | VerifierError::Config(_) => "invalid token".into(),
            };
            GrpcIngressError::Status(GrpcStatusCode::Unauthenticated, msg)
        })
    }

    fn after_dispatch(&self, _resp: &mut GrpcResponse) -> Result<(), GrpcIngressError> {
        Ok(())
    }

    fn is_authorization(&self) -> bool {
        true
    }
}

impl AuthorizationInterceptor for BearerTokenInterceptor {}

impl crate::api::traits::Processor for BearerTokenInterceptor {
    fn describe(&self) -> &'static str {
        const LABEL: &str = "grpc-verifier";
        LABEL
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::time::Duration;

    use swe_edge_ingress_grpc_transport::{GrpcIngressInterceptor, GrpcRequest, GrpcStatusCode};
    use swe_edge_ingress_verifier::{Claims, VerifierError};

    use crate::api::types::BearerTokenInterceptor;

    struct BearerTokenInterceptorAlwaysOk;
    impl swe_edge_ingress_verifier::TokenVerifier for BearerTokenInterceptorAlwaysOk {
        fn verify(&self, _: &str) -> Result<Claims, VerifierError> {
            Ok(serde_json::from_str(r#"{"sub":"grpc-user"}"#).unwrap())
        }
    }

    struct BearerTokenInterceptorAlwaysFail;
    impl swe_edge_ingress_verifier::TokenVerifier for BearerTokenInterceptorAlwaysFail {
        fn verify(&self, _: &str) -> Result<Claims, VerifierError> {
            Err(VerifierError::Expired)
        }
    }

    fn req_with_bearer(token: &str) -> GrpcRequest {
        let mut r = GrpcRequest::new("svc/M", vec![], Duration::from_secs(1));
        r.metadata
            .headers
            .insert("authorization".into(), format!("Bearer {token}"));
        r
    }

    /// @covers: before_dispatch
    #[test]
    fn test_before_dispatch_valid_token_returns_ok() {
        let i = BearerTokenInterceptor::new(Arc::new(BearerTokenInterceptorAlwaysOk));
        let mut req = req_with_bearer("good-token");
        assert!(i.before_dispatch(&mut req).is_ok());
    }

    /// @covers: before_dispatch
    #[test]
    fn test_before_dispatch_missing_metadata_returns_unauthenticated() {
        let i = BearerTokenInterceptor::new(Arc::new(BearerTokenInterceptorAlwaysOk));
        let mut req = GrpcRequest::new("svc/M", vec![], Duration::from_secs(1));
        let err = i.before_dispatch(&mut req).unwrap_err();
        assert!(matches!(
            err,
            swe_edge_ingress_grpc_transport::GrpcIngressError::Status(
                GrpcStatusCode::Unauthenticated,
                _
            )
        ));
    }

    /// @covers: before_dispatch
    #[test]
    fn test_before_dispatch_malformed_prefix_returns_unauthenticated() {
        let i = BearerTokenInterceptor::new(Arc::new(BearerTokenInterceptorAlwaysOk));
        let mut req = GrpcRequest::new("svc/M", vec![], Duration::from_secs(1));
        req.metadata
            .headers
            .insert("authorization".into(), "Basic abc".into());
        let err = i.before_dispatch(&mut req).unwrap_err();
        assert!(matches!(
            err,
            swe_edge_ingress_grpc_transport::GrpcIngressError::Status(
                GrpcStatusCode::Unauthenticated,
                _
            )
        ));
    }

    /// @covers: before_dispatch
    #[test]
    fn test_before_dispatch_expired_token_returns_unauthenticated() {
        let i = BearerTokenInterceptor::new(Arc::new(BearerTokenInterceptorAlwaysFail));
        let mut req = req_with_bearer("expired-token");
        let err = i.before_dispatch(&mut req).unwrap_err();
        assert!(matches!(
            err,
            swe_edge_ingress_grpc_transport::GrpcIngressError::Status(
                GrpcStatusCode::Unauthenticated,
                _
            )
        ));
    }

    /// @covers: is_authorization
    #[test]
    fn test_is_authorization_returns_true() {
        let i = BearerTokenInterceptor::new(Arc::new(BearerTokenInterceptorAlwaysOk));
        assert!(i.is_authorization());
    }
}
