//! Integration tests for TonicServerError.

use swe_edge_ingress_grpc_transport::TonicServerError;

/// @covers: TonicServerError::Bind
#[test]
fn test_tonic_server_error_bind_formats_correctly() {
    let err = TonicServerError::Bind(
        "127.0.0.1:443".into(),
        std::io::Error::new(std::io::ErrorKind::AddrInUse, "port in use"),
    );
    let msg = err.to_string();
    assert!(msg.contains("127.0.0.1:443"));
}
