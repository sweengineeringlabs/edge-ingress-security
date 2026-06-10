//! Verifier SAF — factory methods on [`VerifierSvc`] and [`JwtVerifier`].

use jsonwebtoken::Validation;
use swe_edge_configbuilder::ConfigLoaderFactory;

use crate::api::error::VerifierError;
use crate::api::types::JwtConfig;
use crate::api::types::JwtVerifier;
use crate::api::types::VerifierSvc;
use crate::spi::jwt::jsonwebtoken::default_jwt_verifier::DefaultJwtVerifier;

impl VerifierSvc {
    /// Return a config builder pre-seeded with this crate's package name and version.
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        let builder = ConfigLoaderFactory::create_config_builder();
        builder
            .with_name(env!("CARGO_PKG_NAME"))
            .with_version(env!("CARGO_PKG_VERSION"))
    }

    /// Validate any value implementing the [`Validator`](crate::api::traits::validator::Validator)
    /// contract, returning a human-readable error describing the first failure.
    pub fn validate<V: crate::api::traits::validator::Validator>(v: &V) -> Result<(), String> {
        v.validate()
    }
}

impl JwtVerifier {
    /// Construct from [`JwtConfig`].
    ///
    /// Parses key material eagerly — failures surface at startup, not at
    /// request time. Returns [`VerifierError::Config`] if the key PEM is
    /// malformed or the algorithm is unsupported.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use swe_edge_ingress_verifier::{JwtConfig, JwtKey, JwtVerifier};
    ///
    /// let cfg = JwtConfig {
    ///     key: JwtKey::Hs256 { secret: b"at-least-32-bytes-for-hs256-here".to_vec() },
    ///     required_issuer: None,
    ///     required_audience: None,
    ///     leeway_seconds: 0,
    /// };
    /// let verifier = JwtVerifier::from_config(&cfg).expect("valid config");
    /// ```
    pub fn from_config(config: &JwtConfig) -> Result<Self, VerifierError> {
        let (key, algorithm) = DefaultJwtVerifier::build_decoding_key(&config.key)?;

        let mut validation = Validation::new(algorithm);
        validation.leeway = config.leeway_seconds;

        if let Some(ref iss) = config.required_issuer {
            validation.set_issuer(&[iss]);
        }

        if let Some(ref aud) = config.required_audience {
            validation.set_audience(&[aud]);
        } else {
            validation.validate_aud = false;
        }

        Ok(Self { key, validation })
    }
}
