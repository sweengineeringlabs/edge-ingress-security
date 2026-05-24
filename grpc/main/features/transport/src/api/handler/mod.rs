//! Handler adapter and dispatcher.

pub(crate) mod decode_fn;
pub(crate) mod encode_fn;
pub(crate) mod grpc;

pub use crate::api::types::grpc::{GrpcHandlerAdapter, GrpcHandlerRegistryDispatcher};
pub use decode_fn::DecodeFn;
pub use encode_fn::EncodeFn;
