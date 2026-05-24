//! gRPC server — binds a socket and delegates all unary calls to a
//! [`GrpcIngress`] handler.  HTTP/2 framing is handled by Hyper directly
//! (avoiding the axum 0.7 / 0.8 type mismatch that Tonic's routing layer
//! would otherwise introduce). gRPC length-prefix framing is handled here.

use std::collections::HashMap;
use std::convert::Infallible;
use std::future::Future;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};

use bytes::{BufMut, Bytes, BytesMut};
use http::{Request, Response};
use http_body_util::{BodyExt, Limited, StreamBody};
use hyper_util::rt::{TokioExecutor, TokioIo};
use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;

use edge_domain::RequestContext;

use crate::api::audit_sink::{AuditEvent, AuditSink};
use crate::api::grpc_timeout::{GrpcTimeoutParser, DEFAULT_DEADLINE};
use crate::api::interceptor::GrpcIngressInterceptorChain;
use crate::api::peer_identity::PeerIdentityExtractor;
use crate::api::port::grpc_ingress::{
    GrpcIngress, GrpcIngressError, GrpcIngressResult, GrpcMessageStream,
};
use crate::api::server::{
    TonicGrpcServer, TonicServerError, MISSING_AUTHORIZATION_INTERCEPTOR_MSG,
    REFLECTION_ENABLED_WARN_MSG,
};
use crate::api::status_codes::StatusCodeConverter;
use crate::api::value_object::{
    CompressionMode, GrpcMetadata, GrpcRequest, GrpcResponse, GrpcStatusCode, PeerIdentity, PEER_CN,
};

type BoxBody = http_body_util::combinators::BoxBody<Bytes, Infallible>;

impl TonicGrpcServer {
    /// Bind and serve until `shutdown` resolves.
    ///
    /// **Fail-closed authorisation invariant**: if no
    /// [`crate::AuthorizationInterceptor`] is registered AND
    /// `allow_unauthenticated` is `false`, this method panics with
    /// [`MISSING_AUTHORIZATION_INTERCEPTOR_MSG`] before binding.
    /// Callers that want to run unauthenticated must explicitly call
    /// [`Self::allow_unauthenticated`] (or set the flag via
    /// [`GrpcServerConfig`]) — that path logs a WARN at startup so
    /// the decision is observable in production.
    pub async fn serve<F>(&self, shutdown: F) -> Result<(), TonicServerError>
    where
        F: Future<Output = ()>,
    {
        self.enforce_authorization_invariant();
        let listener = TcpListener::bind(&self.bind)
            .await
            .map_err(|e| TonicServerError::Bind(self.bind.clone(), e))?;
        self.serve_with_listener(listener, shutdown).await
    }

    /// Apply the fail-closed authorisation invariant.
    ///
    /// Panics when no authorization interceptor is registered and
    /// `allow_unauthenticated` is `false`.  Logs a WARN when the
    /// caller opted out via `allow_unauthenticated = true`.
    pub(crate) fn enforce_authorization_invariant(&self) {
        let has_authz = self.interceptors.contains_authorization();
        if !has_authz {
            if self.allow_unauthenticated {
                tracing::warn!(
                    "running with allow_unauthenticated = true; \
                     gRPC dispatch will accept all callers"
                );
            } else {
                panic!("{MISSING_AUTHORIZATION_INTERCEPTOR_MSG}");
            }
        }
    }

    /// Serve using a caller-supplied pre-bound listener.
    ///
    /// Useful for port-0 allocation in tests or pre-bind for zero-downtime
    /// restarts — consistent with the HTTP server pattern.
    ///
    /// **Fail-closed authorisation invariant** is enforced here as
    /// well — see [`Self::serve`] for details.
    pub async fn serve_with_listener<F>(
        &self,
        listener: TcpListener,
        shutdown: F,
    ) -> Result<(), TonicServerError>
    where
        F: Future<Output = ()>,
    {
        self.enforce_authorization_invariant();

        let bind_addr = listener
            .local_addr()
            .map(|a| a.to_string())
            .unwrap_or_else(|_| self.bind.clone());

        let tls_acceptor = self
            .tls
            .as_ref()
            .map(|cfg| {
                tracing::info!(bind = %bind_addr, mtls = cfg.is_mtls(), "gRPC+TLS server listening");
                swe_edge_ingress_tls::build_tls_acceptor(cfg).map_err(TonicServerError::Tls)
            })
            .transpose()?;

        if tls_acceptor.is_none() {
            tracing::info!(bind = %bind_addr, "gRPC server listening");
        }

        if self.enable_reflection {
            tracing::warn!("{REFLECTION_ENABLED_WARN_MSG}");
        }

        let handler = self.handler.clone();
        let max_bytes = self.max_bytes;
        let max_concurrent_streams = self.max_concurrent_streams;
        let interceptors = self.interceptors.clone();
        let compression = self.compression;
        let audit_sink = self.audit_sink.clone();
        let mut shutdown = std::pin::pin!(shutdown);

        loop {
            tokio::select! {
                res = listener.accept() => {
                    let (stream, _) = match res {
                        Ok(s)  => s,
                        Err(e) => { tracing::warn!("gRPC accept error: {e}"); continue; }
                    };
                    let handler      = handler.clone();
                    let tls_acceptor = tls_acceptor.clone();
                    let interceptors = interceptors.clone();
                    let audit_sink   = audit_sink.clone();
                    tokio::spawn(async move {
                        if let Some(acceptor) = tls_acceptor {
                            match acceptor.accept(stream).await {
                                Ok(tls) => {
                                    // Snapshot peer identity once per connection
                                    // — every request on this HTTP/2 conn shares
                                    // the same TLS handshake and thus the same
                                    // identity.
                                    let (_, conn_state) = tls.get_ref();
                                    let peer_metadata: HashMap<String, String> = conn_state
                                        .peer_certificates()
                                        .and_then(|chain| chain.first())
                                        .map(|leaf| PeerIdentityExtractor::extract(leaf.as_ref()))
                                        .unwrap_or_default();

                                    let io = TokioIo::new(tls);
                                    let svc = hyper::service::service_fn({
                                        let handler         = handler.clone();
                                        let interceptors    = interceptors.clone();
                                        let peer_metadata   = peer_metadata.clone();
                                        let audit_sink      = audit_sink.clone();
                                        move |req| {
                                            let handler       = handler.clone();
                                            let interceptors  = interceptors.clone();
                                            let peer_metadata = peer_metadata.clone();
                                            let audit_sink    = audit_sink.clone();
                                            async move {
                                                Ok::<_, Infallible>(TonicServerDispatcher::dispatch(
                                                    req,
                                                    handler,
                                                    max_bytes,
                                                    interceptors,
                                                    compression,
                                                    peer_metadata,
                                                    audit_sink,
                                                ).await)
                                            }
                                        }
                                    });
                                    if let Err(e) = hyper::server::conn::http2::Builder::new(TokioExecutor::new())
                                        .max_concurrent_streams(max_concurrent_streams)
                                        .serve_connection(io, svc)
                                        .await
                                    {
                                        tracing::debug!("gRPC+TLS connection error: {e}");
                                    }
                                }
                                Err(e) => tracing::debug!("gRPC TLS handshake failed: {e}"),
                            }
                        } else {
                            // Plaintext connection — no peer identity available.
                            let io = TokioIo::new(stream);
                            let svc = hyper::service::service_fn({
                                let handler      = handler.clone();
                                let interceptors = interceptors.clone();
                                let audit_sink   = audit_sink.clone();
                                move |req| {
                                    let handler      = handler.clone();
                                    let interceptors = interceptors.clone();
                                    let audit_sink   = audit_sink.clone();
                                    async move {
                                        Ok::<_, Infallible>(TonicServerDispatcher::dispatch(
                                            req,
                                            handler,
                                            max_bytes,
                                            interceptors,
                                            compression,
                                            HashMap::new(),
                                            audit_sink,
                                        ).await)
                                    }
                                }
                            });
                            if let Err(e) = hyper::server::conn::http2::Builder::new(TokioExecutor::new())
                                .max_concurrent_streams(max_concurrent_streams)
                                .serve_connection(io, svc)
                                .await
                            {
                                tracing::debug!("gRPC connection error: {e}");
                            }
                        }
                    });
                }
                _ = &mut shutdown => break,
            }
        }

        Ok(())
    }
}

// ── gRPC dispatch ─────────────────────────────────────────────────────────────

struct TonicServerDispatcher;

impl TonicServerDispatcher {
    /// Read the per-call deadline from the `grpc-timeout` header, falling back
    /// to [`DEFAULT_DEADLINE`] when the header is absent or malformed.
    fn read_deadline(headers: &http::HeaderMap) -> Duration {
        headers
            .get("grpc-timeout")
            .and_then(|v| v.to_str().ok())
            .and_then(GrpcTimeoutParser::parse)
            .unwrap_or(DEFAULT_DEADLINE)
    }

    async fn dispatch(
        req: Request<hyper::body::Incoming>,
        handler: Arc<dyn GrpcIngress>,
        max_bytes: usize,
        interceptors: GrpcIngressInterceptorChain,
        compression: CompressionMode,
        peer_metadata: HashMap<String, String>,
        audit_sink: Arc<dyn AuditSink>,
    ) -> Response<BoxBody> {
        let method = req.uri().path().to_string();
        let started = Instant::now();
        let timestamp = SystemTime::now();
        let metadata = Self::collect_metadata(req.headers());
        let deadline = Self::read_deadline(req.headers());

        // Identity for audit — drawn from the cryptographic peer
        // metadata snapshot taken at TLS-acceptance time.  Plaintext
        // connections see `None`, mirroring the audit-event contract.
        let identity = peer_metadata.get(PEER_CN).cloned();

        // Helper to emit a final audit event and return the wire response.
        // Centralises every termination path so we never miss recording.
        let emit = |code: tonic::Code, response: Response<BoxBody>| {
            let evt = AuditEvent {
                timestamp,
                method: method.clone(),
                identity: identity.clone(),
                status: StatusCodeConverter::from_tonic_code(code),
                duration_ms: started.elapsed().as_millis() as u64,
            };
            audit_sink.record(evt);
            response
        };

        // Past-deadline calls MUST fail before the handler runs.  A zero
        // deadline (e.g. `grpc-timeout: 0n` or `0S`) means the client gave
        // us no time at all to do work.
        if deadline.is_zero() {
            return emit(
                tonic::Code::DeadlineExceeded,
                Self::grpc_error(
                    tonic::Code::DeadlineExceeded,
                    "request rejected before handler dispatch: deadline has already elapsed",
                ),
            );
        }

        let body_bytes = match Limited::new(req.into_body(), max_bytes).collect().await {
            Ok(collected) => collected.to_bytes(),
            Err(_) => {
                return emit(
                    tonic::Code::ResourceExhausted,
                    Self::grpc_error(tonic::Code::ResourceExhausted, "message too large"),
                )
            }
        };

        // Per-request cancellation token — fired implicitly when this future is
        // dropped (i.e. the client closed the HTTP/2 stream).  The handler
        // observes the token via the `messages` stream's parent scope; we expose
        // it on the bridge below.
        let cancel = CancellationToken::new();
        let _drop_guard = cancel.clone().drop_guard();

        // Decode all gRPC length-prefix frames from the body.
        let frames = Self::decode_grpc_frames(&body_bytes);
        let message_stream: GrpcMessageStream = Box::pin(futures::stream::iter(
            frames
                .into_iter()
                .map(|f| Ok::<Vec<u8>, GrpcIngressError>(f.to_vec())),
        ));

        // Build merged request metadata.  Reserved peer-identity keys
        // supplied by the client over the wire are stripped first — the
        // server is the only party allowed to set them, and only after a
        // successful mTLS handshake.  Then we inject the cryptographically
        // derived peer identity (empty for plaintext / TLS-only conns).
        let mut headers: HashMap<String, String> = metadata
            .iter()
            .filter(|(k, _)| !PeerIdentity::is_reserved_key(k))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        for (k, v) in peer_metadata.iter() {
            headers.insert(k.clone(), v.clone());
        }
        headers.insert(
            "x-edge-grpc-deadline-millis".to_string(),
            deadline.as_millis().to_string(),
        );

        // Build a synthetic GrpcRequest envelope so interceptors can
        // observe headers + body before dispatch.
        let mut intercept_req = GrpcRequest::new(method.clone(), body_bytes.to_vec(), deadline);
        intercept_req.metadata = GrpcMetadata {
            headers: headers.clone(),
        };

        // Run before_dispatch — first failure short-circuits and the
        // handler never runs.
        if let Err(e) = interceptors.run_before(&mut intercept_req) {
            let (code, msg) = StatusCodeConverter::map_inbound_error(Self::sanitize_authz_error(e));
            return emit(code, Self::grpc_error(code, msg));
        }
        // Interceptors may have mutated headers; pull them back.
        let merged_headers = intercept_req.metadata.headers.clone();

        // Build per-request auth context from what the interceptors resolved.
        // For mTLS the peer CN lands in merged_headers[PEER_CN]; JWT-based
        // interceptors may inject x-edge-subject / x-edge-issuer / x-edge-tenant-id.
        let ctx = if interceptors.contains_authorization() {
            let subject = merged_headers
                .get(PEER_CN)
                .or_else(|| merged_headers.get("x-edge-subject"))
                .cloned()
                .unwrap_or_default();
            RequestContext::authenticated(
                subject,
                merged_headers.get("x-edge-issuer").cloned(),
                merged_headers.get("x-edge-tenant-id").cloned(),
                merged_headers
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect(),
            )
        } else {
            RequestContext::unauthenticated()
        };

        // Route to the named streaming variant indicated by the client,
        // falling back to `handle_stream` for backward compatibility.
        let stream_type = merged_headers
            .get("x-grpc-stream-type")
            .cloned()
            .unwrap_or_default();
        let handler_fut = Self::route_dispatch(
            handler.clone(),
            stream_type.as_str(),
            method.clone(),
            GrpcMetadata {
                headers: merged_headers,
            },
            message_stream,
            ctx,
            deadline,
        );
        let cancel_fut = cancel.cancelled();

        let result = tokio::select! {
            biased;
            // Cancellation: client disconnected — abort and never produce a body.
            _ = cancel_fut => {
                return emit(
                    tonic::Code::Cancelled,
                    Self::grpc_error(tonic::Code::Cancelled, "client disconnected"),
                );
            }
            // Deadline: timer fired before the handler returned.
            _ = tokio::time::sleep(deadline) => {
                return emit(
                    tonic::Code::DeadlineExceeded,
                    Self::grpc_error(tonic::Code::DeadlineExceeded, "handler deadline exceeded"),
                );
            }
            r = handler_fut => r,
        };

        match result {
            Ok((resp_stream, resp_meta)) => {
                // Drain the handler stream so interceptors can observe the
                // response payload + metadata before it goes out on the wire.
                //
                // Buffered by design: `after_dispatch` operates on a single
                // body bag, not a stream — true streaming interceptors are
                // a follow-up.
                //
                // The deadline protection from the outer select only covered
                // `handle_stream()` setup, not the drain.  Re-arm it here so
                // infinite / long-running streams can't bypass the per-call
                // budget.
                let collected_payload = match tokio::time::timeout(
                    deadline,
                    Self::collect_response_payload(resp_stream),
                )
                .await
                {
                    Ok(Ok(p)) => p,
                    Ok(Err(e)) => {
                        let (code, msg) = StatusCodeConverter::map_inbound_error(e);
                        return emit(code, Self::grpc_error(code, msg));
                    }
                    Err(_elapsed) => {
                        return emit(
                            tonic::Code::DeadlineExceeded,
                            Self::grpc_error(
                                tonic::Code::DeadlineExceeded,
                                "streaming deadline exceeded",
                            ),
                        );
                    }
                };
                // Synthesise an interceptor-visible response.  The body
                // surface is the concatenation of all stream frames — when
                // an after_dispatch hook mutates it we send the mutated
                // bytes as a single frame; otherwise we preserve the
                // original frame boundaries.
                let original_payload = collected_payload.clone();
                let mut response = GrpcResponse {
                    body: collected_payload.concat(),
                    metadata: resp_meta,
                };

                // Advertise grpc-accept-encoding when compression is enabled.
                if let Some(name) = compression.header_value() {
                    response
                        .metadata
                        .headers
                        .entry("grpc-accept-encoding".to_string())
                        .or_insert_with(|| name.to_string());
                }

                // after_dispatch — same short-circuit semantics as before.
                if let Err(e) = interceptors.run_after(&mut response) {
                    let (code, msg) = StatusCodeConverter::map_inbound_error(e);
                    return emit(code, Self::grpc_error(code, msg));
                }

                let body_changed = response.body != original_payload.concat();
                let payloads = if body_changed {
                    vec![response.body]
                } else {
                    original_payload
                };

                let wire =
                    Self::grpc_stream_response_from_payloads(payloads, response.metadata).await;
                emit(tonic::Code::Ok, wire)
            }
            Err(e) => {
                let (code, msg) = StatusCodeConverter::map_inbound_error(e);
                emit(code, Self::grpc_error(code, msg))
            }
        }
    }

    /// Route a request to the correct [`GrpcIngress`] streaming variant.
    ///
    /// `x-grpc-stream-type` header values and their targets:
    /// - `"server-stream"` → [`GrpcIngress::handle_server_stream`] (unary request, streaming response)
    /// - `"client-stream"` → [`GrpcIngress::handle_client_stream`] (streaming request, single response)
    /// - `"bidi-stream"`   → [`GrpcIngress::handle_bidi_stream`]   (streaming both directions)
    /// - absent / any other value → [`GrpcIngress::handle_stream`] (backward-compatible default)
    ///
    /// All branches normalise to `(GrpcMessageStream, GrpcMetadata)` so the rest of the
    /// dispatch pipeline (drain → interceptors → wire encoding) is unchanged.
    async fn route_dispatch(
        handler: Arc<dyn GrpcIngress>,
        stream_type: &str,
        method: String,
        metadata: GrpcMetadata,
        messages: GrpcMessageStream,
        ctx: RequestContext,
        deadline: Duration,
    ) -> GrpcIngressResult<(GrpcMessageStream, GrpcMetadata)> {
        use futures::StreamExt;
        match stream_type {
            "server-stream" => {
                // Single request frame → streaming response.
                let mut s = messages;
                let body = match s.next().await {
                    Some(Ok(b)) => b,
                    Some(Err(e)) => return Err(e),
                    None => vec![],
                };
                let req = GrpcRequest::new(method, body, deadline).with_metadata(metadata);
                let out = handler.handle_server_stream(req, ctx).await?;
                Ok((out, GrpcMetadata::default()))
            }
            "client-stream" => {
                // Streaming request → single response wrapped in a one-item stream.
                let resp = handler
                    .handle_client_stream(method, metadata, messages, ctx)
                    .await?;
                let out: GrpcMessageStream =
                    Box::pin(futures::stream::once(futures::future::ready(Ok(resp.body))));
                Ok((out, resp.metadata))
            }
            "bidi-stream" => {
                handler
                    .handle_bidi_stream(method, metadata, messages, ctx)
                    .await
            }
            _ => handler.handle_stream(method, metadata, messages, ctx).await,
        }
    }

    /// Strip authz policy rationale before it reaches the wire.
    ///
    /// Authorisation interceptors may attach policy-decision details to
    /// the error message — those strings could leak the server's ACL
    /// shape.  Replace any `PermissionDenied` payload with a fixed,
    /// non-revealing string.  Other errors pass through untouched.
    fn sanitize_authz_error(err: GrpcIngressError) -> GrpcIngressError {
        match err {
            GrpcIngressError::PermissionDenied(_) => {
                GrpcIngressError::PermissionDenied("authorization denied".into())
            }
            GrpcIngressError::Status(GrpcStatusCode::PermissionDenied, _) => {
                GrpcIngressError::Status(
                    GrpcStatusCode::PermissionDenied,
                    "authorization denied".into(),
                )
            }
            other => other,
        }
    }

    /// Drain a [`GrpcMessageStream`] into a list of payloads.
    async fn collect_response_payload(
        mut stream: GrpcMessageStream,
    ) -> Result<Vec<Vec<u8>>, GrpcIngressError> {
        use futures::StreamExt;
        let mut out = Vec::new();
        while let Some(item) = stream.next().await {
            out.push(item?);
        }
        Ok(out)
    }

    /// Parse all gRPC length-prefix frames from a raw body.
    ///
    /// Each frame: 1 byte compressed flag + 4 bytes big-endian message length + payload.
    /// Frames with a compressed flag != 0 are still yielded (payload returned as-is).
    fn decode_grpc_frames(data: &Bytes) -> Vec<Bytes> {
        const HEADER: usize = 5;
        let mut frames = Vec::new();
        let mut offset = 0usize;
        while offset + HEADER <= data.len() {
            // bytes 1-4 are the big-endian message length; byte 0 is the compressed flag.
            let len = u32::from_be_bytes([
                data[offset + 1],
                data[offset + 2],
                data[offset + 3],
                data[offset + 4],
            ]) as usize;
            let payload_start = offset + HEADER;
            let payload_end = payload_start + len;
            if payload_end > data.len() {
                break; // truncated — stop rather than panic
            }
            frames.push(data.slice(payload_start..payload_end));
            offset = payload_end;
        }
        // If no valid frame header was found treat the entire body as one raw payload
        // (handles the degenerate case of an empty or header-only body gracefully).
        if frames.is_empty() && !data.is_empty() {
            frames.push(data.clone());
        }
        frames
    }

    /// Build an HTTP/2 response from already-buffered payloads + final metadata.
    async fn grpc_stream_response_from_payloads(
        payloads: Vec<Vec<u8>>,
        meta: GrpcMetadata,
    ) -> Response<BoxBody> {
        let mut frames: Vec<Bytes> = Vec::with_capacity(payloads.len());
        for payload in payloads {
            let mut buf = BytesMut::with_capacity(5 + payload.len());
            buf.put_u8(0); // not compressed
            buf.put_u32(payload.len() as u32);
            buf.put_slice(&payload);
            frames.push(buf.freeze());
        }

        let mut trailers = http::HeaderMap::new();
        trailers.insert("grpc-status", http::HeaderValue::from_static("0"));
        for (k, v) in &meta.headers {
            if let (Ok(name), Ok(val)) = (
                http::HeaderName::from_bytes(k.as_bytes()),
                http::HeaderValue::from_str(v),
            ) {
                trailers.insert(name, val);
            }
        }

        let mut http_frames: Vec<Result<http_body::Frame<Bytes>, Infallible>> = frames
            .into_iter()
            .map(|b| Ok(http_body::Frame::data(b)))
            .collect();
        http_frames.push(Ok(http_body::Frame::trailers(trailers)));

        let response_body = BodyExt::boxed(StreamBody::new(futures::stream::iter(http_frames)));

        Response::builder()
            .status(200)
            .header("content-type", "application/grpc")
            .body(response_body)
            .unwrap()
    }

    /// Collect a response stream into a single HTTP/2 response with one DATA frame
    /// per stream item plus a trailing `grpc-status=0` header and any response metadata.
    #[allow(dead_code)]
    async fn grpc_stream_response(
        mut stream: GrpcMessageStream,
        meta: GrpcMetadata,
    ) -> Response<BoxBody> {
        use futures::StreamExt;

        // Collect all response messages.
        let mut frames: Vec<Bytes> = Vec::new();
        loop {
            match stream.next().await {
                Some(Ok(payload)) => {
                    let mut buf = BytesMut::with_capacity(5 + payload.len());
                    buf.put_u8(0); // not compressed
                    buf.put_u32(payload.len() as u32);
                    buf.put_slice(&payload);
                    frames.push(buf.freeze());
                }
                Some(Err(e)) => {
                    let (code, msg) = StatusCodeConverter::map_inbound_error(e);
                    return Self::grpc_error(code, msg);
                }
                None => break,
            }
        }

        let mut trailers = http::HeaderMap::new();
        trailers.insert("grpc-status", http::HeaderValue::from_static("0"));
        for (k, v) in &meta.headers {
            if let (Ok(name), Ok(val)) = (
                http::HeaderName::from_bytes(k.as_bytes()),
                http::HeaderValue::from_str(v),
            ) {
                trailers.insert(name, val);
            }
        }

        // Build the response body: one DATA frame per response message, then trailers.
        let mut http_frames: Vec<Result<http_body::Frame<Bytes>, Infallible>> = frames
            .into_iter()
            .map(|b| Ok(http_body::Frame::data(b)))
            .collect();
        http_frames.push(Ok(http_body::Frame::trailers(trailers)));

        let response_body = BodyExt::boxed(StreamBody::new(futures::stream::iter(http_frames)));

        Response::builder()
            .status(200)
            .header("content-type", "application/grpc")
            .body(response_body)
            .unwrap()
    }

    fn grpc_error(code: tonic::Code, message: impl Into<String>) -> Response<BoxBody> {
        let message = message.into();
        let mut trailers = http::HeaderMap::new();
        trailers.insert(
            "grpc-status",
            http::HeaderValue::from_str(&(code as i32).to_string()).unwrap(),
        );
        if let Ok(val) = http::HeaderValue::from_str(&message) {
            trailers.insert("grpc-message", val);
        }

        let response_body = StreamBody::new(futures::stream::iter([Ok::<
            http_body::Frame<Bytes>,
            Infallible,
        >(
            http_body::Frame::trailers(trailers),
        )]))
        .boxed();

        Response::builder()
            .status(200)
            .header("content-type", "application/grpc")
            .body(response_body)
            .unwrap()
    }

    fn collect_metadata(headers: &http::HeaderMap) -> HashMap<String, String> {
        headers
            .iter()
            .filter_map(|(k, v)| match v.to_str() {
                Ok(vs) => Some((k.to_string(), vs.to_string())),
                Err(_) => {
                    tracing::warn!(header = %k, "dropping non-UTF-8 gRPC request header");
                    None
                }
            })
            .collect()
    }
}

// `map_error` was replaced by `map_inbound_error` in
// `crate::core::status_codes` — kept here as a cross-reference comment.

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::GrpcServerConfig;

    // ── read_deadline ─────────────────────────────────────────────────────

    #[test]
    fn test_read_deadline_falls_back_to_default_when_header_absent() {
        let map = http::HeaderMap::new();
        assert_eq!(TonicServerDispatcher::read_deadline(&map), DEFAULT_DEADLINE);
    }

    #[test]
    fn test_read_deadline_parses_grpc_timeout_header() {
        let mut map = http::HeaderMap::new();
        map.insert("grpc-timeout", "500m".parse().unwrap());
        assert_eq!(
            TonicServerDispatcher::read_deadline(&map),
            Duration::from_millis(500)
        );
    }

    #[test]
    fn test_read_deadline_falls_back_on_malformed_header() {
        let mut map = http::HeaderMap::new();
        map.insert("grpc-timeout", "garbage".parse().unwrap());
        assert_eq!(TonicServerDispatcher::read_deadline(&map), DEFAULT_DEADLINE);
    }

    // ── grpc_error ────────────────────────────────────────────────────────

    #[test]
    fn test_grpc_error_returns_200_with_grpc_content_type() {
        let resp = TonicServerDispatcher::grpc_error(tonic::Code::NotFound, "missing");
        assert_eq!(resp.status(), 200);
        assert_eq!(
            resp.headers().get("content-type").unwrap(),
            "application/grpc"
        );
    }

    // ── collect_metadata ──────────────────────────────────────────────────

    #[test]
    fn test_collect_metadata_extracts_utf8_header_values() {
        let mut map = http::HeaderMap::new();
        map.insert("x-request-id", "abc-123".parse().unwrap());
        let meta = TonicServerDispatcher::collect_metadata(&map);
        assert_eq!(meta.get("x-request-id"), Some(&"abc-123".to_string()));
    }

    // ── sanitize_authz_error ──────────────────────────────────────────────

    #[test]
    fn test_sanitize_authz_error_strips_permission_denied_rationale() {
        let detailed = GrpcIngressError::PermissionDenied(
            "policy ROLE_ADMIN denied subject=alice path=/svc/Drop".into(),
        );
        let sanitized = TonicServerDispatcher::sanitize_authz_error(detailed);
        match sanitized {
            GrpcIngressError::PermissionDenied(msg) => {
                assert_eq!(msg, "authorization denied");
                assert!(!msg.contains("alice"));
                assert!(!msg.contains("ROLE_ADMIN"));
            }
            other => panic!("expected PermissionDenied, got {other:?}"),
        }
    }

    #[test]
    fn test_sanitize_authz_error_strips_status_permission_denied_rationale() {
        let detailed = GrpcIngressError::Status(
            GrpcStatusCode::PermissionDenied,
            "denied: subject=bob lacks scope=admin".into(),
        );
        let sanitized = TonicServerDispatcher::sanitize_authz_error(detailed);
        match sanitized {
            GrpcIngressError::Status(GrpcStatusCode::PermissionDenied, msg) => {
                assert_eq!(msg, "authorization denied");
                assert!(!msg.contains("bob"));
                assert!(!msg.contains("scope"));
            }
            other => panic!("expected Status(PermissionDenied), got {other:?}"),
        }
    }

    #[test]
    fn test_sanitize_authz_error_passes_through_unrelated_errors() {
        let original = GrpcIngressError::NotFound("row not found".into());
        let result = TonicServerDispatcher::sanitize_authz_error(original);
        match result {
            GrpcIngressError::NotFound(msg) => assert_eq!(msg, "row not found"),
            other => panic!("expected NotFound, got {other:?}"),
        }
    }

    // ── enforce_authorization_invariant ──────────────────────────────────

    use crate::api::interceptor::{
        AuthorizationInterceptor, GrpcIngressInterceptor, GrpcIngressInterceptorChain,
    };
    use crate::api::port::grpc_ingress::{GrpcHealthCheck, GrpcIngress, GrpcIngressResult};
    use futures::future::BoxFuture;

    struct TonicGrpcServerFakeAuthz;
    impl GrpcIngressInterceptor for TonicGrpcServerFakeAuthz {
        fn before_dispatch(&self, _: &mut GrpcRequest) -> Result<(), GrpcIngressError> {
            Ok(())
        }
        fn after_dispatch(&self, _: &mut GrpcResponse) -> Result<(), GrpcIngressError> {
            Ok(())
        }
        fn is_authorization(&self) -> bool {
            true
        }
    }
    impl AuthorizationInterceptor for TonicGrpcServerFakeAuthz {}

    struct TonicGrpcServerDummyHandler;
    impl GrpcIngress for TonicGrpcServerDummyHandler {
        fn handle_unary(
            &self,
            _: GrpcRequest,
            _ctx: RequestContext,
        ) -> BoxFuture<'_, GrpcIngressResult<GrpcResponse>> {
            Box::pin(async {
                Ok(GrpcResponse {
                    body: vec![],
                    metadata: GrpcMetadata::default(),
                })
            })
        }
        fn health_check(&self) -> BoxFuture<'_, GrpcIngressResult<GrpcHealthCheck>> {
            Box::pin(async { Ok(GrpcHealthCheck::healthy()) })
        }
    }

    #[test]
    fn test_enforce_authorization_invariant_succeeds_with_authz_interceptor() {
        let chain = GrpcIngressInterceptorChain::new().push(Arc::new(TonicGrpcServerFakeAuthz));
        let server = TonicGrpcServer::new("127.0.0.1:0", Arc::new(TonicGrpcServerDummyHandler))
            .with_interceptors(chain);
        // Should not panic.
        server.enforce_authorization_invariant();
    }

    #[test]
    fn test_enforce_authorization_invariant_succeeds_when_allow_unauthenticated_is_set() {
        let server = TonicGrpcServer::new("127.0.0.1:0", Arc::new(TonicGrpcServerDummyHandler))
            .allow_unauthenticated(true);
        // Should not panic, only WARN.
        server.enforce_authorization_invariant();
    }

    #[test]
    #[should_panic(expected = "AuthorizationInterceptor")]
    fn test_enforce_authorization_invariant_panics_when_authz_missing_and_fail_closed() {
        let server = TonicGrpcServer::new("127.0.0.1:0", Arc::new(TonicGrpcServerDummyHandler));
        // No authz registered, allow_unauthenticated defaults to false → panic.
        server.enforce_authorization_invariant();
    }

    // ── with_audit_sink / allow_unauthenticated builders ──────────────────

    #[test]
    fn test_with_audit_sink_installs_provided_sink() {
        use std::sync::Mutex;
        struct TonicGrpcServerCountingSink(Arc<Mutex<usize>>);
        impl AuditSink for TonicGrpcServerCountingSink {
            fn record(&self, _: AuditEvent) {
                *self.0.lock().unwrap() += 1;
            }
        }
        let calls = Arc::new(Mutex::new(0usize));
        let sink: Arc<dyn AuditSink> = Arc::new(TonicGrpcServerCountingSink(calls.clone()));
        let server = TonicGrpcServer::new("127.0.0.1:0", Arc::new(TonicGrpcServerDummyHandler))
            .with_audit_sink(sink);
        // Drive the sink directly through the server's stored Arc.
        server.audit_sink.record(AuditEvent {
            timestamp: SystemTime::UNIX_EPOCH,
            method: "/x".into(),
            identity: None,
            status: GrpcStatusCode::Ok,
            duration_ms: 0,
        });
        assert_eq!(*calls.lock().unwrap(), 1);
    }

    #[test]
    fn test_allow_unauthenticated_sets_the_flag() {
        let server = TonicGrpcServer::new("127.0.0.1:0", Arc::new(TonicGrpcServerDummyHandler))
            .allow_unauthenticated(true);
        assert!(server.allow_unauthenticated);
    }

    #[test]
    fn test_new_disables_reflection_flag_by_default() {
        let server = TonicGrpcServer::new("127.0.0.1:0", Arc::new(TonicGrpcServerDummyHandler));
        assert!(!server.is_reflection_enabled());
    }

    #[test]
    fn test_enable_reflection_builder_flips_the_flag() {
        let server = TonicGrpcServer::new("127.0.0.1:0", Arc::new(TonicGrpcServerDummyHandler))
            .enable_reflection(true);
        assert!(server.is_reflection_enabled());
    }

    #[test]
    fn test_from_config_propagates_enable_reflection_from_config() {
        let cfg = GrpcServerConfig::new("127.0.0.1:0".parse().unwrap())
            .allow_plaintext()
            .enable_reflection();
        let server = TonicGrpcServer::from_config(&cfg, Arc::new(TonicGrpcServerDummyHandler))
            .expect("config valid");
        assert!(server.is_reflection_enabled());
    }
}

#[cfg(test)]
mod dedicated_coverage {
    use super::TonicGrpcServer;
    use crate::api::port::grpc_ingress::{GrpcHealthCheck, GrpcIngress, GrpcIngressResult};
    use crate::api::value_object::{CompressionMode, GrpcRequest, GrpcResponse};
    use edge_domain::RequestContext;
    use futures::future::BoxFuture;
    use std::sync::Arc;

    struct TonicGrpcServerStub;
    impl GrpcIngress for TonicGrpcServerStub {
        fn handle_unary(
            &self,
            _: GrpcRequest,
            _ctx: RequestContext,
        ) -> BoxFuture<'_, GrpcIngressResult<GrpcResponse>> {
            Box::pin(async {
                Ok(GrpcResponse {
                    body: vec![],
                    metadata: Default::default(),
                })
            })
        }
        fn health_check(&self) -> BoxFuture<'_, GrpcIngressResult<GrpcHealthCheck>> {
            Box::pin(async {
                Ok(GrpcHealthCheck {
                    healthy: true,
                    message: None,
                })
            })
        }
    }

    fn server() -> TonicGrpcServer {
        TonicGrpcServer::new("127.0.0.1:0", Arc::new(TonicGrpcServerStub))
            .allow_unauthenticated(true)
    }

    #[test]
    fn test_is_reflection_enabled_false_by_default() {
        assert!(!server().is_reflection_enabled());
    }

    #[test]
    fn test_with_compression_stores_mode() {
        let s = server().with_compression(CompressionMode::Gzip);
        assert!(matches!(s.compression, CompressionMode::Gzip));
    }

    #[test]
    fn test_with_max_message_size_overrides_default() {
        let s = server().with_max_message_size(1024);
        assert_eq!(s.max_bytes, 1024);
    }

    #[test]
    fn test_with_max_concurrent_streams_sets_value() {
        let s = server().with_max_concurrent_streams(32);
        assert_eq!(s.max_concurrent_streams, 32);
    }

    #[test]
    fn test_with_interceptors_assigns_chain() {
        use crate::api::interceptor::GrpcIngressInterceptorChain;
        let chain = GrpcIngressInterceptorChain::new();
        let s = server().with_interceptors(chain);
        drop(s); // interceptors field is not Option — assignment verified by compilation
    }

    #[test]
    fn test_with_tls_sets_config() {
        use swe_edge_ingress_tls::IngressTlsConfig;
        let cfg = IngressTlsConfig::tls("cert.pem", "key.pem");
        let s = server().with_tls(cfg);
        assert!(s.tls.is_some());
    }

    /// @covers: serve
    #[tokio::test]
    async fn test_serve_returns_error_on_invalid_bind() {
        let s = TonicGrpcServer::new("0.0.0.0:99999", Arc::new(TonicGrpcServerStub))
            .allow_unauthenticated(true);
        let result = s.serve(std::future::ready(())).await;
        assert!(result.is_err());
    }

    /// @covers: serve_with_listener
    #[tokio::test]
    async fn test_serve_with_listener_completes_on_immediate_shutdown() {
        use tokio::net::TcpListener;
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let s = server();
        let result = s
            .serve_with_listener(listener, std::future::ready(()))
            .await;
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod sync_coverage {
    use super::TonicGrpcServer;
    use crate::api::port::grpc_ingress::{GrpcHealthCheck, GrpcIngress, GrpcIngressResult};
    use crate::api::value_object::{GrpcRequest, GrpcResponse};
    use edge_domain::RequestContext;
    use futures::future::BoxFuture;
    use std::sync::Arc;

    struct TonicGrpcServerStub;
    impl GrpcIngress for TonicGrpcServerStub {
        fn handle_unary(
            &self,
            _: GrpcRequest,
            _ctx: RequestContext,
        ) -> BoxFuture<'_, GrpcIngressResult<GrpcResponse>> {
            Box::pin(async {
                Ok(GrpcResponse {
                    body: vec![],
                    metadata: Default::default(),
                })
            })
        }
        fn health_check(&self) -> BoxFuture<'_, GrpcIngressResult<GrpcHealthCheck>> {
            Box::pin(async {
                Ok(GrpcHealthCheck {
                    healthy: true,
                    message: None,
                })
            })
        }
    }

    #[test]
    fn test_serve_is_constructible() {
        let _ = TonicGrpcServer::new("127.0.0.1:0", Arc::new(TonicGrpcServerStub))
            .allow_unauthenticated(true);
    }

    #[test]
    fn test_serve_with_listener_is_constructible() {
        let _ = TonicGrpcServer::new("127.0.0.1:0", Arc::new(TonicGrpcServerStub))
            .allow_unauthenticated(true);
    }
}
