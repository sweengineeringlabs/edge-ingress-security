//! Registered chain of GrpcIngressInterceptors.

use std::sync::Arc;

use crate::api::port::grpc_ingress::GrpcIngressError;
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

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::time::Duration;

    use crate::api::port::grpc_ingress::GrpcIngressError;
    use crate::api::value::{GrpcMetadata, GrpcRequest, GrpcResponse, GrpcStatusCode};

    use super::*;

    fn req() -> GrpcRequest {
        GrpcRequest::new("svc/M", vec![], Duration::from_secs(1))
    }

    #[test]
    fn test_new_chain_is_empty() {
        let chain = GrpcIngressInterceptorChain::new();
        assert_eq!(chain.len(), 0);
        assert!(chain.is_empty());
    }

    /// @covers: push
    #[test]
    fn test_push_adds_interceptor_to_chain() {
        struct Noop;
        impl GrpcIngressInterceptor for Noop {
            fn before_dispatch(&self, _: &mut GrpcRequest) -> Result<(), GrpcIngressError> {
                Ok(())
            }
            fn after_dispatch(&self, _: &mut GrpcResponse) -> Result<(), GrpcIngressError> {
                Ok(())
            }
        }
        let chain = GrpcIngressInterceptorChain::new().push(Arc::new(Noop));
        assert_eq!(chain.len(), 1);
        assert!(!chain.is_empty());
    }

    /// @covers: len
    #[test]
    fn test_len_returns_interceptor_count() {
        struct Noop;
        impl GrpcIngressInterceptor for Noop {
            fn before_dispatch(&self, _: &mut GrpcRequest) -> Result<(), GrpcIngressError> {
                Ok(())
            }
            fn after_dispatch(&self, _: &mut GrpcResponse) -> Result<(), GrpcIngressError> {
                Ok(())
            }
        }
        let chain = GrpcIngressInterceptorChain::new()
            .push(Arc::new(Noop))
            .push(Arc::new(Noop));
        assert_eq!(chain.len(), 2);
    }

    /// @covers: is_empty
    #[test]
    fn test_is_empty_true_on_empty_chain() {
        assert!(GrpcIngressInterceptorChain::new().is_empty());
    }

    /// @covers: contains_authorization
    #[test]
    fn test_contains_authorization_returns_false_for_empty_chain() {
        assert!(!GrpcIngressInterceptorChain::new().contains_authorization());
    }

    #[test]
    fn test_contains_authorization_returns_false_when_chain_has_no_authz_gates() {
        struct Plain;
        impl GrpcIngressInterceptor for Plain {
            fn before_dispatch(&self, _: &mut GrpcRequest) -> Result<(), GrpcIngressError> {
                Ok(())
            }
            fn after_dispatch(&self, _: &mut GrpcResponse) -> Result<(), GrpcIngressError> {
                Ok(())
            }
        }
        let chain = GrpcIngressInterceptorChain::new()
            .push(Arc::new(Plain))
            .push(Arc::new(Plain));
        assert!(!chain.contains_authorization());
    }

    #[test]
    fn test_contains_authorization_returns_true_when_at_least_one_authz_gate_is_present() {
        use crate::api::interceptor::authorization_interceptor::AuthorizationInterceptor;
        struct Plain;
        impl GrpcIngressInterceptor for Plain {
            fn before_dispatch(&self, _: &mut GrpcRequest) -> Result<(), GrpcIngressError> {
                Ok(())
            }
            fn after_dispatch(&self, _: &mut GrpcResponse) -> Result<(), GrpcIngressError> {
                Ok(())
            }
        }
        struct Authz;
        impl GrpcIngressInterceptor for Authz {
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
        impl AuthorizationInterceptor for Authz {}
        let chain = GrpcIngressInterceptorChain::new()
            .push(Arc::new(Plain))
            .push(Arc::new(Authz));
        assert!(chain.contains_authorization());
    }

    /// @covers: run_before
    #[test]
    fn test_run_before_short_circuits_on_first_failure() {
        struct Deny;
        impl GrpcIngressInterceptor for Deny {
            fn before_dispatch(&self, _: &mut GrpcRequest) -> Result<(), GrpcIngressError> {
                Err(GrpcIngressError::Status(
                    GrpcStatusCode::Unauthenticated,
                    "no creds".into(),
                ))
            }
            fn after_dispatch(&self, _: &mut GrpcResponse) -> Result<(), GrpcIngressError> {
                Ok(())
            }
        }
        struct CountAfter(Arc<AtomicUsize>);
        impl GrpcIngressInterceptor for CountAfter {
            fn before_dispatch(&self, _: &mut GrpcRequest) -> Result<(), GrpcIngressError> {
                self.0.fetch_add(1, Ordering::SeqCst);
                Ok(())
            }
            fn after_dispatch(&self, _: &mut GrpcResponse) -> Result<(), GrpcIngressError> {
                Ok(())
            }
        }
        let counter = Arc::new(AtomicUsize::new(0));
        let chain = GrpcIngressInterceptorChain::new()
            .push(Arc::new(Deny))
            .push(Arc::new(CountAfter(counter.clone())));
        let mut r = req();
        let err = chain.run_before(&mut r).expect_err("must error");
        match err {
            GrpcIngressError::Status(GrpcStatusCode::Unauthenticated, _) => {}
            other => panic!("expected Unauthenticated, got {other:?}"),
        }
        assert_eq!(counter.load(Ordering::SeqCst), 0);
    }

    /// @covers: run_after
    #[test]
    fn test_run_after_invokes_every_after_hook() {
        struct Count(Arc<AtomicUsize>);
        impl GrpcIngressInterceptor for Count {
            fn before_dispatch(&self, _: &mut GrpcRequest) -> Result<(), GrpcIngressError> {
                Ok(())
            }
            fn after_dispatch(&self, _: &mut GrpcResponse) -> Result<(), GrpcIngressError> {
                self.0.fetch_add(1, Ordering::SeqCst);
                Ok(())
            }
        }
        let counter = Arc::new(AtomicUsize::new(0));
        let chain = GrpcIngressInterceptorChain::new()
            .push(Arc::new(Count(counter.clone())))
            .push(Arc::new(Count(counter.clone())));
        let mut resp = GrpcResponse {
            body: vec![],
            metadata: GrpcMetadata::default(),
        };
        chain.run_after(&mut resp).expect("after must pass");
        assert_eq!(counter.load(Ordering::SeqCst), 2);
    }
}
