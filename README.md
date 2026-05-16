# swe-edge-ingress

Inbound port contracts and implementations for the `swe-edge` stack.

Five independent workspaces — HTTP transport, gRPC transport, TLS termination,
token verification, and the aggregating gateway facade. Application code imports
only `api/` traits; factory functions in `saf/` supply the concrete instances.

## Workspaces

| Workspace | Crate | Role |
|-----------|-------|------|
| `ingress/http/` | `swe-edge-ingress-http` | Axum HTTP/1.1 + HTTP/2 inbound server |
| `ingress/grpc/` | `swe-edge-ingress-grpc` | Tonic gRPC inbound server + reflection |
| `ingress/tls/` | `swe-edge-ingress-tls` | rustls TLS acceptor builder |
| `ingress/verifier/` | `swe-edge-ingress-verifier` | JWT + API-key token verification |
| `ingress/gateway/` | `swe-edge-ingress` | Aggregating facade — re-exports all of the above |

## HTTP inbound (`ingress/http/`)

| Export | Purpose |
|--------|---------|
| `HttpInbound` | Port trait — `handle(req)` / `health_check()` |
| `HttpRequest` / `HttpResponse` | Value objects |
| `AxumHttpServer` | `serve(signal)` blocks until shutdown signal |
| `HttpHandlerAdapter` | Adapt a domain `Handler` to `HttpInbound` |
| `HttpHandlerRegistryDispatcher` | Dispatch a `HandlerRegistry` over HTTP |
| `RequestContext` | Auth / tenant / trace metadata |

## gRPC inbound (`ingress/grpc/`)

| Export | Purpose |
|--------|---------|
| `GrpcInbound` | Port trait — `handle(req)` / `health_check()` |
| `GrpcRequest` / `GrpcResponse` | Value objects |
| `TonicGrpcServer` | `serve(signal)` blocks until shutdown signal |
| `GrpcInboundInterceptor` | Per-call interceptor hook |
| `AuthorizationInterceptor` | Bearer token interceptor |
| `GrpcInboundInterceptorChain` | Ordered interceptor composition |

## TLS (`ingress/tls/`)

| Export | Purpose |
|--------|---------|
| `IngressTlsConfig` | Certificate + key paths |
| `build_tls_acceptor(config)` | Returns a `TlsAcceptor` for use with Axum or Tonic |

## HTTP inbound error types (`ingress/http/`)

`HttpInboundError` maps to HTTP status codes via Axum's `error_response`:

| Variant | HTTP status | Meaning |
|---------|-------------|---------|
| `InvalidInput` | 400 | Malformed request |
| `Unauthorized` | 401 | Caller not authenticated |
| `PermissionDenied` | 403 | Caller lacks permission |
| `NotFound` | 404 | Resource not found |
| `MethodNotAllowed` | 405 | Operation not supported by handler |
| `Conflict` | 409 | Duplicate or constraint violation |
| `UnprocessableEntity` | 422 | Valid request rejected by business rule |
| `Timeout` | 504 | Handler timed out |
| `Unavailable` | 503 | Handler unhealthy or upstream unavailable |
| `Internal` | 500 | Unexpected server-side error |

## Token verification (`ingress/verifier/`)

| Export | Purpose |
|--------|---------|
| `TokenVerifier` | Port trait — `verify(token)` → `Claims` |
| `JwtVerifier` | HMAC-SHA256 / RS256 implementation |
| `ApiKeyVerifier` | Static API-key lookup |
| `JwtConfig` / `JwtKey` | Config value objects |
| `Claims` | Decoded JWT payload |
| `From<VerifierError> for HandlerError` | Bridge — auth errors propagate via `?` in handlers |

## Gateway facade (`ingress/gateway/`)

Re-exports the full ingress surface plus:

| Export | Purpose |
|--------|---------|
| `ApplicationConfigBuilder` | Fluent builder for assembling an ingress config |
| `DaemonRunner` | Drive an inbound pipeline to completion |
| `Pipeline` / `Router` | Request processing chain + intent classification |
| `RateLimiter` | Token-bucket rate limiter |
| `Pagination` / `PaginatedResponse` | Common pagination value objects |

## Building

```bash
cd ingress/http     && cargo build && cargo test
cd ingress/grpc     && cargo build && cargo test
cd ingress/tls      && cargo build && cargo test
cd ingress/verifier && cargo build && cargo test
cd ingress/gateway  && cargo build && cargo test
```
