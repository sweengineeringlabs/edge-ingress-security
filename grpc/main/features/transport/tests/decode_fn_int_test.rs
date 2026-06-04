//! Integration tests for DecodeFn.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_ingress_grpc_transport::{GrpcDecodeFn, GrpcIngressError};

/// @covers: DecodeFn
#[test]
fn test_decode_fn_type_alias_is_usable() {
    fn decode(bytes: &[u8]) -> Result<u32, GrpcIngressError> {
        if bytes.len() != 4 {
            return Err(GrpcIngressError::InvalidArgument("need 4 bytes".into()));
        }
        Ok(u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }
    let f: GrpcDecodeFn<u32> = decode;
    assert_eq!(f(&[0, 0, 0, 1]).unwrap(), 1u32);
}
