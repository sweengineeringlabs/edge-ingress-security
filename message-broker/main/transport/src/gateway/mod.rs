//! Gateway layer — public surface for the ingress message-broker transport crate.
pub(crate) mod egress;
pub(crate) mod ingress;

pub use ingress::*;
