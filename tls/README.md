# swe-edge-ingress-tls

rustls TLS termination for `swe-edge` inbound servers.

Provides `IngressTlsConfig` (certificate + key paths) and
`build_tls_acceptor(config)` which returns a `TlsAcceptor` ready to pass
to `AxumHttpServer` or `TonicGrpcServer`. No transport knowledge — the
acceptor is handed off to the transport layer.

## Usage

```rust
use swe_edge_ingress_tls::{IngressTlsConfig, build_tls_acceptor};

let tls_config = IngressTlsConfig {
    cert_path: "certs/server.pem".into(),
    key_path:  "certs/server.key".into(),
};

let acceptor = build_tls_acceptor(&tls_config)?;
// Pass `acceptor` to AxumHttpServer::with_tls(…) or TonicGrpcServer::with_tls(…)
```

## Public surface (`saf/`)

| Export | Purpose |
|--------|---------|
| `IngressTlsConfig` | Certificate and private-key paths |
| `build_tls_acceptor(config)` | Build a `TlsAcceptor` from config |
| `TlsAcceptor` | Re-exported from `tokio-rustls` — no direct dependency needed |
| `IngressTlsError` | Error type for TLS setup failures |

## Crate layout (SEA)

| Layer | Path | Role |
|-------|------|------|
| `api/` | `src/api/` | Traits, value objects, error types |
| `core/` | `src/core/` | `pub(crate)` implementations |
| `saf/` | `src/saf/` | Factory functions + curated re-exports |

## Building

```bash
cd ingress/tls
cargo build
cargo test
cargo clippy -- -D warnings
```
