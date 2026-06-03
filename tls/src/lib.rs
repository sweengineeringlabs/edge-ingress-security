//! `swe-edge-ingress-tls` — server-side TLS and mTLS for inbound servers.
//!
//! Provides [`IngressTlsConfig`] (loaded from PEM files) and
//! [`TlsSvc::build_tls_acceptor`] which constructs a [`tokio_rustls::TlsAcceptor`]
//! backed by the `ring` CryptoProvider without requiring a process-wide default.

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod gateway;
mod saf;
mod spi;

pub use gateway::*;
