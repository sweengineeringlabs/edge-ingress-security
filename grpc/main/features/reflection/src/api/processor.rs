//! `Processor` — core processing contract for the gRPC reflection service.

use futures::future::BoxFuture;

use crate::api::reflection::reflection_request::ReflectionRequest;
use crate::api::reflection::reflection_response::ReflectionResponse;

/// Core processing contract for the gRPC reflection service.
///
/// Implementations translate an inbound [`ReflectionRequest`] into a
/// [`ReflectionResponse`] sent back to grpcurl / evans.
#[allow(dead_code)]
pub trait Processor: Send + Sync {
    /// Process a single reflection request and return the appropriate response.
    fn process<'a>(&'a self, request: ReflectionRequest) -> BoxFuture<'a, ReflectionResponse>;
}
