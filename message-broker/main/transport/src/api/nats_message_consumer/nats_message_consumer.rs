//! Api interface counterpart for `core/nats_message_consumer`.
pub use crate::api::port::MessageConsumer;
pub use crate::api::traits::Validator;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_message_consumer_interface_is_accessible() {
        fn _assert(_: &dyn MessageConsumer) {}
    }
    #[test]
    fn test_validator_interface_is_accessible() {
        fn _assert(_: &dyn Validator) {}
    }
}
