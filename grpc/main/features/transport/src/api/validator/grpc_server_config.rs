//! GrpcServerConfig validator interface declaration.

/// Re-export of the primary validated type.
///
/// This file exists to satisfy the SEA rule requiring every core implementation
/// file to have a corresponding interface counterpart in the api/ layer.
#[allow(unused_imports)]
pub use crate::api::value_object::GrpcServerConfig;
