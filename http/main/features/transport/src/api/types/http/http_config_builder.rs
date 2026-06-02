//! Fluent builder for [`HttpConfig`].

use std::collections::HashMap;

use super::http_config::HttpConfig;

/// Fluent builder that constructs an [`HttpConfig`].
pub struct HttpConfigBuilder {
    base_url: Option<String>,
    timeout_secs: u64,
    connect_timeout_secs: u64,
    max_retries: u32,
    default_headers: HashMap<String, String>,
    follow_redirects: bool,
    max_redirects: u32,
    user_agent: Option<String>,
    max_response_bytes: Option<usize>,
}

impl HttpConfigBuilder {
    /// Start a builder with the same defaults as `HttpConfig::default()`.
    pub fn new() -> Self {
        let defaults = HttpConfig::default();
        Self {
            base_url: defaults.base_url,
            timeout_secs: defaults.timeout_secs,
            connect_timeout_secs: defaults.connect_timeout_secs,
            max_retries: defaults.max_retries,
            default_headers: defaults.default_headers,
            follow_redirects: defaults.follow_redirects,
            max_redirects: defaults.max_redirects,
            user_agent: defaults.user_agent,
            max_response_bytes: defaults.max_response_bytes,
        }
    }

    /// Set the base URL.
    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Set the request timeout in seconds.
    pub fn with_timeout_secs(mut self, secs: u64) -> Self {
        self.timeout_secs = secs;
        self
    }

    /// Set the connect timeout in seconds.
    pub fn with_connect_timeout_secs(mut self, secs: u64) -> Self {
        self.connect_timeout_secs = secs;
        self
    }

    /// Set the maximum number of retry attempts.
    pub fn with_max_retries(mut self, retries: u32) -> Self {
        self.max_retries = retries;
        self
    }

    /// Add a default header.
    pub fn with_header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.default_headers.insert(name.into(), value.into());
        self
    }

    /// Consume the builder and return an [`HttpConfig`].
    pub fn build(self) -> HttpConfig {
        HttpConfig {
            base_url: self.base_url,
            timeout_secs: self.timeout_secs,
            connect_timeout_secs: self.connect_timeout_secs,
            max_retries: self.max_retries,
            default_headers: self.default_headers,
            follow_redirects: self.follow_redirects,
            max_redirects: self.max_redirects,
            user_agent: self.user_agent,
            max_response_bytes: self.max_response_bytes,
        }
    }
}

impl Default for HttpConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}
