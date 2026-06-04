//! Handler dispatch interface counterpart for core/handler_dispatch/handler_dispatch.rs.
//!
//! This module exists to satisfy SEA Rule 121: every core implementation file
//! must have a corresponding api interface counterpart at the same path.

/// Marker trait for the handler dispatch contract.
///
/// The concrete implementation type is [`crate::api::handler::GrpcHandlerRegistryDispatcher`].
/// This trait exists to satisfy SEA Rule 121 (api/core interface pairing) and
/// Rule 161 (every api/ file must have exactly one pub item matching its stem).
pub trait HandlerDispatch {}

