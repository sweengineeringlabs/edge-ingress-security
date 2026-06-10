//! SAF — ingress message consumer public factory surface.
mod consumer_svc;

pub use crate::api::error::ConsumerError;
pub use crate::api::types::ConsumerResult;
pub use crate::api::types::{ApplicationConfigBuilder, MessageConsumerConfig};
pub use crate::api::types::{MessageConsumerHandle, MessageConsumerSvc};
pub use swe_edge_message_broker::{Message, MessageStream};
