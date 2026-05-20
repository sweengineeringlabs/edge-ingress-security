//! Gateway layer — public surface for the gRPC inbound transport crate.
pub(crate) mod input;
pub(crate) mod output;

pub use input::*;
