//! Decode function pointer type for gRPC handler adapters.

use crate::api::port::grpc_inbound::GrpcInboundError;

/// Function pointer that decodes a typed request from raw protobuf bytes.
///
/// Implementations should return [`GrpcInboundError::InvalidArgument`]
/// when the bytes cannot be parsed — that surfaces as
/// `tonic::Code::InvalidArgument` on the wire.
pub type DecodeFn<Req> = fn(&[u8]) -> Result<Req, GrpcInboundError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_fn_type_alias_is_usable() {
        fn decode(bytes: &[u8]) -> Result<u32, GrpcInboundError> {
            if bytes.len() != 4 {
                return Err(GrpcInboundError::InvalidArgument("need 4 bytes".into()));
            }
            Ok(u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
        }
        let f: DecodeFn<u32> = decode;
        assert_eq!(f(&[0, 0, 0, 1]).unwrap(), 1u32);
    }
}
