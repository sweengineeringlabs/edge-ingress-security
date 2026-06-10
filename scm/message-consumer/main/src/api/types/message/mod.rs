//! Concrete message consumer types.
pub mod message_consumer_config;
pub(crate) mod message_consumer_handle;
pub(crate) mod message_consumer_svc;
pub use message_consumer_config::MessageConsumerConfig;
pub use message_consumer_handle::MessageConsumerHandle;
pub use message_consumer_svc::MessageConsumerSvc;
