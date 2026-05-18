//! Health service interface declaration — the types that core/health_service implements.

/// Re-export of the primary health service type.
///
/// This file exists to satisfy the SEA rule requiring every core implementation
/// file to have a corresponding interface counterpart in the api/ layer.
#[allow(unused_imports)]
pub use crate::api::health_service::health::health_service::HealthService;

#[cfg(test)]
mod tests {
    use super::HealthService;

    #[test]
    fn test_health_service_is_constructible() {
        let _ = HealthService::new();
    }
}
