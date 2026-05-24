//! Integration tests for [`MtlsAuthError`].

use swe_edge_ingress_grpc_auth_mtls::*;

/// @covers: MissingIdentity
#[test]
fn test_missing_identity_display_message_mentions_no_identity() {
    let err = MtlsAuthError::MissingIdentity;
    let s = err.to_string();
    assert!(s.contains("no mTLS peer identity"), "unexpected: {s}");
}

/// @covers: DisallowedCn
#[test]
fn test_disallowed_cn_display_includes_offending_cn() {
    let err = MtlsAuthError::DisallowedCn("evil.svc".into());
    assert!(err.to_string().contains("evil.svc"));
}
