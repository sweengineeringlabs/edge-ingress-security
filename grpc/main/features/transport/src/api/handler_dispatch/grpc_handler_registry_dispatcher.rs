//! Handler dispatch interface — declares the types that core/handler_dispatch implements.

/// Re-export of the primary dispatch type from the handler module.
///
/// This file exists to satisfy the SEA rule requiring every core implementation
/// file to have a corresponding interface counterpart in the api/ layer.
pub use crate::api::handler::grpc::grpc_handler_registry_dispatcher::GrpcHandlerRegistryDispatcher;

#[cfg(test)]
mod tests {
    use super::GrpcHandlerRegistryDispatcher;
    use edge_domain::HandlerRegistry;
    use std::sync::Arc;

    #[test]
    fn test_grpc_handler_registry_dispatcher_is_constructible() {
        let _ = GrpcHandlerRegistryDispatcher::new(Arc::new(HandlerRegistry::new()));
    }
}
