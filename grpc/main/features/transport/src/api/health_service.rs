//! Health service declarations and inherent methods.

use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::RwLock;
use tokio::sync::broadcast;

use crate::api::port::grpc_inbound::GrpcInbound;

/// Wire-equivalent of `grpc.health.v1.HealthCheckResponse.ServingStatus`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ServingStatus {
    Unknown        = 0,
    Serving        = 1,
    NotServing     = 2,
    ServiceUnknown = 3,
}

pub const HEALTH_CHECK_METHOD:    &str  = "/grpc.health.v1.Health/Check";
pub const HEALTH_WATCH_METHOD:    &str  = "/grpc.health.v1.Health/Watch";
pub const WATCH_CHANNEL_CAPACITY: usize = 16;

/// Implementation of the standard `grpc.health.v1.Health` service.
pub struct HealthService {
    pub(crate) statuses:    RwLock<HashMap<String, ServingStatus>>,
    pub(crate) broadcaster: broadcast::Sender<(String, ServingStatus)>,
}

/// Aggregate dispatcher — ties [`HealthService`] to a registry-backed
/// dispatcher so the overall service-name reflects the health of every
/// registered handler.
pub struct HealthAggregate {
    pub(crate) service:    Arc<HealthService>,
    pub(crate) dispatcher: Arc<dyn GrpcInbound>,
}

impl HealthService {
    /// Construct an empty service. Overall service starts as [`ServingStatus::Serving`].
    pub fn new() -> Self {
        let (tx, _rx) = broadcast::channel(WATCH_CHANNEL_CAPACITY);
        let mut statuses = HashMap::new();
        statuses.insert(String::new(), ServingStatus::Serving);
        Self { statuses: RwLock::new(statuses), broadcaster: tx }
    }

    /// Set the status for a named service.
    pub fn set_status(&self, service: impl Into<String>, status: ServingStatus) {
        let service = service.into();
        { let mut guard = self.statuses.write(); guard.insert(service.clone(), status); }
        let _ = self.broadcaster.send((service, status));
    }

    /// Convenience alias for `set_status("", status)`.
    pub fn set_overall_status(&self, status: ServingStatus) {
        self.set_status(String::new(), status);
    }

    /// Look up the current status for a service.
    pub fn get_status(&self, service: &str) -> Option<ServingStatus> {
        self.statuses.read().get(service).copied()
    }

    /// Subscribe to status changes.
    pub fn subscribe(&self) -> broadcast::Receiver<(String, ServingStatus)> {
        self.broadcaster.subscribe()
    }

    /// Recompute the overall status from every named service.
    pub fn recompute_overall_status(&self) {
        let new_overall = {
            let guard = self.statuses.read();
            let named: Vec<_> = guard.iter().filter(|(n, _)| !n.is_empty()).collect();
            if named.is_empty() {
                ServingStatus::Serving
            } else if named.iter().all(|(_, s)| **s == ServingStatus::Serving) {
                ServingStatus::Serving
            } else {
                ServingStatus::NotServing
            }
        };
        self.set_overall_status(new_overall);
    }
}

impl Default for HealthService {
    fn default() -> Self { Self::new() }
}

impl HealthAggregate {
    /// Bind a [`HealthService`] to a dispatcher.
    pub fn new(service: Arc<HealthService>, dispatcher: Arc<dyn GrpcInbound>) -> Self {
        Self { service, dispatcher }
    }

    /// Re-poll the dispatcher and update the overall service status.
    pub async fn refresh(&self) {
        use crate::api::port::grpc_inbound::GrpcHealthCheck;
        let h = self.dispatcher.health_check().await;
        let status = match h {
            Ok(c) if c.healthy => ServingStatus::Serving,
            _                  => ServingStatus::NotServing,
        };
        self.service.set_overall_status(status);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: ServingStatus — wire values are stable.
    #[test]
    fn test_serving_status_wire_values_are_correct() {
        assert_eq!(ServingStatus::Unknown        as i32, 0);
        assert_eq!(ServingStatus::Serving        as i32, 1);
        assert_eq!(ServingStatus::NotServing     as i32, 2);
        assert_eq!(ServingStatus::ServiceUnknown as i32, 3);
    }

    /// @covers: HealthService::new — overall service starts as SERVING.
    #[test]
    fn test_new_health_service_overall_status_starts_serving() {
        let svc = HealthService::new();
        assert_eq!(svc.get_status(""), Some(ServingStatus::Serving));
    }

    /// @covers: set_status — registers new service.
    #[test]
    fn test_set_status_registers_new_named_service() {
        let svc = HealthService::new();
        svc.set_status("pkg.A", ServingStatus::Serving);
        assert_eq!(svc.get_status("pkg.A"), Some(ServingStatus::Serving));
    }

    /// @covers: set_overall_status — updates overall slot.
    #[test]
    fn test_set_overall_status_updates_empty_service_slot() {
        let svc = HealthService::new();
        svc.set_overall_status(ServingStatus::NotServing);
        assert_eq!(svc.get_status(""), Some(ServingStatus::NotServing));
    }

    /// @covers: get_status — unknown service returns None.
    #[test]
    fn test_get_status_returns_none_for_unregistered_service() {
        assert!(HealthService::new().get_status("unknown").is_none());
    }

    /// @covers: subscribe — receiver gets subsequent updates.
    #[tokio::test]
    async fn test_subscribe_receives_subsequent_status_changes() {
        let svc = HealthService::new();
        let mut rx = svc.subscribe();
        svc.set_status("pkg.A", ServingStatus::NotServing);
        let (name, status) = rx.recv().await.expect("must receive update");
        assert_eq!(name, "pkg.A");
        assert_eq!(status, ServingStatus::NotServing);
    }

    /// @covers: recompute_overall_status — all serving yields SERVING.
    #[test]
    fn test_recompute_overall_status_all_serving_yields_serving() {
        let svc = HealthService::new();
        svc.set_status("pkg.A", ServingStatus::Serving);
        svc.set_status("pkg.B", ServingStatus::Serving);
        svc.recompute_overall_status();
        assert_eq!(svc.get_status(""), Some(ServingStatus::Serving));
    }
}
