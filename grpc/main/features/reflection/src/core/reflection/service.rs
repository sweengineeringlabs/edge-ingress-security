//! `ReflectionService` trait implementations and private helpers.

use std::collections::BTreeSet;
use std::time::Duration;

use futures::future::BoxFuture;
use futures::StreamExt;

use edge_domain::RequestContext;
use swe_edge_ingress_grpc::{
    GrpcHealthCheck, GrpcIngress, GrpcIngressError, GrpcIngressResult, GrpcMessageStream,
    GrpcMetadata, GrpcRequest, GrpcResponse,
};

use crate::api::reflection::reflection_service::{
    ReflectionService, ERROR_CODE_INVALID_ARGUMENT, ERROR_CODE_NOT_FOUND, ERROR_CODE_UNIMPLEMENTED,
    REFLECTION_INFO_METHOD, REFLECTION_SERVICE_NAME,
};
use crate::api::reflection::reflection_request::ReflectionRequest;
use crate::api::reflection::reflection_response::ReflectionResponse;
use crate::api::traits::Processor;
use crate::api::wire::{decode_request, encode_response};

#[allow(dead_code)]
const REFLECTION_DEFAULT_DEADLINE: Duration = Duration::from_secs(5);

impl ReflectionService {
    pub(crate) fn list_services(&self) -> Vec<String> {
        let mut set: BTreeSet<String> = BTreeSet::new();
        for id in self.registry.list_ids() {
            if let Some(name) = crate::api::reflection::reflection_service::service_name_from_method_path(&id) {
                set.insert(name.to_string());
            }
        }
        set.insert(REFLECTION_SERVICE_NAME.to_string());
        set.into_iter().collect()
    }

    pub(crate) fn handle_request(&self, request: ReflectionRequest) -> ReflectionResponse {
        match request {
            ReflectionRequest::ListServices(_) => {
                ReflectionResponse::ListServices(self.list_services())
            }
            ReflectionRequest::FileByFilename(name) => self.descriptor_response_by_filename(&name),
            ReflectionRequest::FileContainingSymbol(symbol) => {
                self.descriptor_response_by_symbol(&symbol)
            }
            ReflectionRequest::FileContainingExtension { .. } => ReflectionResponse::Error {
                error_code: ERROR_CODE_UNIMPLEMENTED,
                error_message: "FileContainingExtension is not implemented".into(),
            },
            ReflectionRequest::AllExtensionNumbersOfType(_) => ReflectionResponse::Error {
                error_code: ERROR_CODE_UNIMPLEMENTED,
                error_message: "AllExtensionNumbersOfType is not implemented".into(),
            },
            ReflectionRequest::Unknown => ReflectionResponse::Error {
                error_code: ERROR_CODE_INVALID_ARGUMENT,
                error_message: "ServerReflectionRequest had no recognised oneof field".into(),
            },
        }
    }

    fn descriptor_response_by_filename(&self, name: &str) -> ReflectionResponse {
        let guard = self.descriptors.read();
        for d in guard.iter() {
            if d.filename == name {
                return ReflectionResponse::FileDescriptor(vec![d.bytes.clone()]);
            }
        }
        ReflectionResponse::Error {
            error_code: ERROR_CODE_NOT_FOUND,
            error_message: format!("no descriptor registered for filename {name:?}"),
        }
    }

    fn descriptor_response_by_symbol(&self, symbol: &str) -> ReflectionResponse {
        let guard = self.descriptors.read();
        for d in guard.iter() {
            if d.symbols.iter().any(|s| s == symbol) {
                return ReflectionResponse::FileDescriptor(vec![d.bytes.clone()]);
            }
        }
        ReflectionResponse::Error {
            error_code: ERROR_CODE_NOT_FOUND,
            error_message: format!("no descriptor registered for symbol {symbol:?}"),
        }
    }
}

impl GrpcIngress for ReflectionService {
    fn handle_unary(
        &self,
        request: GrpcRequest,
        _ctx: RequestContext,
    ) -> BoxFuture<'_, GrpcIngressResult<GrpcResponse>> {
        Box::pin(async move {
            if request.method.as_str() != REFLECTION_INFO_METHOD {
                return Err(GrpcIngressError::Unimplemented(format!(
                    "unknown method {}",
                    request.method
                )));
            }
            let parsed = match decode_request(&request.body) {
                Ok(p) => p,
                Err(e) => {
                    return Err(GrpcIngressError::InvalidArgument(format!(
                        "malformed ServerReflectionRequest: {e}"
                    )))
                }
            };
            let response = self.handle_request(parsed);
            let bytes = encode_response(&response, &request.body);
            Ok(GrpcResponse {
                body: bytes,
                metadata: GrpcMetadata::default(),
            })
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
            if method.as_str() != REFLECTION_INFO_METHOD {
                return Err(GrpcIngressError::Unimplemented(format!(
                    "unknown method {method}"
                )));
            }
            let mut messages = messages;
            let mut requests: Vec<Vec<u8>> = Vec::new();
            while let Some(item) = messages.next().await {
                match item {
                    Ok(b) => requests.push(b),
                    Err(e) => return Err(e),
                }
            }
            if requests.is_empty() {
                let resp = ReflectionResponse::Error {
                    error_code: ERROR_CODE_INVALID_ARGUMENT,
                    error_message: "empty ServerReflectionInfo stream".into(),
                };
                let bytes = encode_response(&resp, &[]);
                let out: GrpcMessageStream =
                    Box::pin(futures::stream::once(futures::future::ready(Ok(bytes))));
                return Ok((out, GrpcMetadata::default()));
            }
            let mut responses: Vec<Vec<u8>> = Vec::with_capacity(requests.len());
            for body in requests.into_iter() {
                let parsed = match decode_request(&body) {
                    Ok(p) => p,
                    Err(e) => {
                        let resp = ReflectionResponse::Error {
                            error_code: ERROR_CODE_INVALID_ARGUMENT,
                            error_message: format!("malformed request frame: {e}"),
                        };
                        responses.push(encode_response(&resp, &body));
                        continue;
                    }
                };
                let response = self.handle_request(parsed);
                responses.push(encode_response(&response, &body));
            }
            let out: GrpcMessageStream = Box::pin(futures::stream::iter(
                responses.into_iter().map(Ok::<Vec<u8>, GrpcIngressError>),
            ));
            Ok((out, GrpcMetadata::default()))
        })
    }

    fn health_check(&self) -> BoxFuture<'_, GrpcIngressResult<GrpcHealthCheck>> {
        Box::pin(async move { Ok(GrpcHealthCheck::healthy()) })
    }
}

impl Processor for ReflectionService {
    fn process<'a>(&'a self, request: ReflectionRequest) -> BoxFuture<'a, ReflectionResponse> {
        Box::pin(async move { self.handle_request(request) })
    }
}

#[cfg(test)]
mod tests {
    use crate::api::reflection::reflection_service::{service_name_from_method_path, ReflectionService};
    use edge_domain::HandlerRegistry;
    use std::sync::Arc;

    /// @covers: list_services
    #[test]
    fn test_list_services_includes_reflection_service_when_registry_empty() {
        let svc = ReflectionService::new(Arc::new(HandlerRegistry::new()));
        let services = svc.list_services();
        assert!(services.iter().any(|s| s.contains("ServerReflection")));
    }

    /// @covers: list_services
    #[test]
    fn test_service_name_from_method_path_extracts_service_portion() {
        assert_eq!(
            service_name_from_method_path("/pkg.Svc/Method"),
            Some("pkg.Svc")
        );
    }

    /// @covers: list_services
    #[test]
    fn test_service_name_from_method_path_returns_none_for_malformed_input() {
        assert!(service_name_from_method_path("no-leading-slash").is_none());
        assert!(service_name_from_method_path("/").is_none());
        assert!(service_name_from_method_path("").is_none());
    }

    /// @covers: handle_request
    #[test]
    fn test_handle_request_list_services_returns_list_services_response() {
        use crate::api::reflection::reflection_request::ReflectionRequest;
        use crate::api::reflection::reflection_response::ReflectionResponse;
        let svc = ReflectionService::new(Arc::new(HandlerRegistry::new()));
        let resp = svc.handle_request(ReflectionRequest::ListServices(String::new()));
        assert!(matches!(resp, ReflectionResponse::ListServices(_)));
    }

    /// @covers: handle_request
    #[test]
    fn test_handle_request_unknown_returns_invalid_argument_error() {
        use crate::api::reflection::reflection_request::ReflectionRequest;
        use crate::api::reflection::reflection_response::ReflectionResponse;
        let svc = ReflectionService::new(Arc::new(HandlerRegistry::new()));
        let resp = svc.handle_request(ReflectionRequest::Unknown);
        match resp {
            ReflectionResponse::Error { error_code, .. } => {
                assert_eq!(
                    error_code,
                    crate::api::reflection::reflection_service::ERROR_CODE_INVALID_ARGUMENT
                );
            }
            other => panic!("expected Error, got {other:?}"),
        }
    }
}
