//! Handles inbound gRPC requests (server-side).

use futures::future::BoxFuture;

use edge_domain::RequestContext;

use crate::api::value_object::{GrpcMetadata, GrpcRequest, GrpcResponse};

use super::grpc_ingress_result::GrpcIngressResult;
use super::grpc_message_stream::GrpcMessageStream;
use crate::api::port::GrpcHealthCheck;

/// Handles inbound gRPC requests (server-side).
pub trait GrpcIngress: Send + Sync {
    /// Handle a single unary request.
    fn handle_unary(
        &self,
        request: GrpcRequest,
        ctx: RequestContext,
    ) -> BoxFuture<'_, GrpcIngressResult<GrpcResponse>>;

    /// Handle a streaming request (client-streaming, server-streaming, or bidi).
    ///
    /// Returns the response stream **and** any response metadata (trailers) as a
    /// tuple `(stream, metadata)`. The server threads the metadata into HTTP/2
    /// trailers alongside `grpc-status: 0`.
    ///
    /// The default implementation reads the first message from the input stream,
    /// forwards it to [`handle_unary`], and wraps the response in a single-item
    /// output stream, preserving the response metadata.  Implementors that need
    /// true streaming override this method.
    ///
    /// [`handle_unary`]: GrpcIngress::handle_unary
    fn handle_stream(
        &self,
        method: String,
        metadata: GrpcMetadata,
        messages: GrpcMessageStream,
        ctx: RequestContext,
    ) -> BoxFuture<'_, GrpcIngressResult<(GrpcMessageStream, GrpcMetadata)>> {
        Box::pin(async move {
            use futures::StreamExt;
            let mut messages = messages;
            let body = match messages.next().await {
                Some(Ok(b)) => b,
                Some(Err(e)) => return Err(e),
                None => vec![],
            };
            let req = GrpcRequest::new(method, body, std::time::Duration::from_secs(30))
                .with_metadata(metadata);
            let resp = self.handle_unary(req, ctx).await?;
            let out: GrpcMessageStream =
                Box::pin(futures::stream::once(futures::future::ready(Ok(resp.body))));
            Ok((out, resp.metadata))
        })
    }

    /// Handle a server-streaming request — single request, streaming response.
    ///
    /// The default implementation delegates to [`handle_unary`] and wraps the
    /// response in a single-item stream.
    ///
    /// [`handle_unary`]: GrpcIngress::handle_unary
    fn handle_server_stream(
        &self,
        request: GrpcRequest,
        ctx: RequestContext,
    ) -> BoxFuture<'_, GrpcIngressResult<GrpcMessageStream>> {
        Box::pin(async move {
            let resp = self.handle_unary(request, ctx).await?;
            let out: GrpcMessageStream =
                Box::pin(futures::stream::once(futures::future::ready(Ok(resp.body))));
            Ok(out)
        })
    }

    /// Handle a client-streaming request — streaming request messages, single response.
    ///
    /// The default implementation reads the first message from the stream and
    /// delegates to [`handle_unary`].
    ///
    /// [`handle_unary`]: GrpcIngress::handle_unary
    fn handle_client_stream(
        &self,
        method: String,
        metadata: GrpcMetadata,
        messages: GrpcMessageStream,
        ctx: RequestContext,
    ) -> BoxFuture<'_, GrpcIngressResult<GrpcResponse>> {
        Box::pin(async move {
            use futures::StreamExt;
            let mut messages = messages;
            let body = match messages.next().await {
                Some(Ok(b)) => b,
                Some(Err(e)) => return Err(e),
                None => vec![],
            };
            let req = GrpcRequest::new(method, body, std::time::Duration::from_secs(30))
                .with_metadata(metadata);
            self.handle_unary(req, ctx).await
        })
    }

    /// Handle a bidirectional-streaming request — streaming in both directions.
    ///
    /// The default implementation delegates to [`handle_stream`].
    ///
    /// [`handle_stream`]: GrpcIngress::handle_stream
    fn handle_bidi_stream(
        &self,
        method: String,
        metadata: GrpcMetadata,
        messages: GrpcMessageStream,
        ctx: RequestContext,
    ) -> BoxFuture<'_, GrpcIngressResult<(GrpcMessageStream, GrpcMetadata)>> {
        self.handle_stream(method, metadata, messages, ctx)
    }

    /// Perform a health check.
    fn health_check(&self) -> BoxFuture<'_, GrpcIngressResult<GrpcHealthCheck>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct UnaryOnlyHandler;
    impl GrpcIngress for UnaryOnlyHandler {
        fn handle_unary(
            &self,
            _request: GrpcRequest,
            _ctx: RequestContext,
        ) -> BoxFuture<'_, GrpcIngressResult<GrpcResponse>> {
            Box::pin(futures::future::ready(Ok(GrpcResponse {
                body: b"pong".to_vec(),
                metadata: GrpcMetadata::default(),
            })))
        }
        fn health_check(&self) -> BoxFuture<'_, GrpcIngressResult<GrpcHealthCheck>> {
            Box::pin(futures::future::ready(Ok(GrpcHealthCheck::healthy())))
        }
    }

    #[test]
    fn test_grpc_ingress_is_object_safe() {
        fn _assert_object_safe(_: &dyn GrpcIngress) {}
    }

    #[tokio::test]
    async fn test_handle_server_stream_default_wraps_unary_response_in_single_item_stream() {
        use futures::StreamExt;
        let h = UnaryOnlyHandler;
        let req = GrpcRequest::new(
            "svc/M".to_string(),
            vec![],
            std::time::Duration::from_secs(1),
        );
        let mut stream = h
            .handle_server_stream(req, RequestContext::default())
            .await
            .expect("should succeed");
        let frame = stream.next().await.expect("stream must yield one item");
        assert_eq!(frame.unwrap(), b"pong");
        assert!(stream.next().await.is_none());
    }

    #[tokio::test]
    async fn test_handle_client_stream_default_reads_first_message_and_calls_unary() {
        use futures::stream;
        let h = UnaryOnlyHandler;
        let messages: GrpcMessageStream =
            Box::pin(stream::once(futures::future::ready(Ok(b"ping".to_vec()))));
        let resp = h
            .handle_client_stream(
                "svc/M".into(),
                GrpcMetadata::default(),
                messages,
                RequestContext::default(),
            )
            .await
            .expect("should succeed");
        assert_eq!(resp.body, b"pong");
    }

    #[tokio::test]
    async fn test_handle_bidi_stream_default_delegates_to_handle_stream() {
        use futures::stream;
        let h = UnaryOnlyHandler;
        let messages: GrpcMessageStream =
            Box::pin(stream::once(futures::future::ready(Ok(b"ping".to_vec()))));
        let (mut out, _meta) = h
            .handle_bidi_stream(
                "svc/M".into(),
                GrpcMetadata::default(),
                messages,
                RequestContext::default(),
            )
            .await
            .expect("should succeed");
        use futures::StreamExt;
        let frame = out.next().await.expect("must yield one item").unwrap();
        assert_eq!(frame, b"pong");
    }

    #[tokio::test]
    async fn test_handle_stream_default_reads_first_message_and_wraps_response() {
        use futures::stream;
        let h = UnaryOnlyHandler;
        let messages: GrpcMessageStream =
            Box::pin(stream::once(futures::future::ready(Ok(b"ping".to_vec()))));
        let (mut out, _meta) = h
            .handle_stream(
                "svc/M".into(),
                GrpcMetadata::default(),
                messages,
                RequestContext::default(),
            )
            .await
            .expect("should succeed");
        use futures::StreamExt;
        let frame = out.next().await.expect("must yield one item").unwrap();
        assert_eq!(frame, b"pong");
    }
}
