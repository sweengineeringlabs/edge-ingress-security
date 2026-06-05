//! Integration tests — ConsumerResult type alias.

use swe_edge_ingress_message_broker_transport::{ConsumerError, ConsumerResult};

/// @covers: ConsumerResult — Ok variant
#[test]
fn test_consumer_result_ok_variant_holds_value() {
    let r: ConsumerResult<u32> = Ok(42);
    assert_eq!(r.ok(), Some(42));
}

/// @covers: ConsumerResult — Err variant
#[test]
fn test_consumer_result_err_variant_holds_error() {
    let r: ConsumerResult<u32> = Err(ConsumerError::Unavailable("down".into()));
    assert!(r.is_err());
}
