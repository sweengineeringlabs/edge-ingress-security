//! Inbound interceptor: validates `authorization: Bearer <jwt>`,
//! republishes the verified `sub` claim under
//! [`crate::EXTRACTED_BEARER_SUBJECT`].

use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use swe_edge_ingress_grpc::{
    GrpcIngressError, GrpcIngressInterceptor, GrpcRequest, GrpcResponse, GrpcStatusCode,
};

use crate::api::traits::Processor;
use crate::api::{
    bearer::bearer_secret::BearerSecret, BearerAuthError, BearerIngressInterceptor,
    AUTHORIZATION_HEADER, EXTRACTED_BEARER_SUBJECT,
};
use crate::core::bearer::jwt::claims::JwtClaims;

impl Processor for BearerIngressInterceptor {
    fn describe(&self) -> &'static str {
        const LABEL: &str = "grpc-auth-bearer";
        LABEL
    }
}

impl BearerIngressInterceptor {
    fn validate(&self, header_value: &str) -> Result<JwtClaims, BearerAuthError> {
        let token = header_value
            .strip_prefix("Bearer ")
            .or_else(|| header_value.strip_prefix("bearer "))
            .ok_or(BearerAuthError::MalformedHeader)?
            .trim();
        if token.is_empty() {
            return Err(BearerAuthError::MalformedHeader);
        }

        let (alg, key) = match &self.config.secret {
            BearerSecret::Hs256 { secret } => (Algorithm::HS256, DecodingKey::from_secret(secret)),
            BearerSecret::Rs256 { public_pem } => (
                Algorithm::RS256,
                DecodingKey::from_rsa_pem(public_pem).map_err(BearerAuthError::InvalidToken)?,
            ),
        };

        let mut validation = Validation::new(alg);
        validation.set_audience(&[self.config.expected_audience.as_str()]);
        validation.set_issuer(&[self.config.expected_issuer.as_str()]);
        validation.leeway = self.config.leeway_seconds;
        decode::<JwtClaims>(token, &key, &validation)
            .map(|d| d.claims)
            .map_err(BearerAuthError::InvalidToken)
    }
}

impl GrpcIngressInterceptor for BearerIngressInterceptor {
    fn before_dispatch(&self, req: &mut GrpcRequest) -> Result<(), GrpcIngressError> {
        // Strip any incoming subject key — only this interceptor is
        // allowed to set it, and only after successful verification.
        req.metadata.headers.remove(EXTRACTED_BEARER_SUBJECT);

        let header = req
            .metadata
            .headers
            .get(AUTHORIZATION_HEADER)
            .cloned()
            .ok_or_else(|| {
                GrpcIngressError::Status(
                    GrpcStatusCode::Unauthenticated,
                    "missing authorization header".into(),
                )
            })?;

        match self.validate(&header) {
            Ok(claims) => {
                req.metadata
                    .headers
                    .insert(EXTRACTED_BEARER_SUBJECT.to_string(), claims.sub);
                Ok(())
            }
            Err(BearerAuthError::MalformedHeader) | Err(BearerAuthError::MissingHeader) => {
                Err(GrpcIngressError::Status(
                    GrpcStatusCode::Unauthenticated,
                    "malformed authorization header".into(),
                ))
            }
            Err(BearerAuthError::InvalidToken(e)) => {
                tracing::warn!(error = %e, "rejecting invalid bearer token");
                Err(GrpcIngressError::Status(
                    GrpcStatusCode::Unauthenticated,
                    "invalid bearer token".into(),
                ))
            }
        }
    }

    fn after_dispatch(&self, _resp: &mut GrpcResponse) -> Result<(), GrpcIngressError> {
        Ok(())
    }
}
