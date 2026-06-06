//! Helper functions for Axum HTTP server implementation.

use crate::api::error::HttpIngressError;
use crate::api::traits::HttpIngress;
use crate::api::vo::ws::{WsChannel, WsMessage};
use crate::api::vo::{HttpBody, HttpMethod, HttpRequest, HttpResponse};
use axum::http::StatusCode;
use axum::response::IntoResponse as _;
use edge_domain::RequestContext;
use futures::StreamExt as _;
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use swe_edge_ingress_tls::IngressTlsConfig;
use swe_edge_ingress_verifier::TokenVerifier;
use tokio::net::TcpListener;

/// Helper struct for Axum HTTP server operations.
pub struct AxumHttpServerHelper;

impl AxumHttpServerHelper {
    /// Check if request is a WebSocket upgrade.
    pub fn is_websocket_upgrade(headers: &axum::http::HeaderMap) -> bool {
        headers
            .get(axum::http::header::UPGRADE)
            .and_then(|v| v.to_str().ok())
            .map(|v| v.eq_ignore_ascii_case("websocket"))
            .unwrap_or(false)
    }

    /// Check if request is Server-Sent Events.
    pub fn is_sse_request(headers: &axum::http::HeaderMap) -> bool {
        headers
            .get(axum::http::header::ACCEPT)
            .and_then(|v| v.to_str().ok())
            .map(|v| v.contains("text/event-stream"))
            .unwrap_or(false)
    }

    /// Map HTTP method to HttpMethod enum.
    pub fn map_method(m: &axum::http::Method) -> HttpMethod {
        match *m {
            axum::http::Method::GET => HttpMethod::Get,
            axum::http::Method::POST => HttpMethod::Post,
            axum::http::Method::PUT => HttpMethod::Put,
            axum::http::Method::PATCH => HttpMethod::Patch,
            axum::http::Method::DELETE => HttpMethod::Delete,
            axum::http::Method::HEAD => HttpMethod::Head,
            axum::http::Method::OPTIONS => HttpMethod::Options,
            _ => HttpMethod::Get,
        }
    }

    /// Parse query string into HashMap.
    pub fn parse_query(raw: Option<&str>) -> HashMap<String, String> {
        let mut map = HashMap::new();
        if let Some(q) = raw {
            for pair in q.split('&') {
                let mut parts = pair.splitn(2, '=');
                let key = Self::percent_decode(parts.next().unwrap_or(""));
                let value = Self::percent_decode(parts.next().unwrap_or(""));
                if !key.is_empty() {
                    map.insert(key, value);
                }
            }
        }
        map
    }

    /// Collect HTTP headers into HashMap.
    pub fn collect_headers(headers: &axum::http::HeaderMap) -> HashMap<String, String> {
        headers
            .iter()
            .filter_map(|(k, v)| v.to_str().ok().map(|vs| (k.to_string(), vs.to_string())))
            .collect()
    }

    /// Build HttpBody from bytes and content type.
    pub fn build_body(bytes: &bytes::Bytes, content_type: &str) -> Option<HttpBody> {
        if bytes.is_empty() {
            return None;
        }
        if content_type.contains("application/json") {
            serde_json::from_slice(bytes)
                .ok()
                .map(HttpBody::Json)
                .or_else(|| Some(HttpBody::Raw(bytes.to_vec())))
        } else if content_type.contains("application/x-www-form-urlencoded") {
            Some(HttpBody::Form(Self::parse_form(bytes)))
        } else {
            Some(HttpBody::Raw(bytes.to_vec()))
        }
    }

    /// Parse form data from bytes.
    pub fn parse_form(bytes: &bytes::Bytes) -> HashMap<String, String> {
        let mut map = HashMap::new();
        let s = std::str::from_utf8(bytes).unwrap_or("");
        for pair in s.split('&') {
            let mut parts = pair.splitn(2, '=');
            let key = Self::percent_decode(parts.next().unwrap_or(""));
            let value = Self::percent_decode(parts.next().unwrap_or(""));
            if !key.is_empty() {
                map.insert(key, value);
            }
        }
        map
    }

    /// Minimal percent-decode: `+` → space, `%XX` → byte.
    pub fn percent_decode(s: &str) -> String {
        let s = s.replace('+', " ");
        let mut out = String::with_capacity(s.len());
        let mut chars = s.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '%' {
                let h1 = chars.next();
                let h2 = chars.next();
                match (h1, h2) {
                    (Some(a), Some(b)) => {
                        if let Ok(byte) = u8::from_str_radix(&format!("{a}{b}"), 16) {
                            out.push(byte as char);
                        } else {
                            // Invalid hex sequence — pass through literally.
                            out.push('%');
                            out.push(a);
                            out.push(b);
                        }
                    }
                    (Some(a), None) => {
                        out.push('%');
                        out.push(a);
                    }
                    _ => {
                        out.push('%');
                    }
                }
                continue;
            }
            out.push(c);
        }
        out
    }

    /// Build Axum response from HttpResponse.
    pub fn build_response(resp: HttpResponse) -> axum::response::Response {
        let status = axum::http::StatusCode::from_u16(resp.status)
            .unwrap_or(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        let mut builder = axum::response::Response::builder().status(status);
        for (k, v) in &resp.headers {
            builder = builder.header(k.as_str(), v.as_str());
        }
        builder
            .body(axum::body::Body::from(resp.body))
            .unwrap_or_else(|_| Self::internal_server_error("response build failed"))
    }

    /// Build error response from HttpIngressError.
    pub fn error_response(e: HttpIngressError) -> axum::response::Response {
        let status = match &e {
            HttpIngressError::NotFound(_) => axum::http::StatusCode::NOT_FOUND,
            HttpIngressError::InvalidInput(_) => axum::http::StatusCode::BAD_REQUEST,
            HttpIngressError::Unauthorized(_) => axum::http::StatusCode::UNAUTHORIZED,
            HttpIngressError::PermissionDenied(_) => axum::http::StatusCode::FORBIDDEN,
            HttpIngressError::Conflict(_) => axum::http::StatusCode::CONFLICT,
            HttpIngressError::MethodNotAllowed(_) => axum::http::StatusCode::METHOD_NOT_ALLOWED,
            HttpIngressError::UnprocessableEntity(_) => {
                axum::http::StatusCode::UNPROCESSABLE_ENTITY
            }
            HttpIngressError::Timeout(_) => axum::http::StatusCode::GATEWAY_TIMEOUT,
            HttpIngressError::Unavailable(_) => axum::http::StatusCode::SERVICE_UNAVAILABLE,
            HttpIngressError::Internal(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        };
        axum::response::Response::builder()
            .status(status)
            .header("content-type", "text/plain; charset=utf-8")
            .body(axum::body::Body::from(e.to_string()))
            .unwrap_or_else(|_| Self::internal_server_error("error response build failed"))
    }

    /// Build 413 response for request body too large.
    pub fn payload_too_large() -> axum::response::Response {
        Self::plain_text_response(
            axum::http::StatusCode::PAYLOAD_TOO_LARGE,
            "request body exceeds size limit",
        )
    }

    /// Build 500 response for internal server error.
    pub fn internal_server_error(msg: &'static str) -> axum::response::Response {
        Self::plain_text_response(axum::http::StatusCode::INTERNAL_SERVER_ERROR, msg)
    }

    fn plain_text_response(
        status: axum::http::StatusCode,
        body: impl Into<String>,
    ) -> axum::response::Response {
        let mut response = axum::response::Response::new(axum::body::Body::from(body.into()));
        *response.status_mut() = status;
        response.headers_mut().insert(
            axum::http::header::CONTENT_TYPE,
            axum::http::HeaderValue::from_static("text/plain; charset=utf-8"),
        );
        response
    }

    /// Verify bearer token and attach RequestContext.
    #[allow(clippy::result_large_err)]
    pub fn verify_auth(
        mut req: axum::extract::Request,
        verifier: Option<&dyn TokenVerifier>,
    ) -> Result<axum::extract::Request, axum::response::Response> {
        let Some(verifier) = verifier else {
            return Ok(req);
        };

        let token = req
            .headers()
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "))
            .ok_or_else(|| {
                Self::plain_text_response(
                    StatusCode::UNAUTHORIZED,
                    "missing or malformed Authorization header",
                )
            })?;

        let claims = verifier.verify(token).map_err(|e| {
            tracing::debug!(error = %e, "bearer token rejected");
            Self::plain_text_response(StatusCode::UNAUTHORIZED, "invalid token")
        })?;

        let ctx = RequestContext::authenticated(
            claims.sub.clone().unwrap_or_default(),
            claims.iss.clone(),
            claims
                .custom
                .get("tenant_id")
                .map(|v| v.to_string().trim_matches('"').to_string()),
            claims
                .custom
                .iter()
                .map(|(k, v)| (k.clone(), v.to_string()))
                .collect(),
        );
        req.extensions_mut().insert(ctx);
        Ok(req)
    }

    /// Extract HttpRequest and RequestContext from Axum request.
    pub async fn extract_request(
        req: axum::extract::Request,
        body_limit: usize,
    ) -> Result<(HttpRequest, RequestContext), axum::response::Response> {
        let (parts, body) = req.into_parts();

        let ctx = parts
            .extensions
            .get::<RequestContext>()
            .cloned()
            .unwrap_or_default();

        let method = Self::map_method(&parts.method);
        let url = parts.uri.to_string();
        let query = Self::parse_query(parts.uri.query());
        let headers = Self::collect_headers(&parts.headers);
        let ct = headers
            .get("content-type")
            .map(|s| s.as_str())
            .unwrap_or("")
            .to_owned();

        let bytes = axum::body::to_bytes(axum::body::Body::new(body), body_limit)
            .await
            .map_err(|_| Self::payload_too_large())?;

        let body = Self::build_body(&bytes, &ct);
        Ok((
            HttpRequest {
                method,
                url,
                headers,
                query,
                body,
                timeout: None,
            },
            ctx,
        ))
    }

    /// Dispatch SSE (Server-Sent Events) request.
    pub async fn dispatch_sse(
        req: axum::extract::Request,
        body_limit: usize,
        handler: Arc<dyn crate::api::traits::HttpStream>,
    ) -> axum::response::Response {
        let (http_req, ctx) = match Self::extract_request(req, body_limit).await {
            Ok(r) => r,
            Err(resp) => return resp,
        };
        match handler.handle_sse(http_req, ctx).await {
            Ok(stream) => {
                use axum::response::sse::{Event, KeepAlive, Sse};
                let axum_stream = stream.map(|item| {
                    item.map(|ev| {
                        let mut event = Event::default().data(ev.data);
                        if let Some(name) = ev.event {
                            event = event.event(name);
                        }
                        if let Some(id) = ev.id {
                            event = event.id(id);
                        }
                        event
                    })
                    .map_err(|e| e.to_string())
                });
                Sse::new(axum_stream)
                    .keep_alive(KeepAlive::default())
                    .into_response()
            }
            Err(e) => Self::error_response(e),
        }
    }

    /// Dispatch WebSocket upgrade request.
    pub async fn dispatch_websocket(
        req: axum::extract::Request,
        handler: Arc<dyn crate::api::traits::HttpStream>,
    ) -> axum::response::Response {
        use axum::extract::ws::{Message, WebSocketUpgrade};
        use axum::extract::FromRequestParts;

        let (mut parts, _body) = req.into_parts();

        let ctx = parts
            .extensions
            .get::<RequestContext>()
            .cloned()
            .unwrap_or_default();

        let http_req = HttpRequest {
            method: Self::map_method(&parts.method),
            url: parts.uri.to_string(),
            headers: Self::collect_headers(&parts.headers),
            query: Self::parse_query(parts.uri.query()),
            body: None,
            timeout: None,
        };

        let ws_upgrade = match WebSocketUpgrade::from_request_parts(&mut parts, &()).await {
            Ok(u) => u,
            Err(e) => {
                return Self::plain_text_response(
                    StatusCode::BAD_REQUEST,
                    format!("invalid websocket upgrade: {e}"),
                )
            }
        };

        ws_upgrade
            .on_upgrade(move |socket| async move {
                use tokio::sync::mpsc;

                let (out_tx, mut out_rx) = mpsc::unbounded_channel::<WsMessage>();

                let (mut socket_send, socket_recv) = futures::StreamExt::split(socket);

                let incoming = Box::pin(socket_recv.filter_map(|item| async move {
                    match item {
                        Ok(Message::Text(t)) => Some(Ok(WsMessage::text(t.as_str()))),
                        Ok(Message::Binary(b)) => Some(Ok(WsMessage::binary(b))),
                        Ok(Message::Close(_)) => None,
                        Ok(_) => None,
                        Err(e) => Some(Err(crate::api::error::HttpIngressError::Internal(
                            e.to_string(),
                        ))),
                    }
                }));

                let channel = WsChannel {
                    sender: out_tx,
                    receiver: incoming,
                };

                let handler_fut = handler.handle_websocket(http_req, ctx, channel);

                let bridge_fut = async move {
                    while let Some(msg) = out_rx.recv().await {
                        let ws_msg = if msg.binary {
                            Message::Binary(msg.data.to_vec().into())
                        } else {
                            Message::Text(String::from_utf8_lossy(&msg.data).into_owned().into())
                        };
                        use futures::SinkExt as _;
                        if socket_send.send(ws_msg).await.is_err() {
                            break;
                        }
                    }
                };

                tokio::select! {
                    result = handler_fut => {
                        if let Err(e) = result {
                            tracing::warn!("WebSocket handler error: {e}");
                        }
                    }
                    _ = bridge_fut => {}
                }
            })
            .into_response()
    }

    /// Serve TLS connections.
    pub async fn serve_tls<F>(
        listener: TcpListener,
        handler: Arc<dyn HttpIngress>,
        body_limit: usize,
        verifier: Option<Arc<dyn TokenVerifier>>,
        stream_handler: Option<Arc<dyn crate::api::traits::HttpStream>>,
        tls_cfg: &IngressTlsConfig,
        shutdown: F,
    ) -> Result<(), crate::api::server::error::HttpServerError>
    where
        F: std::future::Future<Output = ()> + Send + 'static,
    {
        use hyper_util::rt::{TokioExecutor, TokioIo};

        let acceptor = swe_edge_ingress_tls::TlsSvc::build_tls_acceptor(tls_cfg)
            .map_err(crate::api::server::error::HttpServerError::Tls)?;

        let mut shutdown = std::pin::pin!(shutdown);

        loop {
            tokio::select! {
                res = listener.accept() => {
                    let (stream, _) = match res {
                        Ok(s)  => s,
                        Err(e) => { tracing::warn!("TLS accept error: {e}"); continue; }
                    };
                    let acceptor       = acceptor.clone();
                    let handler        = handler.clone();
                    let verifier       = verifier.clone();
                    let stream_handler = stream_handler.clone();
                    tokio::spawn(async move {
                        let tls = match acceptor.accept(stream).await {
                            Ok(s)  => s,
                            Err(e) => { tracing::debug!("TLS handshake failed: {e}"); return; }
                        };
                        let io  = TokioIo::new(tls);
                        let svc = hyper::service::service_fn(move |req: http::Request<hyper::body::Incoming>| {
                            let handler        = handler.clone();
                            let verifier       = verifier.clone();
                            let stream_handler = stream_handler.clone();
                            async move {
                                let req = req.map(axum::body::Body::new);
                                let req = match AxumHttpServerHelper::verify_auth(req, verifier.as_deref()) {
                                    Ok(r)    => r,
                                    Err(rsp) => return Ok::<_, Infallible>(rsp),
                                };

                                // Streaming: WebSocket upgrade
                                if AxumHttpServerHelper::is_websocket_upgrade(req.headers()) {
                                    if let Some(sh) = stream_handler {
                                        return Ok(AxumHttpServerHelper::dispatch_websocket(req, sh).await);
                                    }
                                }

                                // Streaming: SSE
                                if AxumHttpServerHelper::is_sse_request(req.headers()) {
                                    if let Some(sh) = stream_handler {
                                        return Ok(AxumHttpServerHelper::dispatch_sse(req, body_limit, sh).await);
                                    }
                                }

                                // Regular HTTP
                                let resp = match AxumHttpServerHelper::extract_request(req, body_limit).await {
                                    Ok((http_req, ctx)) => match handler.handle(http_req, ctx).await {
                                        Ok(resp) => AxumHttpServerHelper::build_response(resp),
                                        Err(e)   => AxumHttpServerHelper::error_response(e),
                                    },
                                    Err(resp) => resp,
                                };
                                Ok::<_, Infallible>(resp)
                            }
                        });
                        if let Err(e) = hyper_util::server::conn::auto::Builder::new(TokioExecutor::new())
                            .serve_connection(io, svc)
                            .await
                        {
                            tracing::debug!("HTTPS connection error: {e}");
                        }
                    });
                }
                _ = &mut shutdown => break,
            }
        }

        Ok(())
    }
}
