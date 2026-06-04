//! Integration tests for CompressionMode.

use swe_edge_ingress_grpc_transport::CompressionMode;

/// @covers: CompressionMode::default
#[test]
fn test_default_is_none_compression() {
    assert_eq!(CompressionMode::default(), CompressionMode::None);
}

/// @covers: CompressionMode::header_value
#[test]
fn test_header_value_uses_canonical_grpc_identifiers() {
    assert_eq!(CompressionMode::None.header_value(), None);
    assert_eq!(CompressionMode::Gzip.header_value(), Some("gzip"));
    assert_eq!(CompressionMode::Zstd.header_value(), Some("zstd"));
}
