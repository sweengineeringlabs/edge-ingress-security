//! `HealthService` and `HealthAggregate` trait implementations.

use std::time::Duration;

use futures::future::BoxFuture;
use futures::StreamExt;

use crate::api::health_service::{
    HealthService, ServingStatus,
    HEALTH_CHECK_METHOD, HEALTH_WATCH_METHOD,
};
use edge_domain::RequestContext;

use crate::api::port::grpc_inbound::{
    GrpcHealthCheck, GrpcInbound, GrpcInboundError, GrpcInboundResult, GrpcMessageStream,
};
use crate::api::value_object::{GrpcMetadata, GrpcRequest, GrpcResponse, GrpcStatusCode};

impl GrpcInbound for HealthService {
    fn handle_unary(&self, request: GrpcRequest, _ctx: RequestContext) -> BoxFuture<'_, GrpcInboundResult<GrpcResponse>> {
        Box::pin(async move {
            match request.method.as_str() {
                HEALTH_CHECK_METHOD => self.handle_check(&request.body),
                HEALTH_WATCH_METHOD => Err(GrpcInboundError::Status(
                    GrpcStatusCode::Unimplemented,
                    "Watch must be invoked via streaming dispatch".into(),
                )),
                other => Err(GrpcInboundError::Unimplemented(format!("unknown method {other}"))),
            }
        })
    }

    fn handle_stream(
        &self,
        method:    String,
        _metadata: GrpcMetadata,
        messages:  GrpcMessageStream,
        _ctx:      RequestContext,
    ) -> BoxFuture<'_, GrpcInboundResult<(GrpcMessageStream, GrpcMetadata)>> {
        Box::pin(async move {
            if method.as_str() != HEALTH_WATCH_METHOD {
                let mut messages = messages;
                let body = match messages.next().await {
                    Some(Ok(b))  => b,
                    Some(Err(e)) => return Err(e),
                    None         => vec![],
                };
                let req  = GrpcRequest::new(method, body, Duration::from_secs(30));
                let resp = self.handle_unary(req, RequestContext::unauthenticated()).await?;
                let out: GrpcMessageStream = Box::pin(futures::stream::once(
                    futures::future::ready(Ok(resp.body)),
                ));
                return Ok((out, resp.metadata));
            }

            let mut messages = messages;
            let body = match messages.next().await {
                Some(Ok(b))  => b,
                Some(Err(e)) => return Err(e),
                None         => vec![],
            };
            let service = decode_health_check_request(&body).unwrap_or_default();
            let initial = self.get_status(&service).unwrap_or(ServingStatus::ServiceUnknown);
            let rx      = self.subscribe();
            let target  = service.clone();

            enum WatchPhase { Initial, Streaming, Closed }
            struct WatchState {
                phase:    WatchPhase,
                rx:       tokio::sync::broadcast::Receiver<(String, ServingStatus)>,
                target:   String,
                snapshot: ServingStatus,
            }

            let state = WatchState { phase: WatchPhase::Initial, rx, target, snapshot: initial };
            let stream = futures::stream::unfold(state, |mut s| async move {
                match s.phase {
                    WatchPhase::Initial => {
                        let frame = encode_health_check_response(s.snapshot);
                        s.phase   = WatchPhase::Streaming;
                        Some((Ok(frame), s))
                    }
                    WatchPhase::Streaming => loop {
                        match s.rx.recv().await {
                            Ok((svc, status)) => {
                                if svc == s.target {
                                    return Some((Ok(encode_health_check_response(status)), s));
                                }
                            }
                            Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => {
                                s.phase = WatchPhase::Closed;
                                return Some((Err(GrpcInboundError::Status(
                                    GrpcStatusCode::ResourceExhausted,
                                    "watch subscriber fell behind".into(),
                                )), s));
                            }
                            Err(tokio::sync::broadcast::error::RecvError::Closed) => return None,
                        }
                    },
                    WatchPhase::Closed => None,
                }
            });

            Ok((Box::pin(stream) as GrpcMessageStream, GrpcMetadata::default()))
        })
    }

    fn health_check(&self) -> BoxFuture<'_, GrpcInboundResult<GrpcHealthCheck>> {
        Box::pin(async move { Ok(GrpcHealthCheck::healthy()) })
    }
}

impl HealthService {
    fn handle_check(&self, body: &[u8]) -> GrpcInboundResult<GrpcResponse> {
        let service = decode_health_check_request(body).unwrap_or_default();
        match self.get_status(&service) {
            Some(status) => Ok(GrpcResponse {
                body:     encode_health_check_response(status),
                metadata: GrpcMetadata::default(),
            }),
            None => Err(GrpcInboundError::NotFound(format!("service {service:?} not registered"))),
        }
    }
}

pub(crate) fn decode_health_check_request(body: &[u8]) -> Option<String> {
    if body.is_empty() { return Some(String::new()); }
    if body[0] != 0x0a { return Some(String::new()); }
    let mut idx = 1usize;
    let (len, consumed) = decode_varint(&body[idx..])?;
    idx += consumed;
    if idx + (len as usize) > body.len() { return None; }
    std::str::from_utf8(&body[idx..idx + len as usize]).ok().map(str::to_string)
}

pub(crate) fn encode_health_check_response(status: ServingStatus) -> Vec<u8> {
    let value = status as i32;
    if value == 0 { return Vec::new(); }
    let mut out = Vec::with_capacity(2);
    out.push(0x08);
    encode_varint(value as u64, &mut out);
    out
}

fn decode_varint(bytes: &[u8]) -> Option<(u64, usize)> {
    let mut result = 0u64;
    let mut shift  = 0u32;
    for (i, byte) in bytes.iter().take(10).enumerate() {
        result |= ((byte & 0x7f) as u64) << shift;
        if byte & 0x80 == 0 { return Some((result, i + 1)); }
        shift += 7;
    }
    None
}

fn encode_varint(mut value: u64, out: &mut Vec<u8>) {
    while value >= 0x80 { out.push((value as u8) | 0x80); value >>= 7; }
    out.push(value as u8);
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::time::Duration;
    use futures::future::BoxFuture;

    use edge_domain::RequestContext;

    use crate::api::health_service::{HealthAggregate, HealthService, ServingStatus, HEALTH_CHECK_METHOD, HEALTH_WATCH_METHOD};
    use crate::api::port::grpc_inbound::{GrpcInbound, GrpcInboundError, GrpcInboundResult, GrpcHealthCheck, GrpcMessageStream};
    use crate::api::value_object::{GrpcMetadata, GrpcRequest, GrpcResponse};
    use super::{decode_health_check_request, encode_health_check_response, encode_varint};

    /// @covers: decode_health_check_request — empty body decodes to "".
    #[test]
    fn test_decode_health_check_request_empty_body_yields_empty_service_name() {
        assert_eq!(decode_health_check_request(&[]), Some(String::new()));
    }

    /// @covers: decode_health_check_request — non-empty service round-trips.
    #[test]
    fn test_decode_health_check_request_round_trips_service_name() {
        let body = {
            let b = "pkg.A".as_bytes();
            let mut out = Vec::with_capacity(2 + b.len());
            out.push(0x0a);
            encode_varint(b.len() as u64, &mut out);
            out.extend_from_slice(b);
            out
        };
        assert_eq!(decode_health_check_request(&body), Some("pkg.A".to_string()));
    }

    /// @covers: encode_health_check_response — SERVING encodes as 0x08 0x01.
    #[test]
    fn test_encode_health_check_response_serving_yields_two_byte_payload() {
        assert_eq!(encode_health_check_response(ServingStatus::Serving), vec![0x08, 0x01]);
    }

    /// @covers: handle_unary Check — returns SERVING for registered service.
    #[tokio::test]
    async fn test_check_returns_serving_for_registered_service() {
        let svc = Arc::new(HealthService::new());
        svc.set_status("pkg.A", ServingStatus::Serving);
        let body = {
            let b = "pkg.A".as_bytes();
            let mut out = Vec::with_capacity(2 + b.len());
            out.push(0x0a);
            encode_varint(b.len() as u64, &mut out);
            out.extend_from_slice(b);
            out
        };
        let req  = GrpcRequest::new(HEALTH_CHECK_METHOD, body, Duration::from_secs(1));
        let resp = svc.handle_unary(req, RequestContext::unauthenticated()).await.expect("Check must succeed");
        assert_eq!(resp.body, encode_health_check_response(ServingStatus::Serving));
    }

    /// @covers: HealthAggregate::refresh — propagates dispatcher health.
    #[tokio::test]
    async fn test_health_aggregate_refresh_pushes_dispatcher_health_to_overall() {
        struct AlwaysHealthy;
        impl GrpcInbound for AlwaysHealthy {
            fn handle_unary(&self, _: GrpcRequest, _ctx: RequestContext) -> BoxFuture<'_, GrpcInboundResult<GrpcResponse>> {
                Box::pin(async { Ok(GrpcResponse { body: vec![], metadata: GrpcMetadata::default() }) })
            }
            fn health_check(&self) -> BoxFuture<'_, GrpcInboundResult<GrpcHealthCheck>> {
                Box::pin(async { Ok(GrpcHealthCheck::healthy()) })
            }
        }
        let svc = Arc::new(HealthService::new());
        let agg = HealthAggregate::new(svc.clone(), Arc::new(AlwaysHealthy));
        svc.set_overall_status(ServingStatus::NotServing);
        agg.refresh().await;
        assert_eq!(svc.get_status(""), Some(ServingStatus::Serving));
    }
}
