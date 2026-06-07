//! SAF layer — TLS public facade.

mod tls_svc;

pub use crate::api::error::IngressTlsError;
pub use crate::api::types::IngressTlsConfig;
pub use crate::api::types::{ApplicationConfigBuilder, NoopTlsExtension, TlsSvc};
pub use crate::spi::acceptor::rustls::AcceptorBuilder;
pub use tokio_rustls::TlsAcceptor;
