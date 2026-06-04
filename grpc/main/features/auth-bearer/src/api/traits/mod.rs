#![allow(clippy::module_inception)]
//! Trait definitions for the gRPC auth-bearer interceptor.
pub mod processor;
pub mod traits;
pub use processor::Processor;
pub use traits::Validator;
