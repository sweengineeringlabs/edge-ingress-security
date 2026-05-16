# swe-edge-ingress

Aggregating ingress facade for the `swe-edge` stack.

Re-exports the full public surfaces of `swe-edge-ingress-http`,
`swe-edge-ingress-grpc`, `swe-edge-ingress-tls`, and `edge-domain` behind
a single crate, and adds the gateway-level primitives: `ApplicationConfigBuilder`,
`DaemonRunner`, `Pipeline`, `Router`, and `RateLimiter`.

Most application crates depend on this crate rather than the individual
transport crates.

## Usage

```rust
use swe_edge_ingress::{
    ApplicationConfigBuilder, DaemonRunner,
    HttpInbound, HttpRequest, HttpResponse,
    Handler, HandlerError, HandlerRegistry,
};
```

## Public surface (`saf/`)

### Gateway primitives

| Export | Purpose |
|--------|---------|
| `ApplicationConfigBuilder` | Fluent builder — assembles file-backed ingress config |
| `build_file_input(config)` | Build an `InboundSource` from a file path |
| `DaemonRunner` | Drive an inbound pipeline until shutdown |
| `DaemonContext` | Runtime context passed into the daemon loop |
| `Pipeline` / `Router` | Request processing chain + intent classification |
| `RateLimiter` / `RateLimiterBuilder` / `RateLimiterSpec` | Token-bucket rate limiter |
| `Pagination` / `PaginatedResponse` | Common pagination value objects |
| `HealthCheck` / `HealthStatus` | Inbound-level health primitives |
| `IngressError` / `IngressResult` | Unified error + result types |
| `MetricsCollector` / `MetricFields` | Per-request metric capture |
| `passthrough_validator()` | No-op validator for dev/test |

### HTTP re-exports

`HttpInbound`, `HttpRequest`, `HttpResponse`, `HttpAuth`, `HttpBody`,
`HttpConfig`, `HttpHealthCheck`, `HttpMethod`, `HttpInboundError`,
`HttpInboundResult`, `AxumHttpServer`, `RequestContext`,
`HttpHandlerAdapter`, `HttpHandlerRegistryDispatcher`,
`HttpDecodeFn`, `HttpEncodeFn`

### gRPC re-exports

`GrpcInbound`, `GrpcRequest`, `GrpcResponse`, `GrpcMetadata`,
`GrpcStatusCode`, `GrpcMessageStream`, `GrpcHealthCheck`,
`GrpcInboundError`, `GrpcInboundResult`, `TonicGrpcServer`,
`GrpcInboundInterceptor`, `AuthorizationInterceptor`,
`GrpcInboundInterceptorChain`, `GrpcDecodeFn`, `GrpcEncodeFn`,
`GrpcHandlerAdapter`, `GrpcHandlerRegistryDispatcher`

### TLS re-exports

`IngressTlsConfig`, `IngressTlsError`

### Domain re-exports

`Handler`, `HandlerError`, `HandlerRegistry`

## Crate layout (SEA)

| Layer | Path | Role |
|-------|------|------|
| `api/` | `src/api/` | Traits, value objects, error types |
| `core/` | `src/core/` | `pub(crate)` implementations |
| `saf/` | `src/saf/` | Factory functions + curated re-exports |

## Building

```bash
cd ingress/gateway
cargo build
cargo test
cargo clippy -- -D warnings
```
