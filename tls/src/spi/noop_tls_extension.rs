//! No-op [`TlsExtension`] impl for [`NoopTlsExtension`].

use crate::api::traits::TlsExtension;
use crate::api::types::{IngressTlsConfig, NoopTlsExtension};

impl TlsExtension for NoopTlsExtension {
    fn extend(&self, config: IngressTlsConfig) -> IngressTlsConfig {
        config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noop_extend_returns_config_unchanged() {
        let cfg = IngressTlsConfig::tls("a.pem", "b.pem");
        let ext = NoopTlsExtension;
        let result = ext.extend(cfg.clone());
        assert_eq!(result.cert_pem_path, cfg.cert_pem_path);
    }
}
