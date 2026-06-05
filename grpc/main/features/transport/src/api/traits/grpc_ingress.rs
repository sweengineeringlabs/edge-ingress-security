//! Handles inbound gRPC requests (server-side).

use futures::future::BoxFuture;

use edge_domain::RequestContext;

use crate::api::value::{GrpcMetadata, GrpcRequest, GrpcResponse};

use crate::api::types::GrpcIngressResult;
use crate::api::types::GrpcMessageStream;
use crate::api::types::GrpcHealthCheck;

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
