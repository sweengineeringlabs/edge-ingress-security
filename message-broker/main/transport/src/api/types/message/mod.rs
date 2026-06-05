//! Message-related types for the ingress consumer API.
pub(crate) mod message_broker_svc;
pub(crate) mod message_consumer_config;
pub(crate) mod message_consumer_handle;

pub use message_broker_svc::MessageConsumerSvc;
pub use message_consumer_config::MessageConsumerConfig;
pub use message_consumer_handle::MessageConsumerHandle;
