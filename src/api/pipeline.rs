//! Pipeline and Router traits.

use futures::future::BoxFuture;
use crate::api::ingress_error::IngressError;

/// Router — dispatches a request to produce a response.
pub trait Router<
    Req: Send + Sync + 'static = serde_json::Value,
    Resp: Send + Sync + 'static = serde_json::Value,
    Err: Send + Sync + 'static = IngressError,
>: Send + Sync {
    fn dispatch<'a>(&'a self, request: &'a Req) -> BoxFuture<'a, Result<Resp, Err>>;
}

/// Pipeline — executes a request through an ordered chain of stages.
pub trait Pipeline<
    Req: Send + Sync + 'static = serde_json::Value,
    Resp: Send + Sync + 'static = serde_json::Value,
    Err: Send + Sync + 'static = IngressError,
>: Send + Sync {
    fn execute(&self, request: Req) -> BoxFuture<'_, Result<Resp, Err>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_router_is_object_safe() {
        fn _accepts(_r: &dyn Router) {}
    }

    #[test]
    fn test_pipeline_is_object_safe() {
        fn _accepts(_p: &dyn Pipeline) {}
    }
}
