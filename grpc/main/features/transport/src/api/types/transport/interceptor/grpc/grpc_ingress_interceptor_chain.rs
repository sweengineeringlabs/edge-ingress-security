//! Registered chain of GrpcIngressInterceptors.

use std::sync::Arc;

use crate::api::port::grpc::GrpcIngressError;
use crate::api::value::{GrpcRequest, GrpcResponse};

use super::grpc_ingress_interceptor::GrpcIngressInterceptor;

/// A registered chain of [`GrpcIngressInterceptor`]s.
#[derive(Clone, Default)]
pub struct GrpcIngressInterceptorChain {
    interceptors: Vec<Arc<dyn GrpcIngressInterceptor>>,
}

impl GrpcIngressInterceptorChain {
    /// Construct an empty chain.
    pub fn new() -> Self {
        Self {
            interceptors: Vec::new(),
        }
    }

    /// Register `interceptor` at the end of the chain.
    pub fn push(mut self, interceptor: Arc<dyn GrpcIngressInterceptor>) -> Self {
        self.interceptors.push(interceptor);
        self
    }

    /// Number of registered interceptors.
    pub fn len(&self) -> usize {
        self.interceptors.len()
    }

    /// `true` when no interceptors are registered.
    pub fn is_empty(&self) -> bool {
        self.interceptors.is_empty()
    }

    /// `true` when at least one registered interceptor declares itself
    /// an [`AuthorizationInterceptor`](super::authorization_interceptor::AuthorizationInterceptor) via
    /// [`GrpcIngressInterceptor::is_authorization`].
    pub fn contains_authorization(&self) -> bool {
        self.interceptors.iter().any(|i| i.is_authorization())
    }

    /// Run every `before_dispatch` in order until one fails or all succeed.
    pub fn run_before(&self, req: &mut GrpcRequest) -> Result<(), GrpcIngressError> {
        for i in &self.interceptors {
            i.before_dispatch(req)?;
        }
        Ok(())
    }

    /// Run every `after_dispatch` in order until one fails or all succeed.
    pub fn run_after(&self, resp: &mut GrpcResponse) -> Result<(), GrpcIngressError> {
        for i in &self.interceptors {
            i.after_dispatch(resp)?;
        }
        Ok(())
    }
}
