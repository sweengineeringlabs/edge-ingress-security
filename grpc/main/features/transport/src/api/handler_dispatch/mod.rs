//! Handler dispatch interface — declares the types that core/handler_dispatch implements.

pub(crate) mod grpc_handler_registry_dispatcher;
#[allow(clippy::module_inception)]
pub(crate) mod handler_dispatch;

#[allow(unused_imports)]
pub use grpc_handler_registry_dispatcher::GrpcHandlerRegistryDispatcher;
