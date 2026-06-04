//! Integration tests for EncodeFn.

use swe_edge_ingress_grpc_transport::GrpcEncodeFn;

/// @covers: EncodeFn
#[test]
fn test_encode_fn_type_alias_is_usable() {
    fn encode(v: &u32) -> Vec<u8> {
        v.to_be_bytes().to_vec()
    }
    let f: GrpcEncodeFn<u32> = encode;
    assert_eq!(f(&1u32), vec![0, 0, 0, 1]);
}
