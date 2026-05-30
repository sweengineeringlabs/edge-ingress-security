//! Extension hooks for downstream verifier consumers.

mod noop_verifier_extension;

pub(crate) use noop_verifier_extension::DefaultNoopVerifierExtension;
