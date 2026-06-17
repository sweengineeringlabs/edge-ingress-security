# ADR-002: Security Context Propagation — ingress/security cascade

**Status:** Accepted  
**Date:** 2026-06-12  
**Governing ADR:** [ADR-017](https://github.com/sweengineeringlabs/edge/blob/main/docs/3-architecture/adr/ADR-017-security-context-propagation.md) — Security Context Propagation Pipeline  
**See also:** [ADR-001](ADR-001-ingress-security-specialisations.md) — ingress HTTP security specialisations

---

## Mandate

Cascade-bump `swe-edge-security` dependency when `Principal` moves to domain and `CredentialResolver` gains a context parameter. No source code changes required in this crate.

---

## What changes

### Dependency bump only

```toml
# Bump to the tag that carries the ADR-017 breaking changes
swe-edge-security = { git = "https://github.com/sweengineeringlabs/edge-security", tag = "v0.3.0" }
```

### `TenantId` re-export is unaffected

`TenantId` continues to be re-exported from `swe-edge-security`, which in turn re-exports from `edge-domain-security`. No source change needed.

---

## What does not change

`JwtVerifier`, `ApiKeyVerifier`, `TenantResolver`, `IngressClaims`, `IngressTlsConfig` — no changes. This crate does not implement `CredentialResolver` and does not use `RequestContext`.

---

## Cascade position

Step 5b of 11 (parallel with egress/http and egress/grpc). Blocked on: swe-edge-security ADR-001.
