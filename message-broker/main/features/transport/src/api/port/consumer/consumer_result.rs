//! Result type alias for inbound message consumption operations.

use crate::api::port::consumer::consumer_error::ConsumerError;

/// Result type for [`MessageConsumer`](super::super::message_consumer::MessageConsumer) operations.
pub type ConsumerResult<T> = Result<T, ConsumerError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consumer_result_ok_variant_holds_value() {
        let r: ConsumerResult<u32> = Ok(42);
        assert_eq!(r.unwrap(), 42);
    }
}
