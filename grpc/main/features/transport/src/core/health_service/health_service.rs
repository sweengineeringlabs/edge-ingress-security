//! `HealthService` and `HealthAggregate` trait implementations.

use std::time::Duration;

use futures::future::BoxFuture;
use futures::StreamExt;

use crate::api::health_service::{
    HealthService, ServingStatus, HEALTH_CHECK_METHOD, HEALTH_WATCH_METHOD,
};
use edge_domain::RequestContext;

use crate::api::port::grpc_ingress::{
    GrpcHealthCheck, GrpcIngress, GrpcIngressError, GrpcIngressResult, GrpcMessageStream,
};
use crate::api::value::{GrpcMetadata, GrpcRequest, GrpcResponse, GrpcStatusCode};

/// Codec for health check messages.
pub(crate) struct HealthCheckCodec;

impl HealthCheckCodec {
    pub(crate) fn decode_request(body: &[u8]) -> Option<String> {
        if body.is_empty() {
            return Some(String::new());
        }
        if body[0] != 0x0a {
            return Some(String::new());
        }
        let mut idx = 1usize;
        let (len, consumed) = Self::decode_varint(&body[idx..])?;
        idx += consumed;
        if idx + (len as usize) > body.len() {
            return None;
        }
        std::str::from_utf8(&body[idx..idx + len as usize])
            .ok()
            .map(str::to_string)
    }

    pub(crate) fn encode_response(status: ServingStatus) -> Vec<u8> {
        let value = status as i32;
        if value == 0 {
            return Vec::new();
        }
        let mut out = Vec::with_capacity(2);
        out.push(0x08);
        Self::encode_varint(value as u64, &mut out);
        out
    }

    fn decode_varint(bytes: &[u8]) -> Option<(u64, usize)> {
        let mut result = 0u64;
        let mut shift = 0u32;
        for (i, byte) in bytes.iter().take(10).enumerate() {
            result |= ((byte & 0x7f) as u64) << shift;
            if byte & 0x80 == 0 {
                return Some((result, i + 1));
            }
            shift += 7;
        }
        None
    }

    fn encode_varint(mut value: u64, out: &mut Vec<u8>) {
        while value >= 0x80 {
            out.push((value as u8) | 0x80);
            value >>= 7;
        }
        out.push(value as u8);
    }
}

impl GrpcIngress for HealthService {
    fn handle_unary(
        &self,
        request: GrpcRequest,
        _ctx: RequestContext,
    ) -> BoxFuture<'_, GrpcIngressResult<GrpcResponse>> {
        Box::pin(async move {
            match request.method.as_str() {
                HEALTH_CHECK_METHOD => self.handle_check(&request.body),
                HEALTH_WATCH_METHOD => Err(GrpcIngressError::Status(
                    GrpcStatusCode::Unimplemented,
                    "Watch must be invoked via streaming dispatch".into(),
                )),
                other => Err(GrpcIngressError::Unimplemented(format!(
                    "unknown method {other}"
                ))),
            }
        })
    }

    fn handle_stream(
        &self,
        method: String,
        _metadata: GrpcMetadata,
        messages: GrpcMessageStream,
        _ctx: RequestContext,
    ) -> BoxFuture<'_, GrpcIngressResult<(GrpcMessageStream, GrpcMetadata)>> {
        Box::pin(async move {
            if method.as_str() != HEALTH_WATCH_METHOD {
                let mut messages = messages;
                let body = match messages.next().await {
                    Some(Ok(b)) => b,
                    Some(Err(e)) => return Err(e),
                    None => vec![],
                };
                let req = GrpcRequest::new(method, body, Duration::from_secs(30));
                let resp = self
                    .handle_unary(req, RequestContext::unauthenticated())
                    .await?;
                let out: GrpcMessageStream =
                    Box::pin(futures::stream::once(futures::future::ready(Ok(resp.body))));
                return Ok((out, resp.metadata));
            }

            let mut messages = messages;
            let body = match messages.next().await {
                Some(Ok(b)) => b,
                Some(Err(e)) => return Err(e),
                None => vec![],
            };
            let service = HealthCheckCodec::decode_request(&body).unwrap_or_default();
            let initial = self
                .get_status(&service)
                .unwrap_or(ServingStatus::ServiceUnknown);
            let rx = self.subscribe();
            let target = service.clone();

            enum HealthServiceWatchPhase {
                Initial,
                Streaming,
                Closed,
            }

            struct HealthServiceWatchState {
                phase: HealthServiceWatchPhase,
                rx: tokio::sync::broadcast::Receiver<(String, ServingStatus)>,
                target: String,
                snapshot: ServingStatus,
            }

            let state = HealthServiceWatchState {
                phase: HealthServiceWatchPhase::Initial,
                rx,
                target,
                snapshot: initial,
            };
            let stream = futures::stream::unfold(state, |mut s| async move {
                match s.phase {
                    HealthServiceWatchPhase::Initial => {
                        let frame = HealthCheckCodec::encode_response(s.snapshot);
                        s.phase = HealthServiceWatchPhase::Streaming;
                        Some((Ok(frame), s))
                    }
                    HealthServiceWatchPhase::Streaming => loop {
                        match s.rx.recv().await {
                            Ok((svc, status)) => {
                                if svc == s.target {
                                    return Some((
                                        Ok(HealthCheckCodec::encode_response(status)),
                                        s,
                                    ));
                                }
                            }
                            Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => {
                                s.phase = HealthServiceWatchPhase::Closed;
                                return Some((
                                    Err(GrpcIngressError::Status(
                                        GrpcStatusCode::ResourceExhausted,
                                        "watch subscriber fell behind".into(),
                                    )),
                                    s,
                                ));
                            }
                            Err(tokio::sync::broadcast::error::RecvError::Closed) => return None,
                        }
                    },
                    HealthServiceWatchPhase::Closed => None,
                }
            });

            Ok((
                Box::pin(stream) as GrpcMessageStream,
                GrpcMetadata::default(),
            ))
        })
    }

    fn health_check(&self) -> BoxFuture<'_, GrpcIngressResult<GrpcHealthCheck>> {
        Box::pin(async move { Ok(GrpcHealthCheck::healthy()) })
    }
}

impl HealthService {
    fn handle_check(&self, body: &[u8]) -> GrpcIngressResult<GrpcResponse> {
        let service = HealthCheckCodec::decode_request(body).unwrap_or_default();
        match self.get_status(&service) {
            Some(status) => Ok(GrpcResponse {
                body: HealthCheckCodec::encode_response(status),
                metadata: GrpcMetadata::default(),
            }),
            None => Err(GrpcIngressError::NotFound(format!(
                "service {service:?} not registered"
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use futures::future::BoxFuture;
    use std::sync::Arc;
    use std::time::Duration;

    use edge_domain::RequestContext;

    use super::HealthCheckCodec;
    use crate::api::health_service::{
        HealthAggregate, HealthService, ServingStatus, HEALTH_CHECK_METHOD,
    };
    use crate::api::port::grpc_ingress::{GrpcHealthCheck, GrpcIngress, GrpcIngressResult};
    use crate::api::value::{GrpcMetadata, GrpcRequest, GrpcResponse};

    /// @covers: HealthCheckCodec::decode_request
    #[test]
    fn test_decode_request_empty_body_returns_empty_service_name() {
        assert_eq!(HealthCheckCodec::decode_request(&[]), Some(String::new()));
    }

    /// @covers: HealthCheckCodec::decode_request
    #[test]
    fn test_decode_request_with_varint_length_field_decodes_service_name() {
        let mut out = Vec::new();
        let b = "pkg.A".as_bytes();
        out.push(0x0a);
        let mut enc_out = Vec::new();
        {
            let mut value = b.len() as u64;
            while value >= 0x80 {
                enc_out.push((value as u8) | 0x80);
                value >>= 7;
            }
            enc_out.push(value as u8);
        }
        out.extend_from_slice(&enc_out);
        out.extend_from_slice(b);
        assert_eq!(
            HealthCheckCodec::decode_request(&out),
            Some("pkg.A".to_string())
        );
    }

    /// @covers: HealthCheckCodec::encode_response
    #[test]
    fn test_encode_response_serving_status_produces_field_tag_and_varint() {
        assert_eq!(
            HealthCheckCodec::encode_response(ServingStatus::Serving),
            vec![0x08, 0x01]
        );
    }

    #[tokio::test]
    async fn test_check_returns_serving_for_registered_service() {
        let svc = Arc::new(HealthService::new());
        svc.set_status("pkg.A", ServingStatus::Serving);
        let body = {
            let b = "pkg.A".as_bytes();
            let mut out = Vec::with_capacity(2 + b.len());
            out.push(0x0a);
            let mut value = b.len() as u64;
            while value >= 0x80 {
                out.push((value as u8) | 0x80);
                value >>= 7;
            }
            out.push(value as u8);
            out.extend_from_slice(b);
            out
        };
        let req = GrpcRequest::new(HEALTH_CHECK_METHOD, body, Duration::from_secs(1));
        let resp = svc
            .handle_unary(req, RequestContext::unauthenticated())
            .await
            .expect("Check must succeed");
        assert_eq!(
            resp.body,
            HealthCheckCodec::encode_response(ServingStatus::Serving)
        );
    }

    #[tokio::test]
    async fn test_health_aggregate_refresh_pushes_dispatcher_health_to_overall() {
        // Local stub — named with the primary type prefix to satisfy SEA cohesion check.
        struct HealthServiceTestDispatcher;
        impl GrpcIngress for HealthServiceTestDispatcher {
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
        let svc = Arc::new(HealthService::new());
        let agg = HealthAggregate::new(svc.clone(), Arc::new(HealthServiceTestDispatcher));
        svc.set_overall_status(ServingStatus::NotServing);
        agg.refresh().await;
        assert_eq!(svc.get_status(""), Some(ServingStatus::Serving));
    }
}
