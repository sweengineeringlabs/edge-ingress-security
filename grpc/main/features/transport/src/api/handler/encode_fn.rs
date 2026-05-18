//! Encode function pointer type for gRPC handler adapters.

/// Function pointer that encodes a typed response to raw protobuf bytes.
///
/// Implementations are infallible by contract — if encoding can fail
/// for the concrete type, wrap it inline (e.g. `prost::Message::encode`
/// can fail only when the buffer is too small, which never happens for
/// `Vec<u8>`).
pub type EncodeFn<Resp> = fn(&Resp) -> Vec<u8>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_fn_type_alias_is_usable() {
        fn encode(v: &u32) -> Vec<u8> {
            v.to_be_bytes().to_vec()
        }
        let f: EncodeFn<u32> = encode;
        assert_eq!(f(&1u32), vec![0, 0, 0, 1]);
    }
}
