//! Tests for HttpDispatcherError.

use swe_edge_ingress_http::HttpDispatcherError;

#[test]
fn test_http_dispatcher_error_registration_failed() {
    let err = HttpDispatcherError::RegistrationFailed {
        pattern: "/test".into(),
        reason: "conflict".into(),
    };
    let msg = err.to_string();
    assert!(msg.contains("/test"));
    assert!(msg.contains("conflict"));
}
