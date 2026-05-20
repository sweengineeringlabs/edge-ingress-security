//! gRPC handler adapter and dispatcher types.

pub(crate) mod grpc_handler_adapter;
pub(crate) mod grpc_handler_registry_dispatcher;

pub use grpc_handler_adapter::GrpcHandlerAdapter;
pub use grpc_handler_registry_dispatcher::GrpcHandlerRegistryDispatcher;
