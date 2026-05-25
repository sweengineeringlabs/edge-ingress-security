//! Tests for SseEvent.

use swe_edge_ingress_http::SseEvent;

#[test]
fn test_sse_event_instantiable() {
    let _event = SseEvent::default();
}
