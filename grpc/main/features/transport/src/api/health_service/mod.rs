//! Health service declarations.

pub(crate) mod health;
#[allow(clippy::module_inception)]
pub(crate) mod health_service;
pub(crate) mod serving_status;

pub use health::{
    HealthAggregate, HealthService, HEALTH_CHECK_METHOD, HEALTH_WATCH_METHOD,
    WATCH_CHANNEL_CAPACITY,
};
pub use serving_status::ServingStatus;
