//! `RustlsAcceptorBuilder` — builds a [`tokio_rustls::TlsAcceptor`] from [`IngressTlsConfig`].

use std::fs;
use std::sync::Arc;

use rustls::pki_types::{pem::PemObject, CertificateDer, PrivateKeyDer};
use rustls::ServerConfig;

use crate::api::error::IngressTlsError;
use crate::api::traits::AcceptorBuilder;
use crate::api::types::IngressTlsConfig;

/// Builds a [`tokio_rustls::TlsAcceptor`] from a validated [`IngressTlsConfig`],
/// backed by the rustls `ring` CryptoProvider.
pub(crate) struct RustlsAcceptorBuilder;

impl AcceptorBuilder for RustlsAcceptorBuilder {
    fn build_acceptor(
        &self,
        config: &IngressTlsConfig,
    ) -> Result<tokio_rustls::TlsAcceptor, IngressTlsError> {
        Self::build(config)
    }
}

impl RustlsAcceptorBuilder {
    /// Construct a [`tokio_rustls::TlsAcceptor`] from `config`.
    pub(crate) fn build(
        config: &IngressTlsConfig,
    ) -> Result<tokio_rustls::TlsAcceptor, IngressTlsError> {
        let server_cfg = Self::build_server_config(config)?;
        let acceptor = tokio_rustls::TlsAcceptor::from(server_cfg);
        Ok(acceptor)
    }

    fn build_server_config(
        config: &IngressTlsConfig,
    ) -> Result<Arc<ServerConfig>, IngressTlsError> {
        let cert_chain = Self::load_certs(&config.cert_pem_path)?;
        let key = Self::load_key(&config.key_pem_path)?;
        let provider = Arc::new(rustls::crypto::ring::default_provider());
        let builder = ServerConfig::builder_with_provider(provider.clone())
            .with_safe_default_protocol_versions()
            .map_err(|e| IngressTlsError::Config(e.to_string()))?;
        let mut cfg = if let Some(ca_path) = &config.client_ca_pem_path {
            let roots = Self::load_client_ca(ca_path)?;
            let verifier = rustls::server::WebPkiClientVerifier::builder_with_provider(
                Arc::new(roots),
                provider,
            )
            .build()
            .map_err(|e| IngressTlsError::Config(e.to_string()))?;
            builder
                .with_client_cert_verifier(verifier)
                .with_single_cert(cert_chain, key)
                .map_err(|e| IngressTlsError::Config(e.to_string()))?
        } else {
            builder
                .with_no_client_auth()
                .with_single_cert(cert_chain, key)
                .map_err(|e| IngressTlsError::Config(e.to_string()))?
        };
        cfg.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];
        Ok(Arc::new(cfg))
    }

    fn load_certs(path: &str) -> Result<Vec<CertificateDer<'static>>, IngressTlsError> {
        let pem = fs::read(path).map_err(|e| IngressTlsError::CertLoad(path.to_string(), e))?;
        let chain: Result<Vec<_>, _> = CertificateDer::pem_slice_iter(&pem).collect();
        let chain = chain.map_err(|e| IngressTlsError::CertParse(e.to_string()))?;
        if chain.is_empty() {
            return Err(IngressTlsError::CertParse(format!(
                "no certificates found in {path}"
            )));
        }
        Ok(chain)
    }

    fn load_key(path: &str) -> Result<PrivateKeyDer<'static>, IngressTlsError> {
        let pem = fs::read(path).map_err(|e| IngressTlsError::KeyLoad(path.to_string(), e))?;
        PrivateKeyDer::from_pem_slice(&pem).map_err(|e| IngressTlsError::KeyParse(e.to_string()))
    }

    fn load_client_ca(path: &str) -> Result<rustls::RootCertStore, IngressTlsError> {
        let pem = fs::read(path).map_err(|e| IngressTlsError::CertLoad(path.to_string(), e))?;
        let mut store = rustls::RootCertStore::empty();
        for cert in CertificateDer::pem_slice_iter(&pem) {
            let cert = cert.map_err(|e| IngressTlsError::CertParse(e.to_string()))?;
            store
                .add(cert)
                .map_err(|e| IngressTlsError::CertParse(e.to_string()))?;
        }
        if store.is_empty() {
            return Err(IngressTlsError::CertParse(format!(
                "no CA certificates found in {path}"
            )));
        }
        Ok(store)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write as _;

    fn self_signed_pem() -> (String, String) {
        let cert =
            rcgen::generate_simple_self_signed(vec!["localhost".to_string()]).expect("rcgen");
        (cert.cert.pem(), cert.key_pair.serialize_pem())
    }

    fn write_temp(content: &str) -> tempfile::NamedTempFile {
        let mut f = tempfile::NamedTempFile::new().expect("tempfile");
        f.write_all(content.as_bytes()).expect("write");
        f
    }

    #[test]
    fn test_build_returns_acceptor_for_valid_tls_config() {
        let (cert_pem, key_pem) = self_signed_pem();
        let cert_f = write_temp(&cert_pem);
        let key_f = write_temp(&key_pem);
        let cfg = IngressTlsConfig::tls(
            cert_f.path().to_str().expect("cert path"),
            key_f.path().to_str().expect("key path"),
        );
        assert!(RustlsAcceptorBuilder::build(&cfg).is_ok());
    }

    #[test]
    fn test_build_returns_cert_load_error_for_missing_file() {
        let cfg = IngressTlsConfig::tls("/no/cert.pem", "/no/key.pem");
        assert!(matches!(
            RustlsAcceptorBuilder::build(&cfg),
            Err(IngressTlsError::CertLoad(_, _))
        ));
    }

    #[test]
    fn test_build_acceptor_trait_dispatches_to_rustls_for_valid_config() {
        let (cert_pem, key_pem) = self_signed_pem();
        let cert_f = write_temp(&cert_pem);
        let key_f = write_temp(&key_pem);
        let cfg = IngressTlsConfig::tls(
            cert_f.path().to_str().expect("cert path"),
            key_f.path().to_str().expect("key path"),
        );
        let builder: &dyn AcceptorBuilder = &RustlsAcceptorBuilder;
        assert!(builder.build_acceptor(&cfg).is_ok());
    }
}
