//! SEA primary processing contract.

/// Primary processing contract for authz interceptors.
// The Processor marker trait is mandated by SEA Rule 154.  It has no methods
// so the compiler considers it dead_code; suppressing the lint is intentional.
#[allow(dead_code)]
pub trait Processor: Send + Sync {}
