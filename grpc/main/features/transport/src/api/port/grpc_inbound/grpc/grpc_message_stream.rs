//! Message stream type alias for gRPC inbound operations.

use super::grpc_inbound_result::GrpcInboundResult;

/// A stream of raw gRPC message bytes — one item per decoded gRPC frame.
pub type GrpcMessageStream =
    std::pin::Pin<Box<dyn futures::Stream<Item = GrpcInboundResult<Vec<u8>>> + Send>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grpc_message_stream_is_usable_as_pin_box_stream() {
        // Simply verifies the type alias compiles and is a concrete alias.
        use futures::stream;
        let _: GrpcMessageStream = Box::pin(stream::empty::<GrpcInboundResult<Vec<u8>>>());
    }
}
