//! Aggregate dispatcher tying HealthService to a GrpcIngress dispatcher.

use std::sync::Arc;

use crate::api::port::grpc::GrpcIngress;

use super::super::serving_status::ServingStatus;
use super::health_service::HealthService;

/// Aggregate dispatcher — ties [`HealthService`] to a registry-backed
/// dispatcher so the overall service-name reflects the health of every
/// registered handler.
pub struct HealthAggregate {
    pub(crate) service: Arc<HealthService>,
    pub(crate) dispatcher: Arc<dyn GrpcIngress>,
}

impl HealthAggregate {
    /// Bind a [`HealthService`] to a dispatcher.
    pub fn new(service: Arc<HealthService>, dispatcher: Arc<dyn GrpcIngress>) -> Self {
        Self {
            service,
            dispatcher,
        }
    }

    /// Re-poll the dispatcher and update the overall service status.
    pub async fn refresh(&self) {
        let h = self.dispatcher.health_check().await;
        let status = match h {
            Ok(c) if c.healthy => ServingStatus::Serving,
            _ => ServingStatus::NotServing,
        };
        self.service.set_overall_status(status);
    }
}
