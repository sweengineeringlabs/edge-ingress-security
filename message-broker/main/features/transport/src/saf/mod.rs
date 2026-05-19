//! SAF — ingress message consumer public factory surface.
mod edge_ingress_message_broker_svc;

pub use crate::api::port::{ConsumerError, ConsumerResult, MessageConsumer};
pub use crate::api::ApplicationConfigBuilder;
pub use edge_ingress_message_broker_svc::{check_health, subscribe_to, validate};
pub use swe_edge_message_broker::{Message, MessageStream};

#[cfg(feature = "in-memory")]
pub use edge_ingress_message_broker_svc::default_consumer;

#[cfg(feature = "nats")]
pub use edge_ingress_message_broker_svc::nats_consumer;
