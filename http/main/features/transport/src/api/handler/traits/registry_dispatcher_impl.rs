//! Interface contract for the registry-backed HTTP dispatcher.
//!
//! This file is the api/ counterpart to `core::handler_dispatch::registry_dispatcher_impl`
//! per SEA Rule 121. It declares the marker trait that the core implementation
//! fulfills.

/// Marker trait for types that implement the registry-dispatcher contract.
///
/// The `HttpHandlerRegistryDispatcher` in `core/` must implement this trait
/// to satisfy the SEA interface–implementation pairing.
#[expect(
    dead_code,
    reason = "SEA api/ interface anchor (Rule 121) — intentionally unused"
)]
pub trait RegistryDispatcherImpl: Send + Sync {}
