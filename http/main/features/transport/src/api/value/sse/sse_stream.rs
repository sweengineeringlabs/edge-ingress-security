//! SSE stream type alias.

use std::pin::Pin;

use futures::Stream;

use crate::api::port::http_ingress_error::HttpIngressError;
use crate::api::value::sse::sse_event::SseEvent;

/// A lazy stream of [`SseEvent`] items pushed to a connected HTTP client.
///
/// The server drives this stream; the transport layer converts each item into
/// a `text/event-stream` frame and flushes it to the wire.
pub type SseStream = Pin<Box<dyn Stream<Item = Result<SseEvent, HttpIngressError>> + Send>>;
