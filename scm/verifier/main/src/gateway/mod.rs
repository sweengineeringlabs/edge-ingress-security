//! Gateway layer — inbound and outbound integration boundaries for credential verification.

pub(crate) mod egress;
pub(crate) mod ingress;

pub use crate::saf::*;
