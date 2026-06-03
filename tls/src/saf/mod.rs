//! SAF layer — TLS public facade.

mod tls_svc;

pub use crate::api::acceptor::AcceptorBuilder;
pub use crate::api::error::IngressTlsError;
pub use crate::api::types::{ApplicationConfigBuilder, IngressTlsConfig, NoopTlsExtension, TlsSvc};
pub use tokio_rustls::TlsAcceptor;
