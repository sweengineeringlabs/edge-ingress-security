//! Integration tests for the AcceptorBuilder trait.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_ingress_tls::{AcceptorBuilder, IngressTlsConfig, IngressTlsError, TlsAcceptor};

struct AlwaysFailBuilder;

impl AcceptorBuilder for AlwaysFailBuilder {
    fn build_acceptor(&self, _config: &IngressTlsConfig) -> Result<TlsAcceptor, IngressTlsError> {
        Err(IngressTlsError::Config("AlwaysFailBuilder".into()))
    }
}

/// @covers: AcceptorBuilder — trait is implementable by downstream types
#[test]
fn test_acceptor_builder_custom_impl_returns_error() {
    let builder = AlwaysFailBuilder;
    let cfg = IngressTlsConfig::tls("cert.pem", "key.pem");
    let result = builder.build_acceptor(&cfg);
    assert!(result.is_err());
    assert!(matches!(result.err().unwrap(), IngressTlsError::Config(_)));
}

/// @covers: AcceptorBuilder — trait is object-safe
#[test]
fn test_acceptor_builder_trait_is_object_safe() {
    fn _assert(_: &dyn AcceptorBuilder) {}
    _assert(&AlwaysFailBuilder);
}
