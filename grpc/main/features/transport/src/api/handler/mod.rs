//! Handler adapter and dispatcher.

pub(crate) mod decode_fn;
pub(crate) mod encode_fn;
pub(crate) mod grpc;

pub use decode_fn::DecodeFn;
pub use encode_fn::EncodeFn;
pub use grpc::{GrpcHandlerAdapter, GrpcHandlerRegistryDispatcher};
