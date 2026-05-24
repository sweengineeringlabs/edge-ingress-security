//! Wire-equivalent serving status enum.

/// Wire-equivalent of `grpc.health.v1.HealthCheckResponse.ServingStatus`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ServingStatus {
    /// Service status is unknown.
    Unknown = 0,
    /// Service is serving requests.
    Serving = 1,
    /// Service is not serving requests.
    NotServing = 2,
    /// Named service is unknown to the health reporter.
    ServiceUnknown = 3,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serving_status_wire_values_are_correct() {
        assert_eq!(ServingStatus::Unknown as i32, 0);
        assert_eq!(ServingStatus::Serving as i32, 1);
        assert_eq!(ServingStatus::NotServing as i32, 2);
        assert_eq!(ServingStatus::ServiceUnknown as i32, 3);
    }
}
