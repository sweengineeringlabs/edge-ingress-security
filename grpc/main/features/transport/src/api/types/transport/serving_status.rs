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
