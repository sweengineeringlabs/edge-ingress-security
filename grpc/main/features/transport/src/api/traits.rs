//! SEA interface contract — inbound gRPC transport traits.

/// Validates an inbound configuration or interceptor value.
pub trait Validator {
    /// Returns `Ok(())` when the value is valid, or a human-readable error.
    fn validate(&self) -> Result<(), String>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::port::grpc::GrpcIngress;

    #[test]
    fn test_validator_is_object_safe() {
        fn _assert(_: &dyn Validator) {}
    }

    #[test]
    fn test_grpc_ingress_is_object_safe() {
        fn _assert(_: &dyn GrpcIngress) {}
    }
}
