//! Middleware traits for request/response pipeline processing.

use futures::future::BoxFuture;
use crate::api::ingress_error::IngressError;

/// Action returned by request middleware.
pub enum MiddlewareAction<Req, Resp> {
    Continue(Req),
    ShortCircuit(Resp),
}

/// Middleware that intercepts inbound requests.
pub trait RequestMiddleware<
    Req: Send + Sync + 'static = serde_json::Value,
    Err: Send + Sync + 'static = IngressError,
    Resp: Send + Sync + 'static = serde_json::Value,
>: Send + Sync {
    fn process_request(&self, request: Req) -> BoxFuture<'_, Result<Req, Err>>;

    fn process_request_action(
        &self,
        request: Req,
    ) -> BoxFuture<'_, Result<MiddlewareAction<Req, Resp>, Err>> {
        Box::pin(async move {
            self.process_request(request)
                .await
                .map(MiddlewareAction::Continue)
        })
    }
}

/// Middleware that intercepts outbound responses.
pub trait ResponseMiddleware<
    Resp: Send + Sync + 'static = serde_json::Value,
    Err: Send + Sync + 'static = IngressError,
>: Send + Sync {
    fn process_response(&self, response: Resp) -> BoxFuture<'_, Result<Resp, Err>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_middleware_is_object_safe() {
        fn _accepts(_m: &dyn RequestMiddleware) {}
    }

    #[test]
    fn test_response_middleware_is_object_safe() {
        fn _accepts(_m: &dyn ResponseMiddleware) {}
    }
}
