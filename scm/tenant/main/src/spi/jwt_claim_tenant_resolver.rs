//! [`JwtClaimTenantResolver`] — extracts tenant ID from a configurable JWT claim.

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use http::HeaderMap;

use crate::api::traits::TenantResolver;
use crate::api::types::TenantId;

/// Extracts a [`TenantId`] by decoding the JWT payload (without signature
/// verification) and reading a configurable claim field.
///
/// The token is expected in the `Authorization: Bearer <token>` header.
/// Signature validation is intentionally omitted — this resolver is for
/// *routing* only; actual authentication is the verifier crate's responsibility.
#[derive(Debug)]
pub(crate) struct JwtClaimTenantResolver {
    claim: String,
}

impl JwtClaimTenantResolver {
    pub(crate) fn new(claim: impl Into<String>) -> Self {
        Self {
            claim: claim.into(),
        }
    }

    fn extract_tenant(token: &str, claim: &str) -> Option<TenantId> {
        // JWT format: header.payload.signature
        let payload_b64 = token.split('.').nth(1)?;
        let bytes = URL_SAFE_NO_PAD.decode(payload_b64).ok()?;
        let payload: serde_json::Value = serde_json::from_slice(&bytes).ok()?;
        let value = payload.get(claim)?.as_str()?;
        if value.is_empty() {
            return None;
        }
        Some(TenantId::new(value))
    }
}

impl TenantResolver for JwtClaimTenantResolver {
    fn resolve(&self, headers: &HeaderMap) -> Option<TenantId> {
        let auth = headers.get(http::header::AUTHORIZATION)?.to_str().ok()?;
        let token = auth.strip_prefix("Bearer ")?;
        Self::extract_tenant(token, &self.claim)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::Engine;

    fn make_jwt(payload_json: &str) -> String {
        let header = URL_SAFE_NO_PAD.encode(r#"{"alg":"none"}"#);
        let payload = URL_SAFE_NO_PAD.encode(payload_json);
        format!("{header}.{payload}.sig")
    }

    fn bearer_headers(token: &str) -> HeaderMap {
        let mut m = HeaderMap::new();
        m.insert(
            http::header::AUTHORIZATION,
            http::header::HeaderValue::from_str(&format!("Bearer {token}")).unwrap(),
        );
        m
    }

    #[test]
    fn test_jwt_claim_tenant_resolver_new_stores_claim_name() {
        let r = JwtClaimTenantResolver::new("org");
        assert_eq!(r.claim, "org");
    }

    #[test]
    fn test_jwt_claim_tenant_resolver_resolve_valid_claim_returns_tenant_id() {
        let r = JwtClaimTenantResolver::new("tenant_id");
        let jwt = make_jwt(r#"{"sub":"user1","tenant_id":"acme"}"#);
        assert_eq!(r.resolve(&bearer_headers(&jwt)).unwrap().as_str(), "acme");
    }

    #[test]
    fn test_jwt_claim_tenant_resolver_resolve_missing_claim_returns_none() {
        let r = JwtClaimTenantResolver::new("tenant_id");
        let jwt = make_jwt(r#"{"sub":"user1"}"#);
        assert!(r.resolve(&bearer_headers(&jwt)).is_none());
    }

    #[test]
    fn test_jwt_claim_tenant_resolver_resolve_no_auth_header_returns_none() {
        let r = JwtClaimTenantResolver::new("tenant_id");
        assert!(r.resolve(&HeaderMap::new()).is_none());
    }

    #[test]
    fn test_jwt_claim_tenant_resolver_resolve_non_bearer_auth_returns_none() {
        let r = JwtClaimTenantResolver::new("tenant_id");
        let mut headers = HeaderMap::new();
        headers.insert(
            http::header::AUTHORIZATION,
            http::header::HeaderValue::from_static("Basic dXNlcjpwYXNz"),
        );
        assert!(r.resolve(&headers).is_none());
    }

    #[test]
    fn test_jwt_claim_tenant_resolver_resolve_custom_claim_field_returns_tenant_id() {
        let r = JwtClaimTenantResolver::new("org");
        let jwt = make_jwt(r#"{"org":"globex","sub":"u2"}"#);
        assert_eq!(r.resolve(&bearer_headers(&jwt)).unwrap().as_str(), "globex");
    }
}
