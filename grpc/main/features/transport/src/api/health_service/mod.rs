//! Health service declarations.
//!
//! Re-exports from [`crate::api::types::health`] and [`crate::api::types::serving_status`].

pub use crate::api::types::health::{
    HealthAggregate, HealthService, HEALTH_CHECK_METHOD, HEALTH_WATCH_METHOD,
    WATCH_CHANNEL_CAPACITY,
};
pub use crate::api::types::serving_status::ServingStatus;
