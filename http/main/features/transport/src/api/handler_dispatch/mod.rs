//! Handler dispatch interface counterpart — types consumed by the core dispatcher.
//!
//! This module re-exports the dispatcher and error types declared in `api::handler::http`
//! so that `core::handler_dispatch` has a matching API counterpart directory
//! per SEA Rule 121.
pub(crate) mod registry_dispatcher_impl;
