//! SAF layer — TLS public facade.

mod tls_svc;

pub use crate::api::error::IngressTlsError;
pub use crate::api::types::{IngressTlsConfig, NoopTlsExtension, TlsSvc};
pub use tokio_rustls::TlsAcceptor;
