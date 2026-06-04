//! Public API tests for transport SAF layer.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::net::SocketAddr;
use swe_edge_ingress_grpc_transport::{validate, GrpcServerConfig};

#[test]
fn test_create_config_builder_returns_builder_with_name_and_version() {
    // Note: swe_edge_ingress_grpc_transport module is re-exported via saf/ with builder
    let builder = swe_edge_ingress_grpc_transport::create_config_builder();
    assert_eq!(builder.name(), "swe-edge-ingress-grpc-transport");
    assert_eq!(builder.version(), env!("CARGO_PKG_VERSION"));
}

#[test]
fn test_validate_accepts_plaintext_grpc_server_config() {
    let config =
        GrpcServerConfig::new("127.0.0.1:0".parse::<SocketAddr>().unwrap()).allow_plaintext();
    let result = validate(&config);
    assert!(
        result.is_ok(),
        "validate must accept plaintext GrpcServerConfig: {:?}",
        result
    );
}

#[test]
fn test_validate_rejects_config_with_tls_required_but_no_tls_material() {
    let config = GrpcServerConfig::new("127.0.0.1:0".parse::<SocketAddr>().unwrap());
    let result = validate(&config);
    assert!(
        result.is_err(),
        "validate must reject config with tls_required=true but no TLS material"
    );
}
