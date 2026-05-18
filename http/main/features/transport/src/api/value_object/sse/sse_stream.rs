//! SSE stream type alias.

use std::pin::Pin;

use futures::Stream;

use crate::api::port::http_inbound::HttpInboundError;
use crate::api::value_object::sse::sse_event::SseEvent;

/// A lazy stream of [`SseEvent`] items pushed to a connected HTTP client.
///
/// The server drives this stream; the transport layer converts each item into
/// a `text/event-stream` frame and flushes it to the wire.
pub type SseStream = Pin<Box<dyn Stream<Item = Result<SseEvent, HttpInboundError>> + Send>>;

#[cfg(test)]
mod tests {
    use futures::stream;

    use super::*;

    /// @covers: SseStream
    #[test]
    fn test_sse_stream_empty_stream_is_valid() {
        let _s: SseStream = Box::pin(stream::empty());
    }
}
