# ADR-001: Ingress HTTP Security — Local Contract

**Status:** Accepted  
**Date:** 2026-06-12  
**Governing ADR:** [ADR-015](../../../../../docs/3-architecture/adr/ADR-015-security-layer-architecture.md) (rules R1–R7, shared surface, cascade)

---

## Mandate

Receive, verify, and route inbound HTTP identity. Never produce credentials or attach auth headers.

---

## What Lives Here

| Item | Crate | Why not shared |
|------|-------|---------------|
| `JwtVerifier` | verifier | Implements shared `TokenVerifier`; algorithm config is ingress-specific |
| `ApiKeyVerifier` | verifier | Implements shared `TokenVerifier`; constant-time server-side check |
| `JwtConfig` / `JwtKey` | verifier | Algorithm + key config for `JwtVerifier` only |
| `IngressClaims` | verifier | Wraps shared `Claims` + `custom: HashMap` for non-standard JWT fields |
| `VerifierError` | verifier | Detailed HTTP verification failures |
| `TenantResolver` trait | tenant | `fn resolve(&HeaderMap) -> Option<TenantId>`; HTTP headers only |
| `TenantResolverConfig` | tenant | noop/header/subdomain/jwt-claim strategies |
| `IngressTlsConfig` | tls | Server PEM paths + optional client CA; wraps `TlsConfigError` from shared |

## What Is Re-exported from Shared

| Re-export | Source |
|-----------|--------|
| `TokenVerifier` | `swe_edge_security::TokenVerifier` |
| `TenantId` | `swe_edge_security::TenantId` |
| `IngressTlsError` (alias) | `swe_edge_security::TlsConfigError` |

## `IngressClaims` vs `Claims`

`IngressClaims` wraps `swe_edge_security::Claims` and adds `custom: HashMap<String, serde_json::Value>` for non-standard JWT fields. Handlers that only need standard fields accept `&Claims`; handlers that need custom fields accept `&IngressClaims`. `IngressClaims::base()` downcasts to the shared type.

This separation is intentional — the `HashMap` couples to `serde_json` which must not enter the shared layer.
