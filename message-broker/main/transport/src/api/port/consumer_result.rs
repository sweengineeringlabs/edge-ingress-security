//! Result type alias for inbound message consumption operations.

use crate::api::error::consumer_error::ConsumerError;

/// Result type for [`MessageConsumer`](crate::api::port::MessageConsumer) operations.
pub type ConsumerResult<T> = Result<T, ConsumerError>;
