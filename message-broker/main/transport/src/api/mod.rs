//! API layer — ingress message consumer port contracts.
#[cfg(feature = "in-memory")]
pub(crate) mod default;
pub(crate) mod error;
#[cfg(feature = "nats")]
pub(crate) mod nats;
pub(crate) mod port;
pub(crate) mod traits;
pub(crate) mod types;
