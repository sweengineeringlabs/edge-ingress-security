//! API layer — HTTP domain types and port.
pub(crate) mod error;
pub mod handler;
pub mod server;
pub mod traits;
pub(crate) mod types;
pub mod validator;
pub(crate) mod vo;

pub use traits::{HttpIngress, HttpStream};
