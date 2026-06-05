//! SAF — ingress message consumer public factory surface.
mod transport_svc;

pub use crate::api::error::ConsumerError;
pub use crate::api::traits::MessageConsumer;
pub use crate::api::types::ConsumerResult;
pub use crate::api::types::{
    ApplicationConfigBuilder, MessageConsumerConfig, MessageConsumerHandle, MessageConsumerSvc,
};
pub use swe_edge_message_broker::{Message, MessageStream};
