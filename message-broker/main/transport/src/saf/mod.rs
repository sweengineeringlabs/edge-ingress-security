//! SAF — ingress message consumer public factory surface.
mod transport_svc;

pub use crate::api::error::ConsumerError;
pub use crate::api::port::{ConsumerResult, MessageConsumer};
pub use crate::api::types::{
    ApplicationConfigBuilder, MessageBrokerSvc, MessageConsumerConfig, MessageConsumerHandle,
};
pub use swe_edge_message_broker::{Message, MessageStream};
