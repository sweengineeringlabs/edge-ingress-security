//! Integration tests for the authz interceptor's public surface.

use std::sync::Arc;
use std::time::Duration;

use swe_edge_ingress_grpc::{
    GrpcIngressError, GrpcIngressInterceptor, GrpcMetadata, GrpcRequest, GrpcStatusCode,
    PeerIdentity, PEER_CN,
};
use swe_edge_ingress_grpc_authz::{
    AuthzInterceptor, AuthzPolicy, MethodAclConfig, MethodAclPolicy,
};

fn req_with_cn(cn: &str, method: &str) -> GrpcRequest {
    let mut headers = std::collections::HashMap::new();
    headers.insert(PEER_CN.to_string(), cn.to_string());
    GrpcRequest::new(method, vec![], Duration::from_secs(1)).with_metadata(GrpcMetadata { headers })
}

/// @covers: before_dispatch
#[test]
fn authz_struct_interceptor_method_acl_policy_allows_listed_method_int_test() {
    let cfg = MethodAclConfig::deny_all().allow("alice", ["/svc/Read".into()]);
    let policy = MethodAclPolicy::from_config(cfg);
    let interceptor = AuthzInterceptor::from_policy(policy);

    let mut req = req_with_cn("alice", "/svc/Read");
    interceptor
        .before_dispatch(&mut req)
        .expect("alice/Read allowed");
}

/// @covers: before_dispatch
#[test]
fn authz_struct_interceptor_method_acl_policy_denies_method_outside_acl_int_test() {
    let cfg = MethodAclConfig::deny_all().allow("alice", ["/svc/Read".into()]);
    let policy = MethodAclPolicy::from_config(cfg);
    let interceptor = AuthzInterceptor::from_policy(policy);

    let mut req = req_with_cn("alice", "/svc/Write");
    match interceptor.before_dispatch(&mut req) {
        Err(GrpcIngressError::Status(GrpcStatusCode::PermissionDenied, _)) => {}
        other => panic!("expected PermissionDenied, got {other:?}"),
    }
}

/// @covers: before_dispatch
#[test]
fn authz_struct_interceptor_closure_policy_round_trips_int_test() {
    let interceptor = AuthzInterceptor::from_policy(|id: &PeerIdentity, m: &str| {
        id.cn.as_deref() == Some("alice") && m == "/svc/Read"
    });
    let mut allowed = req_with_cn("alice", "/svc/Read");
    let mut denied = req_with_cn("alice", "/svc/Write");
    interceptor.before_dispatch(&mut allowed).expect("allowed");
    match interceptor.before_dispatch(&mut denied) {
        Err(GrpcIngressError::Status(GrpcStatusCode::PermissionDenied, _)) => {}
        other => panic!("expected PermissionDenied, got {other:?}"),
    }
}

/// @covers: from_shared_policy
#[test]
fn authz_struct_interceptor_from_shared_policy_accepts_arc_int_test() {
    let policy: Arc<dyn AuthzPolicy> = Arc::new(|_: &PeerIdentity, _: &str| true);
    let interceptor = AuthzInterceptor::from_shared_policy(policy);
    let mut req = req_with_cn("alice", "/svc/M");
    interceptor
        .before_dispatch(&mut req)
        .expect("shared policy");
}
