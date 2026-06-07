# Architecture — edge-ingress-security

Two crates in this workspace: `swe-edge-ingress-verifier` (JWT token verification) and `swe-edge-ingress-tls` (TLS configuration).

---

## Sequence — Token Verification

> An HTTP server extracts the `Authorization: Bearer <token>` header and verifies it via `JwtVerifier` before forwarding the request.

```mermaid
sequenceDiagram
    participant HttpServer
    participant JwtVerifier
    participant JwtDecoder

    HttpServer->>JwtVerifier: from_config(&JwtConfig)
    JwtVerifier-->>HttpServer: JwtVerifier

    HttpServer->>JwtVerifier: verify(bearer_token)
    JwtVerifier->>JwtDecoder: decode(token, secret)
    JwtDecoder-->>JwtVerifier: Claims or DecodeError

    JwtVerifier-->>HttpServer: Result<Claims, VerificationError>

    alt valid
        HttpServer->>HttpServer: proceed with request
    else invalid / expired
        HttpServer-->>Client: 401 Unauthorized
    end
```

## Data Flow — Token Verification

> A raw bearer token enters `JwtVerifier`; validated claims or a typed error exit.

```mermaid
flowchart LR
    A["bearer_token: &str\n(from Authorization header)"] --> B["JwtVerifier::verify"]
    B --> C{decode + validate}
    C -->|signature valid\nexpiry ok\nissuer matches| D["Claims\n──────\nsub, exp\naud, iss\ncustom fields"]
    C -->|expired| E["VerificationError\n::Expired"]
    C -->|bad signature| F["VerificationError\n::InvalidSignature"]
    C -->|missing field| G["VerificationError\n::MissingClaim"]
```

---

## Sequence — TLS Configuration

> TLS is loaded once at startup from TOML and applied to the axum/tonic server builder.

```mermaid
sequenceDiagram
    participant Runtime
    participant ConfigLoader
    participant IngressTlsConfig
    participant AxumServer

    Runtime->>ConfigLoader: load_section("tls")
    ConfigLoader-->>Runtime: IngressTlsConfig { cert_path, key_path }

    Runtime->>IngressTlsConfig: load_rustls_config()
    IngressTlsConfig-->>Runtime: Arc<ServerConfig>

    Runtime->>AxumServer: bind_rustls(addr, tls_config)
    AxumServer-->>Runtime: listening on HTTPS
```

## Data Flow — TLS Configuration

> A TOML section becomes a `rustls::ServerConfig` used by the HTTP/gRPC listener.

```mermaid
flowchart LR
    A["TOML [tls]\ncert_path\nkey_path\noptional_ca_path"] --> B["IngressTlsConfig\n(deserialized)"]
    B --> C["load_rustls_config()"]
    C -->|read PEM files| D["rustls::Certificate\nrustls::PrivateKey"]
    D --> E["Arc<rustls::ServerConfig>"]
    E --> F["axum / tonic\nserver listener"]
```
