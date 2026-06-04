//! Implementation of the standard `grpc.health.v1.Health` service.

use std::collections::HashMap;

use parking_lot::RwLock;
use tokio::sync::broadcast;

use super::super::serving_status::ServingStatus;

/// gRPC method path for health-check unary calls.
pub const HEALTH_CHECK_METHOD: &str = "/grpc.health.v1.Health/Check";
/// gRPC method path for health-watch streaming calls.
pub const HEALTH_WATCH_METHOD: &str = "/grpc.health.v1.Health/Watch";
/// Broadcast channel capacity for status-change notifications.
pub const WATCH_CHANNEL_CAPACITY: usize = 16;

/// Implementation of the standard `grpc.health.v1.Health` service.
pub struct HealthService {
    pub(crate) statuses: RwLock<HashMap<String, ServingStatus>>,
    pub(crate) broadcaster: broadcast::Sender<(String, ServingStatus)>,
}

impl HealthService {
    /// Construct an empty service. Overall service starts as [`ServingStatus::Serving`].
    pub fn new() -> Self {
        let (tx, _rx) = broadcast::channel(WATCH_CHANNEL_CAPACITY);
        let mut statuses = HashMap::new();
        statuses.insert(String::new(), ServingStatus::Serving);
        Self {
            statuses: RwLock::new(statuses),
            broadcaster: tx,
        }
    }

    /// Set the status for a named service.
    pub fn set_status(&self, service: impl Into<String>, status: ServingStatus) {
        let service = service.into();
        {
            let mut guard = self.statuses.write();
            guard.insert(service.clone(), status);
        }
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
            if named.is_empty() || named.iter().all(|(_, s)| **s == ServingStatus::Serving) {
                ServingStatus::Serving
            } else {
                ServingStatus::NotServing
            }
        };
        self.set_overall_status(new_overall);
    }
}

impl Default for HealthService {
    fn default() -> Self {
        Self::new()
    }
}
