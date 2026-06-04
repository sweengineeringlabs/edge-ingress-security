#![allow(clippy::module_inception)]
//! Trait definitions for the gRPC auth-mtls interceptor.
pub mod processor;
pub mod traits;
pub use processor::Processor;
pub use traits::Validator;
