//! SAF layer — public facade.

mod reflection_svc;

pub use crate::api::error::ReflectionError;
pub use crate::api::types::reflection_service::{
    ReflectionService, ERROR_CODE_INVALID_ARGUMENT, ERROR_CODE_NOT_FOUND, ERROR_CODE_UNIMPLEMENTED,
    REFLECTION_INFO_METHOD, REFLECTION_SERVICE_NAME,
};
pub use crate::api::types::ApplicationConfigBuilder;
pub use crate::api::types::{Descriptor, ReflectionRequest, ReflectionResponse};
pub use crate::api::wire::ReflectionCodec;
pub use reflection_svc::{
    create_config_builder, handle_reflection, validate_payload, validate_with,
};
