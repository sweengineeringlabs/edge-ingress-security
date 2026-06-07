# swe-edge-ingress-verifier

> **TLDR:** Pluggable inbound token verification for swe-edge — JWT (HMAC-SHA256 / RS256), API key, and noop stub behind one `TokenVerifier` trait. See [Overview](../docs/README.md) for details.

Token verification for `swe-edge` inbound transports.

Provides the `TokenVerifier` port trait and two implementations: `JwtVerifier`
(HMAC-SHA256 / RS256) and `ApiKeyVerifier` (static lookup). Used by the HTTP
bearer interceptor and the gRPC `AuthorizationInterceptor`.

## Usage

```rust
use swe_edge_ingress_verifier::{JwtVerifier, JwtConfig, JwtKey, TokenVerifier};

let config   = JwtConfig { key: JwtKey::Hmac("secret".into()), ..Default::default() };
let verifier = JwtVerifier::new(config);

let claims = verifier.verify("Bearer eyJ…").await?;
println!("subject: {}", claims.sub);
```

## Public surface (`saf/`)

| Export | Purpose |
|--------|---------|
| `TokenVerifier` | Port trait — `verify(token)` → `Result<Claims, VerifierError>` |
| `JwtVerifier` | HMAC-SHA256 / RS256 JWT implementation |
| `ApiKeyVerifier` | Static API-key lookup implementation |
| `JwtConfig` | Algorithm, issuer, audience, and expiry settings |
| `JwtKey` | Key material — `Hmac(secret)` or `Rsa(pem)` |
| `Claims` | Decoded JWT payload (`sub`, `iss`, `exp`, custom fields) |
| `VerifierError` | `InvalidToken`, `Expired`, `Unauthorized`, `Internal` |

## Crate layout (SEA)

| Layer | Path | Role |
|-------|------|------|
| `api/` | `src/api/` | Traits, value objects, error types |
| `core/` | `src/core/` | `pub(crate)` implementations |
| `saf/` | `src/saf/` | Factory functions + curated re-exports |

## Building

```bash
cd ingress/verifier
cargo build
cargo test
cargo clippy -- -D warnings
```
