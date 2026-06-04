//! Encode function pointer type for gRPC handler adapters.

/// Function pointer that encodes a typed response to raw protobuf bytes.
///
/// Implementations are infallible by contract — if encoding can fail
/// for the concrete type, wrap it inline (e.g. `prost::Message::encode`
/// can fail only when the buffer is too small, which never happens for
/// `Vec<u8>`).
pub type EncodeFn<Resp> = fn(&Resp) -> Vec<u8>;
