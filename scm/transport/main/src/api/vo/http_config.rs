//! HTTP client/server configuration.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// HTTP client configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct HttpConfig {
    /// Base URL prepended to all request URLs.
    pub base_url: Option<String>,
    /// Request timeout in seconds.
    pub timeout_secs: u64,
    /// Connection timeout in seconds.
    pub connect_timeout_secs: u64,
    /// Maximum number of retry attempts.
    pub max_retries: u32,
    /// Headers added to every request.
    #[serde(default)]
    pub default_headers: HashMap<String, String>,
    /// Whether to follow HTTP redirects.
    pub follow_redirects: bool,
    /// Maximum number of redirects to follow.
    pub max_redirects: u32,
    /// User-Agent header value.
    pub user_agent: Option<String>,
    /// Maximum response body size to accept.
    #[serde(default = "HttpConfig::default_max_response_bytes")]
    pub max_response_bytes: Option<usize>,
}

impl HttpConfig {
    fn default_max_response_bytes() -> Option<usize> {
        Some(10 * 1024 * 1024)
    }
}

impl Default for HttpConfig {
    fn default() -> Self {
        Self {
            base_url: None,
            timeout_secs: 30,
            connect_timeout_secs: 10,
            max_retries: 3,
            default_headers: HashMap::new(),
            follow_redirects: true,
            max_redirects: 10,
            user_agent: Some("swe-edge/0.1.0".to_string()),
            max_response_bytes: HttpConfig::default_max_response_bytes(),
        }
    }
}

impl HttpConfig {
    /// Create a config with a base URL.
    pub fn with_base_url(base_url: impl Into<String>) -> Self {
        Self {
            base_url: Some(base_url.into()),
            ..Default::default()
        }
    }

    /// Add a default header to the config.
    pub fn with_header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.default_headers.insert(name.into(), value.into());
        self
    }

    /// Set the request timeout.
    pub fn with_timeout(mut self, secs: u64) -> Self {
        self.timeout_secs = secs;
        self
    }
}
