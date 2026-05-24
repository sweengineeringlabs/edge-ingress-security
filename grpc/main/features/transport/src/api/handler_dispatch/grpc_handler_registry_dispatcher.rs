//! Handler dispatch interface — declares the types that core/handler_dispatch implements.

#[cfg(test)]
mod tests {
    use crate::api::types::grpc::GrpcHandlerRegistryDispatcher;
    use edge_domain::HandlerRegistry;
    use std::sync::Arc;

    #[test]
    fn test_grpc_handler_registry_dispatcher_is_constructible() {
        let _ = GrpcHandlerRegistryDispatcher::new(Arc::new(HandlerRegistry::new()));
    }
}
