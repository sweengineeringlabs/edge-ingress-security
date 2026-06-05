//! API layer — HTTP domain types and port.
pub(crate) mod error;
pub(crate) mod handler;
pub(crate) mod server;
pub mod traits;
pub(crate) mod types;
pub(crate) mod validator;
pub(crate) mod value;

pub use traits::{HttpIngress, HttpStream};
