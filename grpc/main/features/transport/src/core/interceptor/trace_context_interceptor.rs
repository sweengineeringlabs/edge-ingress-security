//! W3C Trace Context interceptor implementation.

/// Marker confirming this module provides the W3C Trace Context interceptor implementation.
///
/// The actual type is [`crate::api::interceptor::trace_context_interceptor::TraceContextInterceptor`].
/// This struct exists to satisfy the SEA rule requiring every core module file to define
/// a primary type matching the filename.
pub(crate) struct TraceContextInterceptorImpl;

use crate::api::interceptor::trace_context_interceptor::{
    TraceContextInterceptor, EXTRACTED_TRACEPARENT, EXTRACTED_TRACESTATE, TRACEPARENT, TRACESTATE,
};
use crate::api::interceptor::GrpcIngressInterceptor;
use crate::api::port::grpc_ingress::GrpcIngressError;
use crate::api::value_object::{GrpcRequest, GrpcResponse};

fn is_valid_traceparent(value: &str) -> bool {
    if value.len() != 55 {
        return false;
    }
    let bytes = value.as_bytes();
    if &bytes[0..2] != b"00" || bytes[2] != b'-' {
        return false;
    }
    if bytes[35] != b'-' || bytes[52] != b'-' {
        return false;
    }
    let hex =
        |slice: &[u8]| -> bool { slice.iter().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f')) };
    hex(&bytes[3..35]) && hex(&bytes[36..52]) && hex(&bytes[53..55])
}

impl GrpcIngressInterceptor for TraceContextInterceptor {
    fn before_dispatch(&self, req: &mut GrpcRequest) -> Result<(), GrpcIngressError> {
        if let Some(tp) = req.metadata.headers.get(TRACEPARENT).cloned() {
            if is_valid_traceparent(&tp) {
                req.metadata
                    .headers
                    .insert(EXTRACTED_TRACEPARENT.to_string(), tp);
                if let Some(ts) = req.metadata.headers.get(TRACESTATE).cloned() {
                    req.metadata
                        .headers
                        .insert(EXTRACTED_TRACESTATE.to_string(), ts);
                }
            } else {
                tracing::warn!(value = %tp, "dropping malformed inbound traceparent");
            }
        }
        Ok(())
    }

    fn after_dispatch(&self, _resp: &mut GrpcResponse) -> Result<(), GrpcIngressError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    fn req() -> GrpcRequest {
        GrpcRequest::new("svc/M", vec![], Duration::from_secs(1))
    }

    #[test]
    fn test_extracts_well_formed_traceparent_into_internal_key() {
        let tp = "00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-01";
        let interceptor = TraceContextInterceptor::new();
        let mut r = req();
        r.metadata.headers.insert(TRACEPARENT.into(), tp.into());
        interceptor
            .before_dispatch(&mut r)
            .expect("before_dispatch");
        assert_eq!(
            r.metadata
                .headers
                .get(EXTRACTED_TRACEPARENT)
                .map(String::as_str),
            Some(tp),
        );
    }

    #[test]
    fn test_drops_malformed_traceparent_and_continues() {
        let interceptor = TraceContextInterceptor::new();
        let mut r = req();
        r.metadata
            .headers
            .insert(TRACEPARENT.into(), "garbage".into());
        interceptor
            .before_dispatch(&mut r)
            .expect("before_dispatch");
        assert!(!r.metadata.headers.contains_key(EXTRACTED_TRACEPARENT));
    }
}
