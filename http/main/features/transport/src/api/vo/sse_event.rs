//! SSE event value object.

/// A single Server-Sent Event frame.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SseEvent {
    /// Optional `event:` field — names the event type.
    pub event: Option<String>,
    /// The `data:` field — the payload carried by the event.
    pub data: String,
    /// Optional `id:` field — the last-event-ID for reconnect resumption.
    pub id: Option<String>,
}

impl SseEvent {
    /// Construct a data-only event with no type or ID.
    pub fn data(data: impl Into<String>) -> Self {
        Self {
            event: None,
            data: data.into(),
            id: None,
        }
    }
}
