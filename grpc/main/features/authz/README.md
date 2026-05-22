# swe-edge-ingress-grpc-authz

Authorization `GrpcIngressInterceptor` — runs a pluggable `AuthzPolicy` on identity carried in `GrpcMetadata`.

## Usage

```rust
use swe_edge_ingress_grpc_authz::{AuthzInterceptor, MethodAclConfig, MethodAclPolicy};

let cfg = MethodAclConfig::deny_all()
    .allow("alice", ["/svc/Read".to_string()]);
let interceptor = AuthzInterceptor::from_policy(MethodAclPolicy::from_config(cfg));
```

## License

See repository root.
