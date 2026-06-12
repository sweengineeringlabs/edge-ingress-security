# ADR-001: Ingress Security Specialisations

**Status:** Accepted  
**Date:** 2026-06-12  
**Deciders:** phdsystems  
**Parent:** [ADR-015 — Three-Tier Security Layer Architecture](../../../../docs/3-architecture/adr/ADR-015-security-layer-architecture.md)  
**Affects:** `ingress/verifier`, `ingress/tls`, `ingress/tenant`

---

## Context

This ADR defines which security contracts are ingress-specific and why they must not be promoted to `swe-edge-security`. It also captures the adaptations required when the shared layer is extended per ADR-015.

Ingress security has one directional mandate:

> Receive an inbound identity claim, verify its authenticity, and surface a typed identity to domain handlers.

Everything in this workspace either serves that purpose or is out of scope.

---

## Ingress-Specific Concepts (must stay here)

### `JwtVerifier` and `ApiKeyVerifier`

These are **implementations** of the shared `TokenVerifier` trait. They know about JWT algorithms, key material, and API key constant-time comparison. These are ingress-specific because:

- Egress does not verify inbound tokens; it produces or attaches outbound credentials.
- The algorithm negotiation (HS256/RS256/ES256) and key configuration are inbound concerns.
- `ApiKeyVerifier` uses `subtle::ConstantTimeEq` — a server-side protection against timing attacks on incoming requests.

`JwtConfig` and `JwtKey` are configuration types for `JwtVerifier`. They live here for the same reason.

### `IngressClaims`

The standard JWT registered claims (`sub`, `iss`, `aud`, `exp`, `nbf`, `iat`, `jti`) are captured in shared `swe_edge_security::Claims`. Real-world JWTs routinely include non-standard claims (`role`, `scope`, `email`, `tenant_id`, `permissions`, …). The ingress verifier must surface these to domain handlers that need them.

`IngressClaims` wraps `swe_edge_security::Claims` and adds:
```rust
pub custom: HashMap<String, serde_json::Value>
```

**Why not put this in shared?**  
A `HashMap<String, serde_json::Value>` couples the shared primitive to `serde_json`, which is an ingress implementation concern. Egress never verifies tokens; it has no use for custom claims. Domain handlers that only need standard claims accept `&Claims`; handlers that also need custom claims accept `&IngressClaims`.

`IngressClaims` always provides `fn base(&self) -> &Claims` to allow downcasting to the shared type.

### `IngressTlsConfig` and `TlsAcceptor`

Server-side TLS uses `tokio_rustls::TlsAcceptor`. This is an inbound-only type — a server binds a certificate and key to accept incoming TLS connections. The egress equivalent is a client identity (`reqwest::Identity`); they are not interchangeable.

**After ADR-015 Step 2:** `IngressTlsError` is replaced by re-exporting `swe_edge_security::TlsConfigError`. The config struct (`IngressTlsConfig`) stays here.

### `TenantResolver`

Extracts `TenantId` from inbound HTTP headers using one of four strategies (noop, header, subdomain, jwt-claim). This is purely an inbound concept — egress never performs tenant routing from headers.

---

## Relationship to the Shared Layer

After ADR-015 is implemented:

| Shared item used | How ingress uses it |
|-----------------|---------------------|
| `TokenVerifier` trait | `JwtVerifier` + `ApiKeyVerifier` implement it |
| `Claims` | Wrapped by `IngressClaims`; base fields re-exposed via `fn base()` |
| `TenantId` | Re-exported from shared; produced by `TenantResolver` |
| `Principal` | Implemented by `TenantId` |
| `SecurityError` | `VerifierError` converts `Into<SecurityError>` (Verification variant); `TenantError` converts (Tenant variant) |
| `TlsConfigError` | `IngressTlsError` is a type alias for this |

---

## What Ingress Security Must Never Do

- Produce or attach outbound credentials (belongs in egress)
- Import from `egress/*` crates
- Own a `CredentialResolver` implementation (domain credentials are egress concerns)
- Mint tokens (ingress receives and verifies; it does not issue)

---

## Domain Boundary Contract

Domain handlers receive security context via well-typed arguments, never raw HTTP primitives:

| What handler receives | Source |
|----------------------|--------|
| `Option<TenantId>` | Resolved by `TenantResolver` in the ingress adapter |
| `Claims` | Verified by `TokenVerifier`; standard fields only |
| `IngressClaims` | Only when handler explicitly opts in to custom claim access |

Handlers must import from `swe_edge_security` or `swe_edge_ingress_verifier`, never from `swe_edge_ingress_tls` or internal crate paths.

---

## Implementation Notes (ADR-015 Step 2)

1. `TokenVerifier` in `api/traits/token_verifier.rs` → change to `pub use swe_edge_security::TokenVerifier`
2. Rename `Claims` → `IngressClaims` everywhere within this workspace
3. `IngressClaims::base(&self) -> &swe_edge_security::Claims` — new method
4. `IngressTlsError` → `pub type IngressTlsError = swe_edge_security::TlsConfigError`
5. `VerifierError` + `TenantError` → add `Into<swe_edge_security::SecurityError>` impls
6. Bump workspace to **v0.6.x** (Claims rename is breaking for consumers of `swe_edge_ingress_verifier::Claims`)
