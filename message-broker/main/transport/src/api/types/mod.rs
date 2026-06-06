//! Concrete types for the ingress message consumer API.
pub(crate) mod consumer_result;
pub(crate) mod message_broker_svc;
pub(crate) mod message_consumer_handle;

pub use consumer_result::ConsumerResult;
pub use message_broker_svc::MessageConsumerSvc;
pub use message_consumer_handle::MessageConsumerHandle;
