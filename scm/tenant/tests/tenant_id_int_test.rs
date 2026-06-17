//! Integration tests for [`TenantId`].
// @covers: api/types/tenant/tenant_id.rs
#![allow(clippy::unwrap_used)]

use swe_edge_ingress_tenant::TenantId;

#[test]
fn test_tenant_id_new_stores_string_value() {
    let id = TenantId::new("acme");
    assert_eq!(id.as_str(), "acme");
}

#[test]
fn test_tenant_id_from_string_wraps_value() {
    let id = TenantId::from("globex".to_string());
    assert_eq!(id.as_str(), "globex");
}

#[test]
fn test_tenant_id_from_str_slice_wraps_value() {
    let id = TenantId::from("initech");
    assert_eq!(id.as_str(), "initech");
}

#[test]
fn test_tenant_id_display_formats_inner_string() {
    let id = TenantId::new("umbrella");
    assert_eq!(id.to_string(), "umbrella");
}

#[test]
fn test_tenant_id_clone_is_equal_to_original() {
    let id = TenantId::new("acme");
    assert_eq!(id.clone(), id);
}

#[test]
fn test_tenant_id_equality_compares_inner_string() {
    let a = TenantId::new("acme");
    let b = TenantId::new("acme");
    assert_eq!(a, b);
}

#[test]
fn test_tenant_id_inequality_for_different_strings() {
    let a = TenantId::new("acme");
    let b = TenantId::new("globex");
    assert_ne!(a, b);
}
