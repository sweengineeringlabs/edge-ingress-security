//! API layer — HTTP domain types and port.
pub(crate) mod error;
pub mod traits;
pub(crate) mod types;
pub(crate) mod vo;

pub use traits::{HttpIngress, HttpStream};
