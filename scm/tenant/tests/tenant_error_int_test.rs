//! Integration tests for [`TenantError`].
// @covers: api/error/tenant_error.rs
#![allow(clippy::unwrap_used)]

use swe_edge_ingress_tenant::TenantError;

#[test]
fn test_tenant_error_invalid_config_displays_message() {
    let err = TenantError::InvalidConfig("bad strategy".to_string());
    assert!(err.to_string().contains("bad strategy"));
}

#[test]
fn test_tenant_error_invalid_config_is_debug_formattable() {
    let err = TenantError::InvalidConfig("x".to_string());
    assert!(format!("{err:?}").contains("InvalidConfig"));
}
