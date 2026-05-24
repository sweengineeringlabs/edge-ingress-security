//! Integration tests for the HandlerDispatch interface module.

use edge_domain::HandlerRegistry;
use std::sync::Arc;
use swe_edge_ingress_grpc_transport::GrpcHandlerRegistryDispatcher;

/// @covers: HandlerDispatch
/// Verifies the handler dispatch concrete type is accessible through the public SAF.
#[test]
fn test_handler_dispatch_concrete_type_is_accessible_via_saf() {
    // GrpcHandlerRegistryDispatcher is the concrete impl of the HandlerDispatch contract.
    // If the api/handler_dispatch interface module disappears or breaks, this will fail to compile.
    let d = GrpcHandlerRegistryDispatcher::new(Arc::new(HandlerRegistry::new()));
    assert!(d.registry().is_empty());
}
