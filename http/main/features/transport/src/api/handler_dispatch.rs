//! HTTP handler dispatch declarations and inherent methods.

use std::sync::Arc;

use edge_domain::{Handler as _, HandlerRegistry};
use parking_lot::RwLock;
use swe_observ_metrics::MetricsProvider;

use crate::api::handler_adapter::HttpHandlerAdapter;
use crate::api::value_object::{HttpRequest, HttpResponse};

/// Error returned when a handler registration fails.
#[derive(Debug, thiserror::Error)]
pub enum HttpDispatcherError {
    /// Route pattern could not be registered.
    #[error("failed to register pattern `{pattern}`: {reason}")]
    RegistrationFailed {
        /// The route pattern that failed to register.
        pattern: String,
        /// The reason the registration was rejected.
        reason: String,
    },
}

/// Dispatcher that routes inbound HTTP requests through a
/// [`HandlerRegistry`] keyed by handler id, using `matchit` path-pattern
/// matching.
pub struct HttpHandlerRegistryDispatcher {
    pub(crate) registry: Arc<HandlerRegistry<HttpRequest, HttpResponse>>,
    pub(crate) router:   RwLock<matchit::Router<String>>,
    pub(crate) metrics:  Option<Arc<dyn MetricsProvider>>,
}

impl HttpHandlerRegistryDispatcher {
    /// Construct a dispatcher backed by `registry`.
    pub fn new(registry: Arc<HandlerRegistry<HttpRequest, HttpResponse>>) -> Self {
        Self { registry, router: RwLock::new(matchit::Router::new()), metrics: None }
    }

    /// Attach a metrics provider; per-handler counters and latency histograms
    /// are recorded automatically on every dispatch.
    pub fn with_metrics(mut self, provider: Arc<dyn MetricsProvider>) -> Self {
        self.metrics = Some(provider);
        self
    }

    /// Register a typed adapter.
    pub fn register<Req, Resp>(
        &self,
        adapter: HttpHandlerAdapter<Req, Resp>,
    ) -> Result<(), HttpDispatcherError>
    where
        Req:  Send + 'static,
        Resp: Send + 'static,
    {
        let id      = adapter.id().to_string();
        let pattern = adapter.pattern().to_string();
        self.registry.register(Arc::new(adapter));
        self.router.write().insert(pattern.clone(), id).map_err(|e| {
            HttpDispatcherError::RegistrationFailed { pattern, reason: e.to_string() }
        })
    }

    /// Borrow the inner registry.
    pub fn registry(&self) -> &Arc<HandlerRegistry<HttpRequest, HttpResponse>> {
        &self.registry
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use edge_domain::HandlerRegistry;
    use super::*;

    fn fresh_dispatcher() -> HttpHandlerRegistryDispatcher {
        HttpHandlerRegistryDispatcher::new(Arc::new(HandlerRegistry::new()))
    }

    /// @covers: new — creates empty dispatcher.
    #[test]
    fn test_new_dispatcher_creates_empty_registry() {
        assert!(fresh_dispatcher().registry().is_empty());
    }

    /// @covers: registry — returns same Arc.
    #[test]
    fn test_registry_returns_shared_registry() {
        let d = fresh_dispatcher();
        let r1 = d.registry().clone();
        let r2 = d.registry().clone();
        assert!(Arc::ptr_eq(&r1, &r2));
    }

    /// @covers: HttpDispatcherError::RegistrationFailed — formats with pattern and reason.
    #[test]
    fn test_http_dispatcher_error_formats_with_pattern_and_reason() {
        let e = HttpDispatcherError::RegistrationFailed {
            pattern: "/api/v1".into(),
            reason:  "conflict".into(),
        };
        let msg = e.to_string();
        assert!(msg.contains("/api/v1"), "{msg}");
        assert!(msg.contains("conflict"), "{msg}");
    }
}
