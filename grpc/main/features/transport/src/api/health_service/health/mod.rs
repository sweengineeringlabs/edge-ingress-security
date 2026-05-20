//! Health service and aggregate types.

pub(crate) mod health_aggregate;
pub(crate) mod health_service;

pub use health_aggregate::HealthAggregate;
pub use health_service::{
    HealthService, HEALTH_CHECK_METHOD, HEALTH_WATCH_METHOD, WATCH_CHANNEL_CAPACITY,
};
