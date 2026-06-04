#![allow(clippy::module_inception)]
//! Trait definitions for the gRPC reflection service.
pub mod processor;
pub mod traits;
pub use traits::Validator;
