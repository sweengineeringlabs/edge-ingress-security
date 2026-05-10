//! Reflection service declarations, constants, and builder methods.

use std::sync::Arc;

use edge_domain::HandlerRegistry;
use parking_lot::RwLock;

use crate::api::types::Descriptor;

pub const REFLECTION_INFO_METHOD: &str =
    "/grpc.reflection.v1alpha.ServerReflection/ServerReflectionInfo";

pub const REFLECTION_SERVICE_NAME: &str = "grpc.reflection.v1alpha.ServerReflection";

pub const ERROR_CODE_NOT_FOUND:         i32 = 5;
pub const ERROR_CODE_UNIMPLEMENTED:     i32 = 12;
pub const ERROR_CODE_INVALID_ARGUMENT:  i32 = 3;

/// Implementation of `grpc.reflection.v1alpha.ServerReflection`.
pub struct ReflectionService {
    pub(crate) registry:    Arc<HandlerRegistry<Vec<u8>, Vec<u8>>>,
    pub(crate) descriptors: RwLock<Vec<Descriptor>>,
}

impl ReflectionService {
    /// Construct a reflection service backed by `registry`.
    pub fn new(registry: Arc<HandlerRegistry<Vec<u8>, Vec<u8>>>) -> Self {
        Self { registry, descriptors: RwLock::new(Vec::new()) }
    }

    /// Register a parsed descriptor for `FileByFilename` / `FileContainingSymbol` lookups.
    pub fn add_descriptor(self, descriptor: Descriptor) -> Self {
        self.descriptors.write().push(descriptor);
        self
    }

    /// Register a list of descriptors in one call.
    pub fn with_descriptors(self, descriptors: impl IntoIterator<Item = Descriptor>) -> Self {
        { let mut w = self.descriptors.write(); for d in descriptors { w.push(d); } }
        self
    }
}

/// Extract the service name from a `/pkg.Service/Method` path.
pub fn service_name_from_method_path(path: &str) -> Option<&str> {
    let path = path.strip_prefix('/')?;
    let slash = path.find('/')?;
    let name = &path[..slash];
    if name.is_empty() { return None; }
    Some(name)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use edge_domain::HandlerRegistry;
    use super::*;

    /// @covers: REFLECTION_INFO_METHOD — correct gRPC path format.
    #[test]
    fn test_reflection_info_method_has_correct_grpc_path_format() {
        assert!(REFLECTION_INFO_METHOD.starts_with('/'));
        assert!(REFLECTION_INFO_METHOD.contains("ServerReflectionInfo"));
    }

    /// @covers: ReflectionService::new — creates empty service.
    #[test]
    fn test_new_creates_reflection_service_with_empty_registry() {
        let svc = ReflectionService::new(Arc::new(HandlerRegistry::new()));
        assert!(svc.descriptors.read().is_empty());
    }

    /// @covers: add_descriptor — adds a descriptor.
    #[test]
    fn test_add_descriptor_appends_to_descriptors_list() {
        let svc = ReflectionService::new(Arc::new(HandlerRegistry::new()))
            .add_descriptor(Descriptor {
                filename: "test.proto".into(),
                bytes: vec![],
                symbols: vec![],
            });
        assert_eq!(svc.descriptors.read().len(), 1);
    }
}
