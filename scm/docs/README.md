# swe-edge-ingress-security

## WHAT

Inbound token verification for swe-edge services — pluggable auth back-ends (JWT, API key, noop)
behind a single `TokenVerifier` trait.

Key capabilities:

- **`TokenVerifier`** — core trait: `verify(token: &str) → Result<Claims, VerifierError>`; object-safe; pluggable (JWT, API key, noop stub)
- **`JwtVerifier`** — HMAC-SHA256 / RS256 JWT implementation
- **`ApiKeyVerifier`** — static key-lookup for API-key authentication
- **`Claims`** / **`ClaimsBuilder`** — decoded token payload VO (`sub`, custom fields)
- **`JwtConfig`** / **`JwtKey`** — config VOs for algorithm, key material, audience, issuer (loaded from TOML)
- **`VerifierError`** — enum for auth failures: invalid signature, expired, missing claims, bad key
- **`VerifierSvc`** — SAF factory returning `Arc<dyn TokenVerifier>`; `NoopVerifierExtension` for test environments

## WHY

| Problem | Solution |
|---------|----------|
| Token verification coupled to a specific auth protocol (JWT vs. API key) | `TokenVerifier` trait; swap implementations via config without changing handler code |
| Test handlers blocked on a real JWT signing server | `NoopVerifierExtension` accepts any token in tests; production wires the real `JwtVerifier` via `VerifierSvc::from_config()` |
| Auth config buried in handler code | Key material, algorithm, audience, and issuer live in `[jwt]` TOML; rotated without recompiling |
| Error bridging from auth failures to `HandlerError` repeated at every handler boundary | `VerifierError` variants map cleanly to `HandlerError::Unauthorized` / `HandlerError::PermissionDenied` via `From` impls |
| Diamond dep conflicts when token types change | One crate, one tag — all consumers pin the same version; kgraph detects conflicts pre-commit |
