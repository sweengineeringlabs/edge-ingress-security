//! Tests for HttpHandlerRegistryDispatcher.

use swe_edge_ingress_http::HttpHandlerRegistryDispatcher;

#[test]
fn test_http_handler_registry_dispatcher_exists() {
    let _ = std::any::type_name::<HttpHandlerRegistryDispatcher>();
}
