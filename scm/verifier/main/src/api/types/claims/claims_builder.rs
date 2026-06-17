//! `ClaimsBuilder` — fluent builder for [`Claims`].

use serde_json::Value;
use std::collections::HashMap;

use super::Claims;

/// Fluent builder for [`Claims`].
#[derive(Default)]
pub struct ClaimsBuilder {
    sub: Option<String>,
    iss: Option<String>,
    aud: Option<Value>,
    exp: Option<u64>,
    nbf: Option<u64>,
    iat: Option<u64>,
    jti: Option<String>,
    custom: HashMap<String, Value>,
}

impl ClaimsBuilder {
    /// Set the `sub` claim.
    #[allow(clippy::should_implement_trait)]
    pub fn sub(mut self, v: impl Into<String>) -> Self {
        self.sub = Some(v.into());
        self
    }
    /// Set the `iss` claim.
    pub fn iss(mut self, v: impl Into<String>) -> Self {
        self.iss = Some(v.into());
        self
    }
    /// Set the `exp` claim.
    pub fn exp(mut self, v: u64) -> Self {
        self.exp = Some(v);
        self
    }
    /// Set the `nbf` claim.
    pub fn nbf(mut self, v: u64) -> Self {
        self.nbf = Some(v);
        self
    }
    /// Set the `iat` claim.
    pub fn iat(mut self, v: u64) -> Self {
        self.iat = Some(v);
        self
    }
    /// Set the `jti` claim.
    pub fn jti(mut self, v: impl Into<String>) -> Self {
        self.jti = Some(v.into());
        self
    }
    /// Insert a custom claim.
    pub fn custom(mut self, k: impl Into<String>, v: Value) -> Self {
        self.custom.insert(k.into(), v);
        self
    }
    /// Build the [`Claims`].
    pub fn build(self) -> Claims {
        Claims {
            sub: self.sub,
            iss: self.iss,
            aud: self.aud,
            exp: self.exp,
            nbf: self.nbf,
            iat: self.iat,
            jti: self.jti,
            custom: self.custom,
        }
    }
}
